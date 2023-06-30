#![recursion_limit = "10000"]

mod qpu;
mod vc4_card;
mod vc4_cl;
pub mod vc4_image;

use drm::control::{connector::State, Device};
use std::io::Write;
use vc4_card::{drm_vc4_submit_rcl_surface, Buffer, Card};
use vc4_cl::*;

use std::io::Cursor;

struct DisplayFramebuffers {
    pub size: (u16, u16),
    pub bo: Buffer,
    crtc: drm::control::crtc::Handle,
    framebuffer: drm::control::framebuffer::Handle,
    connector: drm::control::connector::Handle,
    mode: drm::control::Mode,
}

impl DisplayFramebuffers {
    pub fn set_crtc(&self, card: &Card) {
        card.set_crtc(
            self.crtc,
            Some(self.framebuffer),
            (0, 0),
            &[self.connector],
            Some(self.mode),
        )
        .expect("unable to set_crtc");
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
        let image_buffer = card
            .vc4_create_bgra_image_buffer((mode.size().0 as u32, mode.size().1 as u32))
            .expect("unable to create image buffer");
        let framebuffer = card
            .add_framebuffer(&image_buffer, 32, 32)
            .expect("unable to add framebuffer");
        return DisplayFramebuffers {
            size: mode.size(),
            bo: image_buffer.buffer(),
            crtc,
            framebuffer,
            connector: *connector,
            mode,
        };
    }
    panic!("Couldn't find a display");
}

async fn async_main() {
    let card = Card::open_global();

    let display_framebuffers = open_and_allocate_display_framebuffers(&card);

    const VS_ASM: [u64; 14] = qpu! {
        //0x40000000 = 2.0
        //uni = 1.0
        //rb0 = 2 - 1 = 1
        sig_small_imm ; rb0 = fsub.ws.always(b, a, uni, _2_1) ; nop = nop(r0, r0) ;
        //set up VPM read for subsequent reads
        //0x00201a00: 0000 0000 0010 0000 0001 1010 0000 0000
        //addr: 0
        //size: 32bit
        //packed
        //horizontal
        //stride=1
        //vectors to read = 2 (how many components)
        sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(2, 1, 0)) ; nop = load32.always() ;
        //uni = viewportXScale
        //r0 = vpm * uni
        sig_none ; nop = nop(r0, r0, vpm_read, uni) ; r0 = fmul.always(a, b) ;
        //r1 = r0 * rb0 (1)
        sig_none ; nop = nop(r0, r0, nop, rb0) ; r1 = fmul.always(r0, b) ;
        //uni = viewportYScale
        //ra0.16a = int(r1), r2 = vpm * uni
        sig_none ; ra0._16a = ftoi.always(r1, r1, vpm_read, uni) ; r2 = fmul.always(a, b) ;
        //r3 = r2 * rb0
        sig_none ; nop = nop(r0, r0, nop, rb0) ; r3 = fmul.always(r2, b) ;
        //ra0.16b = int(r3)
        sig_none ; ra0._16b = ftoi.always(r3, r3) ; nop = nop(r0, r0) ;
        //set up VPM write for subsequent writes
        //0x00001a00: 0000 0000 0000 0000 0001 1010 0000 0000
        //addr: 0
        //size: 32bit
        //horizontal
        //stride = 1
        sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
        //shaded vertex format for PSE
        // Ys and Xs
        //vpm = ra0
        sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0);
        // Zs
        //uni = 0.5
        //vpm = uni
        sig_none ; vpm = or.always(a, a, uni, nop) ; nop = nop(r0, r0);
        // 1.0 / Wc
        //vpm = rb0 (1)
        sig_none ; vpm = or.always(b, b, nop, rb0) ; nop = nop(r0, r0);
        //END
        sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    };

    const CS_ASM: [u64; 18] = qpu! {
        //uni = 1.0
        //r3 = 2.0 - uni
        sig_small_imm ; r3 = fsub.always(b, a, uni, _2_1) ; nop = nop(r0, r0);
        sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(2, 1, 0)) ; nop = load32.always() ;
        //r2 = vpm
        sig_none ; r2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0);
        sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
        //shaded coordinates format for PTB
        // write Xc
        //r1 = vpm, vpm = r2
        sig_none ; r1 = or.always(a, a, vpm_read, nop) ; vpm = v8min.always(r2, r2);
        // write Yc
        //uni = viewportXscale
        //vpm = r1, r2 = r2 * uni
        sig_none ; vpm = or.always(r1, r1, uni, nop) ; r2 = fmul.always(r2, a);
        //uni = viewportYscale
        //r1 = r1 * uni
        sig_none ; nop = nop(r0, r0, uni, nop) ; r1 = fmul.always(r1, a);
        //r0 = r2 * r3
        sig_none ; nop = nop(r0, r0) ; r0 = fmul.always(r2, r3);
        //ra0.16a = r0, r1 = r1 * r3
        sig_none ; ra0._16a = ftoi.always(r0, r0) ; r1 = fmul.always(r1, r3) ;
        //ra0.16b = r1
        sig_none ; ra0._16b = ftoi.always(r1, r1) ; nop = nop(r0, r0) ;
        //write Zc
        //vpm = 0
        sig_small_imm ; vpm = or.always(b, b, nop, _0) ; nop = nop(r0, r0) ;
        //write Wc
        //vpm = 1.0
        sig_small_imm ; vpm = or.always(b, b, nop, _1_1) ; nop = nop(r0, r0) ;
        //write Ys and Xs
        //vpm = ra0
        sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
        //write Zs
        //uni = 0.5
        //vpm = uni
        sig_none ; vpm = or.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
        //write 1/Wc
        //vpm = r3
        sig_none ; vpm = or.always(r3, r3) ; nop = nop(r0, r0) ;
        //END
        sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    };

    const FS_ASM: [u64; 6] = qpu! {
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_load_imm ; r0 = load32.always(0xffa14ccc) ; nop = load32() ;
        sig_none ; tlb_color_all = or.always(r0, r0) ; nop = nop(r0, r0) ;
        sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    };

    const FS_ASM_TEX: [u64; 19] = qpu! {
        sig_none ; r0 = itof.always(b, b, x_pix, y_pix) ; nop = nop(r0, r0) ;
        sig_load_imm ; r2 = load32.always(0x3a72b9d6) ; nop = load32() ; //1/1080
        sig_none ; r0 = itof.always(a, a, x_pix, y_pix) ; r1 = fmul.always(r2, r0) ; //r1 contains tex coord y
        sig_load_imm ; r2 = load32.always(0x3a088888) ; nop = load32() ; //1/1920
        //write texture addresses (x, y)
        //writing tmu0_s signals that all coordinates are written
        sig_none ; tmu0_t = or.always(r1, r1) ; r0 = fmul.always(r2, r0) ; //r0 contains tex coord x
        sig_none ; tmu0_s = or.always(r0, r0) ; nop = nop(r0, r0) ;
        //suspend thread (after 2 nops) to wait for TMU request to finish
        sig_thread_switch ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        //read TMU0 request result to R4
        sig_load_tmu0 ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        //when thread has been awakened, MOV from R4 to R0
        sig_none ; r0 = fmax.pm.always._8a(r4, r4) ; nop = nop(r0, r0) ;
        sig_none ; r1 = fmax.pm.always._8b(r4, r4) ; r0._8a = v8min.always(r0, r0) ;
        sig_none ; r2 = fmax.pm.always._8c(r4, r4) ; r0._8b = v8min.always(r1, r1) ;
        sig_none ; r3 = fmax.pm.always._8d(r4, r4) ; r0._8c = v8min.always(r2, r2) ;
        sig_none ; nop = nop.pm(r0, r0) ; r0._8d = v8min.always(r3, r3) ;
        sig_none ; tlb_color_all = or.always(r0, r0) ; nop = nop(r0, r0) ;
        sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
        sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    };

    let vs = card
        .vc4_create_shader_bo(&VS_ASM)
        .expect("unable to create vs");
    let cs = card
        .vc4_create_shader_bo(&CS_ASM)
        .expect("unable to create cs");
    let fs = card
        .vc4_create_shader_bo(&FS_ASM)
        .expect("unable to create fs");

    let vbo = card.vc4_create_bo(24 * 1000).unwrap();

    let uniforms = [
        // VS uniforms
        u32::from_le_bytes(1.0_f32.to_le_bytes()),
        u32::from_le_bytes(((display_framebuffers.size.0 * 16 / 2) as f32).to_le_bytes()),
        u32::from_le_bytes(((display_framebuffers.size.1 * 16 / 2) as f32).to_le_bytes()),
        u32::from_le_bytes(1.0_f32.to_le_bytes()),
        // CS uniforms
        u32::from_le_bytes(1.0_f32.to_le_bytes()),
        u32::from_le_bytes(((display_framebuffers.size.0 * 16 / 2) as f32).to_le_bytes()),
        u32::from_le_bytes(((display_framebuffers.size.1 * 16 / 2) as f32).to_le_bytes()),
        u32::from_le_bytes(1.0_f32.to_le_bytes()),
    ];

    let mut bin_cl_buf = Vec::<u8>::new();
    let mut shader_rec_buf = Vec::<u8>::new();
    let mut shader_rec_count = 0;

    let mut tile_bin_config = TileBinningModeConfiguration::default();
    tile_bin_config.set_size_in_pixels(display_framebuffers.size.0, display_framebuffers.size.1);
    tile_bin_config
        .encode(&mut bin_cl_buf)
        .expect("unable to write TileBinningModeConfiguration");

    // START_TILE_BINNING resets the statechange counters in the hardware,
    // which are what is used when a primitive is binned to a tile to
    // figure out what new state packets need to be written to that tile's
    // command list.
    StartTileBinning {}
        .encode(&mut bin_cl_buf)
        .expect("unable to write StartTileBinning");

    LineWidth { line_width: 0.0 }
        .encode(&mut bin_cl_buf)
        .unwrap();

    ClipWindow {
        clip_window_left_pixel_coordinate: 0,
        clip_window_bottom_pixel_coordinate: 0,
        clip_window_width_in_pixels: display_framebuffers.size.0,
        clip_window_height_in_pixels: display_framebuffers.size.1,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    ClipperXYScaling {
        viewport_half_width_in_1_16th_of_pixel: (display_framebuffers.size.0 * 16 / 2) as f32,
        viewport_half_height_in_1_16th_of_pixel: (display_framebuffers.size.1 * 16 / 2) as f32,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    ViewportOffset {
        viewport_centre_x_coordinate_12_4: display_framebuffers.size.0 * 16 / 2,
        viewport_centre_y_coordinate_12_4: display_framebuffers.size.1 * 16 / 2,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    let depth_test_enable = false;
    let depth_write_enable = false;
    ConfigurationBits {
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
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    DepthOffset {
        depth_offset_factor: 0,
        depth_offset_units: 0,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    ClipperZScaleAndOffset {
        viewport_z_scale_zc_to_zs: 1.0,
        viewport_z_offset_zc_to_zs: 0.0,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    PointSize { point_size: 1.0 }
        .encode(&mut bin_cl_buf)
        .unwrap();

    FlatShadeFlags {
        flat_shading_flags: 0,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    GlShaderState {
        address: 0,
        extended_shader_record: false,
        number_of_attribute_arrays: 1,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    shader_rec_count += 1;
    shader_rec_buf.write_all(&1_u32.to_le_bytes()).unwrap();
    shader_rec_buf.write_all(&2_u32.to_le_bytes()).unwrap();
    shader_rec_buf.write_all(&3_u32.to_le_bytes()).unwrap();
    shader_rec_buf.write_all(&4_u32.to_le_bytes()).unwrap();

    GlShaderRecord {
        fragment_shader_is_single_threaded: true,
        point_size_included_in_shaded_vertex_data: false,
        enable_clipping: true,
        fragment_shader_number_of_uniforms_not_used_currently: 0,
        fragment_shader_number_of_varyings: 0,
        fragment_shader_code_address_offset: 0,
        fragment_shader_uniforms_address: 0,
        vertex_shader_number_of_uniforms_not_used_currently: 0,
        vertex_shader_attribute_array_select_bits: 1 << 0,
        vertex_shader_total_attributes_size: 8,
        vertex_shader_code_address_offset: 0,
        vertex_shader_uniforms_address: 0,
        coordinate_shader_number_of_uniforms_not_used_currently: 0,
        coordinate_shader_attribute_array_select_bits: 1 << 0,
        coordinate_shader_total_attributes_size: 8,
        coordinate_shader_code_address_offset: 0,
        coordinate_shader_uniforms_address: 0,
    }
    .encode(&mut shader_rec_buf)
    .unwrap();

    AttributeRecord {
        address: 0,
        number_of_bytes_minus_1: 7,
        stride: 8,
        vertex_shader_vpm_offset: 0,
        coordinate_shader_vpm_offset: 0,
    }
    .encode(&mut shader_rec_buf)
    .unwrap();

    VertexArrayPrimitives {
        index_of_first_vertex: 0,
        length: 3 * 1000,
        primitive_mode: PrimitiveMode::Triangles,
    }
    .encode(&mut bin_cl_buf)
    .unwrap();

    // Increment the semaphore indicating that binning is done and
    // unblocking the render thread.  Note that this doesn't act
    // until the FLUSH completes.
    // The FLUSH caps all of our bin lists with a
    // VC4_PACKET_RETURN.
    IncrementSemaphore {}
        .encode(&mut bin_cl_buf)
        .expect("unable to write IncrementSemaphore");
    Flush {}
        .encode(&mut bin_cl_buf)
        .expect("unable to write IncrementSemaphore");

    let mut color_write = drm_vc4_submit_rcl_surface::default();
    color_write.hindex = 0;
    const VC4_RENDER_CONFIG_FORMAT_RGBA8888: u16 = 1;
    const VC4_TILING_FORMAT_LINEAR: u16 = 0;
    const VC4_TILING_FORMAT_T: u16 = 1;
    const VC4_TILING_FORMAT_LT: u16 = 2;
    color_write.bits = VC4_RENDER_CONFIG_FORMAT_RGBA8888 << 2 | VC4_TILING_FORMAT_T << 6;

    let bo_handles = [display_framebuffers.bo.into(), fs, vs, cs, vbo.handle()];
    display_framebuffers.set_crtc(&card);

    let mut i = 0;
    loop {
        {
            let mut vbo_map = card.vc4_mmap_bo(&vbo).unwrap();
            let mut cur = Cursor::new(vbo_map.as_mut());

            let side_len = 3.0 / f32::sqrt(3.0) / 2.0;

            for j in 0..1000 {
                let rot_mat =
                    glam::Mat2::from_angle(j as f32 * 2.0 * std::f32::consts::PI / 2048.0);

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
            &bin_cl_buf,
            &shader_rec_buf,
            &uniforms,
            &bo_handles,
            shader_rec_count,
            display_framebuffers.size.0,
            display_framebuffers.size.1,
            0,
            0,
            tile_bin_config.width_in_tiles - 1,
            tile_bin_config.height_in_tiles - 1,
            drm_vc4_submit_rcl_surface::default(),
            color_write,
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

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
