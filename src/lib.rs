use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, OnceLock};
use vc4_drm::card::{BufferMapping, Card};
use vc4_drm::cl::*;
use vc4_drm::drm::{
    buffer,
    control::{connector, crtc, framebuffer, Device, Mode, PageFlipFlags},
};

pub struct Framebuffer {
    pub bo: Buffer,
    pub framebuffer: framebuffer::Handle,
}

pub struct DisplayFramebuffers {
    pub size: (u16, u16),
    crtc: crtc::Handle,
    pub framebuffers: [Framebuffer; 2],
    connector: connector::Handle,
    mode: Mode,
}

impl DisplayFramebuffers {
    pub fn set_crtc(&self, index: usize) {
        get_card()
            .set_crtc(
                self.crtc,
                Some(self.framebuffers[index].framebuffer),
                (0, 0),
                &[self.connector],
                Some(self.mode),
            )
            .expect("unable to set_crtc");
    }

    pub async fn page_flip(&self, index: usize) {
        let card = get_card();
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

pub fn open_and_allocate_display_framebuffers() -> DisplayFramebuffers {
    let card = get_card();
    for connector in card
        .resource_handles()
        .expect("Unable to get resource handles")
        .connectors()
    {
        let connector_info = card
            .get_connector(*connector, false)
            .expect("Unable to get_connector");
        if connector_info.state() != connector::State::Connected {
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
            Framebuffer {
                bo: Buffer::from_vc4_buffer(image_buffer.buffer()),
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

#[derive(Eq, PartialEq, Hash)]
struct BufferInner(vc4_drm::card::Buffer);

impl Drop for BufferInner {
    fn drop(&mut self) {
        get_card().vc4_destroy_bo(self.0).unwrap();
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Buffer(Arc<BufferInner>);

impl Buffer {
    pub fn new(size: u32) -> Self {
        Self(Arc::new(BufferInner(
            get_card().vc4_create_bo(size).unwrap(),
        )))
    }

    pub fn from_vc4_buffer(buffer: vc4_drm::card::Buffer) -> Self {
        Self(Arc::new(BufferInner(buffer)))
    }

    pub fn mmap(&self) -> BufferMapping {
        get_card().vc4_mmap_bo(&self.0 .0).unwrap()
    }

    pub fn handle(&self) -> buffer::Handle {
        self.0 .0.handle()
    }
}

impl From<Buffer> for buffer::Handle {
    fn from(value: Buffer) -> Self {
        value.handle()
    }
}

pub struct ShaderAttribute {
    pub buffer: Buffer,
    pub record: AttributeRecord,
    pub vs: bool,
    pub cs: bool,
}

pub struct TextureUniform {
    pub buffer: Buffer,
    pub config: TextureConfigUniform,
}

pub enum ShaderUniform {
    Texture(TextureUniform),
    Constant(u32),
}

#[derive(Default)]
struct StateTracker<T: Default + BinClStructure, const N: u16>(T);

impl<T: Default + BinClStructure, const N: u16> StateTracker<T, N> {
    fn clear(&mut self) {
        self.0 = T::default();
    }

    fn set(&mut self, dirty_bits: &mut u16, val: T) {
        self.0 = val;
        *dirty_bits |= 1 << N;
    }

    fn flush<W: Write>(&mut self, dirty_bits: u16, writer: &mut W) {
        if (dirty_bits & (1 << N)) != 0 {
            self.0.encode(writer).unwrap()
        }
    }
}

#[derive(Default)]
pub struct CommandEncoder {
    bin_cl_buf: Vec<u8>,
    shader_rec_buf: Vec<u8>,
    shader_rec_count: u32,
    uniforms: Vec<u32>,
    bo_buffer_map: HashMap<Buffer, u32>,
    bo_handle_map: HashMap<buffer::Handle, u32>,
    bo_handles: Vec<buffer::Handle>,
    window_size: (u16, u16),
    width_in_tiles: u8,
    height_in_tiles: u8,

    // State tracking
    line_width: StateTracker<LineWidth, 0>,
    clip_window: StateTracker<ClipWindow, 1>,
    clipper_xy_scaling: StateTracker<ClipperXYScaling, 2>,
    viewport_offset: StateTracker<ViewportOffset, 3>,
    configuration_bits: StateTracker<ConfigurationBits, 4>,
    depth_offset: StateTracker<DepthOffset, 5>,
    clipper_z_scale_and_offset: StateTracker<ClipperZScaleAndOffset, 6>,
    point_size: StateTracker<PointSize, 7>,
    flat_shade_flags: StateTracker<FlatShadeFlags, 8>,
    dirty_bits: u16,
}

macro_rules! expand_commands {
    ($sub_macro:ident $(,$args:ident)*) => {
        $sub_macro!(set_line_width, line_width, LineWidth $(,$args)*);
        $sub_macro!(set_clip_window, clip_window, ClipWindow $(,$args)*);
        $sub_macro!(set_clipper_xy_scaling, clipper_xy_scaling, ClipperXYScaling $(,$args)*);
        $sub_macro!(set_viewport_offset, viewport_offset, ViewportOffset $(,$args)*);
        $sub_macro!(
            set_configuration_bits,
            configuration_bits,
            ConfigurationBits
            $(,$args)*
        );
        $sub_macro!(set_depth_offset, depth_offset, DepthOffset $(,$args)*);
        $sub_macro!(
            set_clipper_z_scale_and_offset,
            clipper_z_scale_and_offset,
            ClipperZScaleAndOffset
            $(,$args)*
        );
        $sub_macro!(set_point_size, point_size, PointSize $(,$args)*);
        $sub_macro!(set_flat_shade_flags, flat_shade_flags, FlatShadeFlags $(,$args)*);
    }
}

macro_rules! command_recorder_set {
    ($setter:ident, $var:ident, $typename:path) => {
        pub fn $setter(&mut self, $var: $typename) {
            self.$var.set(&mut self.dirty_bits, $var);
        }
    };
}

macro_rules! command_recorder_clear {
    ($setter:ident, $var:ident, $typename:path, $self:ident) => {
        $self.$var.clear();
    };
}

macro_rules! command_recorder_flush {
    ($setter:ident, $var:ident, $typename:path, $self:ident) => {
        $self.$var.flush($self.dirty_bits, &mut $self.bin_cl_buf);
    };
}

impl CommandEncoder {
    pub fn new(window_size: (u16, u16)) -> Self {
        let mut obj = Self::default();
        obj.window_size = window_size;
        obj
    }

    pub fn window_size(&self) -> (u16, u16) {
        self.window_size
    }

    pub fn relocate_buffer(&mut self, buffer: Buffer) -> u32 {
        if let Some(index) = self.bo_buffer_map.get(&buffer) {
            *index
        } else {
            let index = self.bo_handles.len() as u32;
            self.bo_handles.push(buffer.handle());
            self.bo_buffer_map.insert(buffer, index);
            index
        }
    }

    pub fn relocate_handle(&mut self, handle: buffer::Handle) -> u32 {
        if let Some(index) = self.bo_handle_map.get(&handle) {
            *index
        } else {
            let index = self.bo_handles.len() as u32;
            self.bo_handles.push(handle);
            self.bo_handle_map.insert(handle, index);
            index
        }
    }

    pub fn clear(&mut self) {
        self.bin_cl_buf.clear();
        self.shader_rec_buf.clear();
        self.shader_rec_count = 0;
        self.uniforms.clear();
        self.bo_buffer_map.clear();
        self.bo_handle_map.clear();
        self.bo_handles.clear();

        expand_commands!(command_recorder_clear, self);
        self.dirty_bits = 0;
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
            depth_offset_factor: 0.0,
            depth_offset_units: 0.0,
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
                let tex_idx = self.relocate_buffer(tex.buffer.clone());
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

    pub fn set_depth_test(
        &mut self,
        depth_test: bool,
        compare_function: CompareFunction,
        depth_write: bool,
    ) {
        let mut new_configuration_bits = self.configuration_bits.0;
        new_configuration_bits.early_z_enable = depth_test;
        new_configuration_bits.z_updates_enable = depth_test && depth_write;
        new_configuration_bits.depth_test_function = if depth_test {
            compare_function
        } else {
            CompareFunction::Always
        };
        self.set_configuration_bits(new_configuration_bits);
    }

    pub fn set_cull_test(
        &mut self,
        enable_forward_facing_primitive: bool,
        enable_reverse_facing_primitive: bool,
    ) {
        let mut new_configuration_bits = self.configuration_bits.0;
        new_configuration_bits.enable_forward_facing_primitive = enable_forward_facing_primitive;
        new_configuration_bits.enable_reverse_facing_primitive = enable_reverse_facing_primitive;
        self.set_configuration_bits(new_configuration_bits);
    }

    pub fn bind_shader(
        &mut self,
        fs_single_threaded: bool,
        fs_number_of_varyings: u8,
        fs: buffer::Handle,
        vs: buffer::Handle,
        cs: buffer::Handle,
        attributes: &[ShaderAttribute],
        fs_uniforms: &[ShaderUniform],
        vs_uniforms: &[ShaderUniform],
        cs_uniforms: &[ShaderUniform],
    ) {
        GlShaderState {
            address: 0,
            extended_shader_record: false,
            number_of_attribute_arrays: attributes.len() as _,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();

        let fs_idx = self.relocate_handle(fs);
        let vs_idx = self.relocate_handle(vs);
        let cs_idx = self.relocate_handle(cs);

        self.shader_rec_count += 1;
        self.shader_rec_buf
            .write_all(&fs_idx.to_le_bytes())
            .unwrap();
        self.shader_rec_buf
            .write_all(&vs_idx.to_le_bytes())
            .unwrap();
        self.shader_rec_buf
            .write_all(&cs_idx.to_le_bytes())
            .unwrap();

        let mut vs_attributes_size = 0;
        let mut vs_attributes_bits = 0;
        let mut cs_attributes_size = 0;
        let mut cs_attributes_bits = 0;
        for (i, attribute) in attributes.iter().enumerate() {
            let buf_idx = self.relocate_buffer(attribute.buffer.clone());
            self.shader_rec_buf
                .write_all(&buf_idx.to_le_bytes())
                .unwrap();
            if attribute.vs {
                vs_attributes_size += attribute.record.number_of_bytes_minus_1 + 1;
                vs_attributes_bits |= 1 << i;
            }
            if attribute.cs {
                cs_attributes_size += attribute.record.number_of_bytes_minus_1 + 1;
                cs_attributes_bits |= 1 << i;
            }
        }

        GlShaderRecord {
            fragment_shader_is_single_threaded: fs_single_threaded,
            point_size_included_in_shaded_vertex_data: false,
            enable_clipping: true,
            fragment_shader_number_of_uniforms_not_used_currently: 0,
            fragment_shader_number_of_varyings: fs_number_of_varyings,
            fragment_shader_code_address_offset: 0,
            fragment_shader_uniforms_address: 0,
            vertex_shader_number_of_uniforms_not_used_currently: 0,
            vertex_shader_attribute_array_select_bits: vs_attributes_bits,
            vertex_shader_total_attributes_size: vs_attributes_size,
            vertex_shader_code_address_offset: 0,
            vertex_shader_uniforms_address: 0,
            coordinate_shader_number_of_uniforms_not_used_currently: 0,
            coordinate_shader_attribute_array_select_bits: cs_attributes_bits,
            coordinate_shader_total_attributes_size: cs_attributes_size,
            coordinate_shader_code_address_offset: 0,
            coordinate_shader_uniforms_address: 0,
        }
        .encode(&mut self.shader_rec_buf)
        .unwrap();

        for attr in attributes {
            attr.record.encode(&mut self.shader_rec_buf).unwrap();
        }

        self.add_uniform_relocs(fs_uniforms);
        self.add_uniform_relocs(vs_uniforms);
        self.add_uniform_relocs(cs_uniforms);

        self.add_uniforms(fs_uniforms);
        self.add_uniforms(vs_uniforms);
        self.add_uniforms(cs_uniforms);
    }

    pub fn draw_array_primitives(
        &mut self,
        primitive_mode: PrimitiveMode,
        start: u32,
        length: u32,
    ) {
        self.flush_state();

        VertexArrayPrimitives {
            index_of_first_vertex: start,
            length,
            primitive_mode,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();
    }

    pub fn draw_indexed_primitives(
        &mut self,
        index_buffer: Buffer,
        index_type: IndexType,
        primitive_mode: PrimitiveMode,
        start: u32,
        length: u32,
        maximum_index: u32,
    ) {
        self.flush_state();

        let index_buffer_idx = self.relocate_buffer(index_buffer);

        GemRelocations {
            buffer0: index_buffer_idx,
            buffer1: 0,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();

        IndexedPrimitiveList {
            index_type,
            primitive_mode,
            length,
            address_of_indices_list: (index_type as u32 + 1) * start,
            maximum_index,
        }
        .encode(&mut self.bin_cl_buf)
        .unwrap();
    }

    fn flush_state(&mut self) {
        if self.dirty_bits != 0 {
            expand_commands!(command_recorder_flush, self);
            self.dirty_bits = 0;
        }
    }

    expand_commands!(command_recorder_set);

    pub async fn submit(&mut self, clear_color: u32, color_write: Buffer) {
        use vc4_drm::card::drm_vc4_submit_rcl_surface;
        let fb_bo_idx = self.relocate_buffer(color_write);
        get_card()
            .vc4_submit_cl_async(
                &self.bin_cl_buf,
                &self.shader_rec_buf,
                &self.uniforms,
                &self.bo_handles,
                self.shader_rec_count,
                self.window_size.0,
                self.window_size.1,
                0,
                0,
                self.width_in_tiles - 1,
                self.height_in_tiles - 1,
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
    }
}
