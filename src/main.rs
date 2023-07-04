#![recursion_limit = "10000"]

mod shaders;

use std::collections::HashMap;
use std::io::Write;
use vc4_drm::card::{drm_vc4_submit_rcl_surface, Card};
use vc4_drm::cl::*;
use vc4_drm::drm::control::{connector::State, Device, PageFlipFlags};

use std::io::Cursor;
use std::sync::OnceLock;
use vc4_drm::glam::UVec2;

struct Framebuffer {
    pub bo: vc4_drm::drm::buffer::Handle,
    pub framebuffer: vc4_drm::drm::control::framebuffer::Handle,
}

struct DisplayFramebuffers {
    pub size: (u16, u16),
    crtc: vc4_drm::drm::control::crtc::Handle,
    framebuffers: [Framebuffer; 2],
    connector: vc4_drm::drm::control::connector::Handle,
    mode: vc4_drm::drm::control::Mode,
}

impl DisplayFramebuffers {
    pub fn set_crtc(&self, card: &Card, index: usize) {
        card.set_crtc(
            self.crtc,
            Some(self.framebuffers[index].framebuffer),
            (0, 0),
            &[self.connector],
            Some(self.mode),
        )
        .expect("unable to set_crtc");
    }

    pub async fn page_flip(&self, card: &Card, index: usize) {
        card.page_flip(
            self.crtc,
            self.framebuffers[index].framebuffer,
            PageFlipFlags::EVENT,
            None,
        )
        .expect("unable to page_flip");

        card.wait_for_flip().await;
    }
}

fn open_and_allocate_display_framebuffers(card: &Card) -> DisplayFramebuffers {
    for connector in card
        .resource_handles()
        .expect("Unable to get resource handles")
        .connectors()
    {
        let connector_info = card
            .get_connector(*connector, false)
            .expect("Unable to get_connector");
        if connector_info.state() != State::Connected {
            continue;
        }
        if connector_info.modes().len() == 0 {
            continue;
        }
        let mode = connector_info.modes()[0];
        let current_encoder = connector_info
            .current_encoder()
            .expect("unable to get current encoder");
        let encoder_info = card
            .get_encoder(current_encoder)
            .expect("unable to get encoder info");
        let crtc = encoder_info.crtc().expect("unable to get crtc");

        let create_framebuffer = || {
            let image_buffer = card
                .vc4_create_bgra_image_buffer((mode.size().0 as u32, mode.size().1 as u32))
                .expect("unable to create image buffer");
            let framebuffer = card
                .add_framebuffer(&image_buffer, 32, 32)
                .expect("unable to add framebuffer");
            use vc4_drm::drm::buffer::Buffer;
            Framebuffer {
                bo: image_buffer.handle(),
                framebuffer,
            }
        };

        return DisplayFramebuffers {
            size: mode.size(),
            crtc,
            framebuffers: [create_framebuffer(), create_framebuffer()],
            connector: *connector,
            mode,
        };
    }
    panic!("Couldn't find a display");
}

pub fn get_card() -> &'static Card {
    static CARD: OnceLock<Card> = OnceLock::new();
    CARD.get_or_init(|| Card::open_global())
}

pub struct ShaderAttribute {
    handle: vc4_drm::card::Buffer,
    record: AttributeRecord,
}

pub struct TextureUniform {
    handle: vc4_drm::card::Buffer,
    config: TextureConfigUniform,
}

pub enum ShaderUniform {
    Texture(TextureUniform),
    Constant(u32),
}

#[derive(Default)]
pub struct CommandRecorder {
    bin_cl_buf: Vec<u8>,
    shader_rec_buf: Vec<u8>,
    shader_rec_count: u32,
    uniforms: Vec<u32>,
    bo_handle_map: HashMap<vc4_drm::drm::buffer::Handle, u32>,
    bo_handles: Vec<vc4_drm::drm::buffer::Handle>,
    window_size: (u16, u16),
    width_in_tiles: u8,
    height_in_tiles: u8,

    // State tracking
    line_width: Option<LineWidth>,
    clip_window: Option<ClipWindow>,
    clipper_xy_scaling: Option<ClipperXYScaling>,
    viewport_offset: Option<ViewportOffset>,
    configuration_bits: Option<ConfigurationBits>,
    depth_offset: Option<DepthOffset>,
    clipper_z_scale_and_offset: Option<ClipperZScaleAndOffset>,
    point_size: Option<PointSize>,
    flat_shade_flags: Option<FlatShadeFlags>,
}

macro_rules! command_recorder_setter {
    ($setter:ident, $var:ident, $typename:path) => {
        pub fn $setter(&mut self, $var: $typename) {
            if self.$var.is_none() || self.$var.as_ref().unwrap() != &$var {
                $var.encode(&mut self.bin_cl_buf).unwrap();
                self.$var = Some($var);
            }
        }
    };
}

impl CommandRecorder {
    pub fn new(window_size: (u16, u16)) -> Self {
        let mut obj = Self::default();
        obj.window_size = window_size;
        obj
    }

    pub fn relocate_handle(&mut self, handle: vc4_drm::drm::buffer::Handle) -> u32 {
        if let Some(index) = self.bo_handle_map.get(&handle) {
            *index
        } else {
            let index = self.bo_handles.len() as u32;
            self.bo_handle_map.insert(handle, index);
            self.bo_handles.push(handle);
            index
        }
    }

    pub fn clear(&mut self) {
        self.bin_cl_buf.clear();
        self.shader_rec_buf.clear();
        self.shader_rec_count = 0;
        self.uniforms.clear();
        self.bo_handle_map.clear();
        self.bo_handles.clear();

        self.line_width = None;
        self.clip_window = None;
        self.clipper_xy_scaling = None;
        self.viewport_offset = None;
        self.configuration_bits = None;
        self.depth_offset = None;
        self.clipper_z_scale_and_offset = None;
        self.point_size = None;
        self.flat_shade_flags = None;
    }

    pub fn begin_pass(&mut self) {
        let tile_bin_config = TileBinningModeConfiguration::with_size_in_pixels(
            self.window_size.0,
            self.window_size.1,
        );
        tile_bin_config.encode(&mut self.bin_cl_buf).unwrap();
        self.width_in_tiles = tile_bin_config.width_in_tiles;
        self.height_in_tiles = tile_bin_config.height_in_tiles;

        // START_TILE_BINNING resets the statechange counters in the hardware,
        // which are what is used when a primitive is binned to a tile to
        // figure out what new state packets need to be written to that tile's
        // command list.
        StartTileBinning {}.encode(&mut self.bin_cl_buf).unwrap();

        self.set_line_width(LineWidth { line_width: 0.0 });

        self.set_clip_window(ClipWindow {
            clip_window_left_pixel_coordinate: 0,
            clip_window_bottom_pixel_coordinate: 0,
            clip_window_width_in_pixels: self.window_size.0,
            clip_window_height_in_pixels: self.window_size.1,
        });

        self.set_clipper_xy_scaling(ClipperXYScaling {
            viewport_half_width_in_1_16th_of_pixel: (self.window_size.0 * 16 / 2) as f32,
            viewport_half_height_in_1_16th_of_pixel: (self.window_size.1 * 16 / 2) as f32,
        });

        self.set_viewport_offset(ViewportOffset {
            viewport_centre_x_coordinate_12_4: self.window_size.0 * 16 / 2,
            viewport_centre_y_coordinate_12_4: self.window_size.1 * 16 / 2,
        });

        let depth_test_enable = false;
        let depth_write_enable = false;
        self.set_configuration_bits(ConfigurationBits {
            early_z_updates_enable: true,
            early_z_enable: depth_test_enable,
            z_updates_enable: depth_write_enable && depth_test_enable,
            depth_test_function: if depth_test_enable {
                CompareFunction::LEqual
            } else {
                CompareFunction::Always
            },
            coverage_read_mode: false,
            coverage_pipe_select: false,
            coverage_update_mode: 0,
            coverage_read_type: false,
            antialiased_points_and_lines: false,
            rasteriser_oversample_mode: 0,
            enable_depth_offset: false,
            clockwise_primitives: false,
            enable_reverse_facing_primitive: true,
            enable_forward_facing_primitive: true,
        });

        self.set_depth_offset(DepthOffset {
            depth_offset_factor: 0,
            depth_offset_units: 0,
        });

        self.set_clipper_z_scale_and_offset(ClipperZScaleAndOffset {
            viewport_z_scale_zc_to_zs: 1.0,
            viewport_z_offset_zc_to_zs: 0.0,
        });

        self.set_point_size(PointSize { point_size: 1.0 });

        self.set_flat_shade_flags(FlatShadeFlags {
            flat_shading_flags: 0,
        });
    }

    pub fn end_pass(&mut self) {
        // Increment the semaphore indicating that binning is done and
        // unblocking the render thread.  Note that this doesn't act
        // until the FLUSH completes.
        // The FLUSH caps all of our bin lists with a
        // VC4_PACKET_RETURN.
        IncrementSemaphore {}.encode(&mut self.bin_cl_buf).unwrap();
        Flush {}.encode(&mut self.bin_cl_buf).unwrap();
    }

    fn add_uniform_relocs(&mut self, uniforms: &[ShaderUniform]) {
        for uniform in uniforms {
            if let ShaderUniform::Texture(tex) = uniform {
                let tex_idx = self.relocate_handle(tex.handle.into());
                self.uniforms.push(tex_idx);
            }
        }
    }

    fn add_uniforms(&mut self, uniforms: &[ShaderUniform]) {
        for uniform in uniforms {
            match uniform {
                ShaderUniform::Texture(tex) => {
                    self.uniforms.push(tex.config.get_1d_word());
                    if tex.config.height > 1 {
                        self.uniforms.push(tex.config.get_2d_word());
                    }
                }
                ShaderUniform::Constant(constant) => {
                    self.uniforms.push(*constant);
                }
            }
        }
    }

    pub fn bind_shader(
        &mut self,
        fs_single_threaded: bool,
        fs: vc4_drm::drm::buffer::Handle,
        cs: vc4_drm::drm::buffer::Handle,
        vs: vc4_drm::drm::buffer::Handle,
        attributes: &[ShaderAttribute],
        fs_uniforms: &[ShaderUniform],
        cs_uniforms: &[ShaderUniform],
        vs_uniforms: &[ShaderUniform],
    ) {
        GlShaderState {
            address: 0,
            extended_shader_record: false,
            number_of_attribute_arrays: attributes.len() as _,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();

        let fs_idx = self.relocate_handle(fs);
        let cs_idx = self.relocate_handle(cs);
        let vs_idx = self.relocate_handle(vs);

        self.shader_rec_count += 1;
        self.shader_rec_buf
            .write_all(&fs_idx.to_le_bytes())
            .unwrap();
        self.shader_rec_buf
            .write_all(&cs_idx.to_le_bytes())
            .unwrap();
        self.shader_rec_buf
            .write_all(&vs_idx.to_le_bytes())
            .unwrap();

        let mut attributes_size = 0;
        let mut attributes_bits = 0;
        for i in 0..attributes.len() {
            let buf_idx = self.relocate_handle(attributes[i].handle.into());
            self.shader_rec_buf
                .write_all(&buf_idx.to_le_bytes())
                .unwrap();
            attributes_size += attributes[i].record.number_of_bytes_minus_1 + 1;
            attributes_bits |= 1 << i;
        }

        GlShaderRecord {
            fragment_shader_is_single_threaded: fs_single_threaded,
            point_size_included_in_shaded_vertex_data: false,
            enable_clipping: true,
            fragment_shader_number_of_uniforms_not_used_currently: 0,
            fragment_shader_number_of_varyings: 0,
            fragment_shader_code_address_offset: 0,
            fragment_shader_uniforms_address: 0,
            vertex_shader_number_of_uniforms_not_used_currently: 0,
            vertex_shader_attribute_array_select_bits: attributes_bits,
            vertex_shader_total_attributes_size: attributes_size,
            vertex_shader_code_address_offset: 0,
            vertex_shader_uniforms_address: 0,
            coordinate_shader_number_of_uniforms_not_used_currently: 0,
            coordinate_shader_attribute_array_select_bits: attributes_bits,
            coordinate_shader_total_attributes_size: attributes_size,
            coordinate_shader_code_address_offset: 0,
            coordinate_shader_uniforms_address: 0,
        }
        .encode(&mut self.shader_rec_buf)
        .unwrap();

        for attr in attributes {
            attr.record.encode(&mut self.shader_rec_buf).unwrap();
        }

        self.add_uniform_relocs(fs_uniforms);
        self.add_uniform_relocs(cs_uniforms);
        self.add_uniform_relocs(vs_uniforms);

        self.add_uniforms(fs_uniforms);
        self.add_uniforms(cs_uniforms);
        self.add_uniforms(vs_uniforms);
    }

    pub fn draw_array_primitives(
        &mut self,
        primitive_mode: PrimitiveMode,
        start: u32,
        length: u32,
    ) {
        VertexArrayPrimitives {
            index_of_first_vertex: start,
            length,
            primitive_mode,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();
    }

    command_recorder_setter!(set_line_width, line_width, LineWidth);
    command_recorder_setter!(set_clip_window, clip_window, ClipWindow);
    command_recorder_setter!(set_clipper_xy_scaling, clipper_xy_scaling, ClipperXYScaling);
    command_recorder_setter!(set_viewport_offset, viewport_offset, ViewportOffset);
    command_recorder_setter!(
        set_configuration_bits,
        configuration_bits,
        ConfigurationBits
    );
    command_recorder_setter!(set_depth_offset, depth_offset, DepthOffset);
    command_recorder_setter!(
        set_clipper_z_scale_and_offset,
        clipper_z_scale_and_offset,
        ClipperZScaleAndOffset
    );
    command_recorder_setter!(set_point_size, point_size, PointSize);
    command_recorder_setter!(set_flat_shade_flags, flat_shade_flags, FlatShadeFlags);
}

async fn async_main() {
    let card = get_card();
    shaders::initialize_shaders().await;

    let display_framebuffers = open_and_allocate_display_framebuffers(&card);

    let tex_bo = {
        use vc4_drm::image::*;
        let image_size = UVec2::new(256, 256);
        let (translator, size_in_bytes) = Translator::new_with_alloc_size(image_size, 32);
        let buffer = card.vc4_create_bo(size_in_bytes).unwrap();
        {
            let mut mapping = card.vc4_mmap_bo(&buffer).unwrap();
            for y in 0..image_size.y {
                for x in 0..image_size.x {
                    let offset = translator
                        .coordinate_to_tile_address(UVec2::new(x, y))
                        .offset as usize;
                    mapping.as_mut()[offset] = 0;
                    mapping.as_mut()[offset + 1] = y as u8;
                    mapping.as_mut()[offset + 2] = x as u8;
                    mapping.as_mut()[offset + 3] = 255;
                }
            }
        }
        buffer
    };

    let vbo = card.vc4_create_bo(24).unwrap();

    let mut command_recorder = CommandRecorder::new(display_framebuffers.size);
    command_recorder.begin_pass();
    shaders::test_triangle::bind(&mut command_recorder, vbo, tex_bo);
    command_recorder.draw_array_primitives(PrimitiveMode::Triangles, 0, 3);
    command_recorder.end_pass();

    display_framebuffers.set_crtc(&card, 0);

    let mut wait_usec: i64 = 0;

    let mut i = 0;
    loop {
        let framebuffer = &display_framebuffers.framebuffers[i & 1];
        let fb_bo_idx = command_recorder.relocate_handle(framebuffer.bo.into());

        use vc4_drm::tokio::time::Duration;
        if wait_usec > 0 {
            //tokio::time::sleep(Duration::from_micros(wait_usec as u64)).await;
        }

        {
            let mut vbo_map = card.vc4_mmap_bo(&vbo).unwrap();
            let mut cur = Cursor::new(vbo_map.as_mut());

            let side_len = 3.0 / f32::sqrt(3.0) / 2.0;

            for j in 0..1 {
                let rot_mat = vc4_drm::glam::Mat2::from_angle(
                    (i + j) as f32 * 2.0 * std::f32::consts::PI / 128.0,
                );

                let v0 = rot_mat.mul_vec2([-side_len, 0.5].into());
                cur.write_all(&f32::from(v0.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v0.y).to_le_bytes()).unwrap();

                let v1 = rot_mat.mul_vec2([side_len, 0.5].into());
                cur.write_all(&f32::from(v1.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v1.y).to_le_bytes()).unwrap();

                let v2 = rot_mat.mul_vec2([0.0, -1.0].into());
                cur.write_all(&f32::from(v2.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v2.y).to_le_bytes()).unwrap();
            }
        }

        let clear_color = 0xffffffff;

        card.vc4_submit_cl_async(
            &command_recorder.bin_cl_buf,
            &command_recorder.shader_rec_buf,
            &command_recorder.uniforms,
            &command_recorder.bo_handles,
            command_recorder.shader_rec_count,
            display_framebuffers.size.0,
            display_framebuffers.size.1,
            0,
            0,
            command_recorder.width_in_tiles - 1,
            command_recorder.height_in_tiles - 1,
            drm_vc4_submit_rcl_surface::default(),
            drm_vc4_submit_rcl_surface::new_tiled_rgba8(fb_bo_idx),
            drm_vc4_submit_rcl_surface::default(),
            drm_vc4_submit_rcl_surface::default(),
            drm_vc4_submit_rcl_surface::default(),
            drm_vc4_submit_rcl_surface::default(),
            [clear_color, clear_color],
            0,
            0,
            true,
            false,
            false,
            false,
        )
        .expect("Unable to vc4_submit_cl")
        .await;

        use vc4_drm::tokio::time::Instant;
        let start = Instant::now();
        display_framebuffers.page_flip(&card, i & 1).await;
        let flip_wait = Instant::now() - start;
        let delta = flip_wait.as_micros() as i64 - Duration::from_millis(2).as_micros() as i64;
        let new_wait_usec = wait_usec + delta;
        if new_wait_usec < 1000000 / 60 - 2000 {
            wait_usec = new_wait_usec;
        } else if new_wait_usec > 1000000 / 60 - 1000 {
            wait_usec -= 1000;
        }
        //println!("{}us will wait {}us", flip_wait.as_micros(), wait_usec);

        i += 1;
    }
}

fn main() {
    use std::os::unix::process::CommandExt;
    let current_exe = std::env::current_exe().unwrap();
    if let Some(_) = std::env::args_os().find(|a| a == "--debugserver") {
        std::process::Command::new("gdbserver")
            .args([":1235", current_exe.to_str().unwrap()])
            .exec();
    }

    vc4_drm::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
