use super::ShaderNode;
use rpi_drm::{Buffer, CommandEncoder, ShaderAttribute, ShaderUniform};
use vc4_drm::cl::AttributeRecord;
use vc4_drm::qpu;

const VS_ASM_CODE: [u64; 28] = qpu! {
    sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(4, 1, 0)) ; nop = load32.always() ;
    sig_none ; r0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read X
    sig_none ; ra1 = or.always(r0, r0) ; nop = nop(r0, r0) ; // Write ra1 = X
    sig_none ; r1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read Y
    sig_none ; ra2 = or.always(r1, r1) ; nop = nop(r0, r0) ; // Write ra2 = Y
    sig_none ; r2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read Z
    sig_none ; ra3 = or.always(r2, r2) ; nop = nop(r0, r0) ; // Write ra3 = Z
    sig_none ; r3 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read W
    sig_none ; sfu_recip = or.always(r3, r3) ; nop = nop(r0, r0) ; // Recip W

    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ; // Wait

    sig_none ; nop = nop(r0, r0, uni, nop) ; r0 = fmul.always(r0, a) ;
    sig_none ; nop = nop(r0, r0) ; r0 = fmul.always(r0, r4) ;
    sig_none ; ra0._16a = ftoi.always(r0, r0) ; nop = nop(r0, r0) ; // Xs = X * viewportXscale / W
    sig_none ; nop = nop(r0, r0, uni, nop) ; r1 = fmul.always(r1, a) ;
    sig_none ; nop = nop(r0, r0) ; r1 = fmul.always(r1, r4) ;
    sig_none ; ra0._16b = ftoi.always(r1, r1) ; nop = nop(r0, r0) ; // Xs = X * viewportXscale / W

    sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ; // Write Ys | Xs

    sig_none ; nop = nop(r0, r0) ; r2 = fmul.always(r2, r4) ;
    sig_small_imm ; nop = nop(r0, r0, nop, _1_2) ; r2 = fmul.always(r2, b) ;
    sig_small_imm ; vpm = fadd.always(r2, b, nop, _1_2) ; nop = nop(r0, r0) ; // Write Zs = Z / W * 0.5 + 0.5

    sig_none ; vpm = or.always(r4, r4) ; nop = nop(r0, r0) ; // Write 1 / Wc

    // X Y Z varyings
    sig_none ; vpm = or.always(a, a, ra1, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(a, a, ra2, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(a, a, ra3, nop) ; nop = nop(r0, r0) ;

    // END
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

const CS_ASM_CODE: [u64; 25] = qpu! {
    sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(4, 1, 0)) ; nop = load32.always() ;
    sig_none ; r0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read X
    sig_none ; r1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read Y
    sig_none ; r2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read Z
    sig_none ; r3 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ; // Read W
    sig_none ; sfu_recip = or.always(r3, r3) ; nop = nop(r0, r0) ; // Recip W

    sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
    sig_none ; vpm = or.always(r0, r0) ; nop = nop(r0, r0) ; // Write Xc
    sig_none ; vpm = or.always(r1, r1) ; nop = nop(r0, r0) ; // Write Yc

    sig_none ; nop = nop(r0, r0, uni, nop) ; r0 = fmul.always(r0, a) ;
    sig_none ; nop = nop(r0, r0) ; r0 = fmul.always(r0, r4) ;
    sig_none ; ra0._16a = ftoi.always(r0, r0) ; nop = nop(r0, r0) ; // Xs = X * viewportXscale / W
    sig_none ; nop = nop(r0, r0, uni, nop) ; r1 = fmul.always(r1, a) ;
    sig_none ; nop = nop(r0, r0) ; r1 = fmul.always(r1, r4) ;
    sig_none ; ra0._16b = ftoi.always(r1, r1) ; nop = nop(r0, r0) ; // Xs = X * viewportXscale / W

    sig_none ; vpm = or.always(r2, r2) ; nop = nop(r0, r0) ; // Write Zc
    sig_none ; vpm = or.always(r3, r3) ; nop = nop(r0, r0) ; // Write Wc

    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ; // Write Ys | Xs

    sig_none ; nop = nop(r0, r0) ; r2 = fmul.always(r2, r4) ;
    sig_small_imm ; nop = nop(r0, r0, nop, _1_2) ; r2 = fmul.always(r2, b) ;
    sig_small_imm ; vpm = fadd.always(r2, b, nop, _1_2) ; nop = nop(r0, r0) ; // Write Zs = Z / W * 0.5 + 0.5

    sig_none ; vpm = or.always(r4, r4) ; nop = nop(r0, r0) ; // Write 1 / Wc

    // END
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static CS_ASM: ShaderNode = ShaderNode::new(&CS_ASM_CODE);

const FS_ASM_TEX_CODE: [u64; 18] = qpu! {
    sig_none ; r0 = itof.always(b, b, x_pix, y_pix) ; nop = nop(r0, r0) ;
    sig_load_imm ; r2 = load32.always(qpu::transmute_f32(1.0 / 480.0)) ; nop = load32() ; //1/480
    sig_none ; r0 = itof.always(a, a, x_pix, y_pix) ; r1 = fmul.always(r2, r0) ; //r1 contains tex coord y
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
pub static FS_ASM_TEX: ShaderNode = ShaderNode::new(&FS_ASM_TEX_CODE);

const FS_ASM_CODE: [u64; 14] = qpu! {
    sig_load_imm ; r0 = load32.always(0xffa14ccc) ; nop = load32() ;

    sig_none ; nop = nop(r0, r0, pay_w, vary) ; r1 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r5) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0) ; r0._8c = v8min.always(r1, r1) ;

    sig_none ; nop = nop(r0, r0, pay_w, vary) ; r1 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r5) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0) ; r0._8b = v8min.always(r1, r1) ;

    sig_none ; nop = nop(r0, r0, pay_w, vary) ; r1 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r5) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0) ; r0._8a = v8min.always(r1, r1) ;

    sig_none ; tlb_color_all = or.always(r0, r0) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static FS_ASM: ShaderNode = ShaderNode::new(&FS_ASM_CODE);

pub fn bind(encoder: &mut CommandEncoder, vbo_vs: &Buffer) {
    let vs_uniforms = [
        ShaderUniform::Constant(qpu::transmute_f32(
            (encoder.window_size().0 * 16 / 2) as f32,
        )),
        ShaderUniform::Constant(qpu::transmute_f32(
            (encoder.window_size().1 * 16 / 2) as f32,
        )),
    ];
    encoder.bind_shader(
        true,
        3,
        *FS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        &[ShaderAttribute {
            buffer: vbo_vs,
            record: AttributeRecord {
                address: 0,
                number_of_bytes_minus_1: 15,
                stride: 16,
                vertex_shader_vpm_offset: 0,
                coordinate_shader_vpm_offset: 0,
            },
            vs: true,
            cs: true,
        }],
        &[/*ShaderUniform::Texture(TextureUniform {
            buffer: tex,
            config: TextureConfigUniform {
                base_address: 0,
                cache_swizzle: 0,
                cube_map: false,
                flip_y: false,
                data_type: TextureDataType::RGBA8888,
                num_mips: 1,
                height: 256u16,
                etc_flip: false,
                width: 256u16,
                mag_filt: TextureMagFilterType::Linear,
                min_filt: TextureMinFilterType::Linear,
                wrap_t: TextureWrapType::Repeat,
                wrap_s: TextureWrapType::Repeat,
            },
        })*/],
        &vs_uniforms,
        &vs_uniforms,
    )
}
