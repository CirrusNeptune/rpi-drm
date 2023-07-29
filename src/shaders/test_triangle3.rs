#![allow(unused_imports, nonstandard_style)]
use super::ShaderNode;
use rpi_drm::{Buffer, CommandEncoder, ShaderAttribute, ShaderUniform, TextureUniform};
use vc4_drm::cl::AttributeRecord;
use vc4_drm::{glam, qpu};

const CS_ASM_CODE: [u64; 26] = qpu! {
    sig_small_imm ; tmu0_t = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_small_imm ; tmu0_b = or.ws.always(b, b, nop, _0) ; nop = nop(r0, r0) ;
    sig_small_imm ; tmu0_s = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_load_imm ; vr_setup = load32.always(0x00101a00) ; nop = load32.always() ;
    sig_load_tmu0 ; nop = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(r4, r4) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fmax.always(a, a, ra1, uni) ; r2 = fmul.always(a, b) ;
    sig_none ; sfu_recip = or.ws.always(r1, r1) ; vpm = v8min.always(r1, r1) ;
    sig_none ; vpm = or.always(r1, r1, ra1, uni) ; r3 = fmul.always(a, b) ;
    sig_none ; vpm = or.always(r1, r1) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(r1, r1, ra1, nop) ; r0 = fmul.always(a, r4) ;
    sig_small_imm ; r1 = fsub.always(b, r0, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; rb0 = fmul.always(r4, r1) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r0 = fmul.always(r2, b) ;
    sig_none ; ra0._16a = ftoi.always(r0, r0, nop, rb0) ; r2 = fmul.always(r3, b) ;
    sig_none ; ra0._16b = ftoi.always(r2, r2, ra1, uni) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r1 = fmul.always(r0, b) ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = fadd.always(r1, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb0) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static CS_ASM: ShaderNode = ShaderNode::new(&CS_ASM_CODE);

const VS_ASM_CODE: [u64; 26] = qpu! {
    sig_small_imm ; tmu0_t = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_small_imm ; tmu0_b = or.ws.always(b, b, nop, _0) ; nop = nop(r0, r0) ;
    sig_small_imm ; tmu0_s = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_load_imm ; vr_setup = load32.always(0x00101a00) ; nop = load32.always() ;
    sig_load_tmu0 ; nop = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(r4, r4) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fmax.always(a, a, ra1, uni) ; r3 = fmul.always(a, b) ;
    sig_none ; sfu_recip = or.ws.always(r1, r1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra1, nop) ; r0 = fmul.always(a, r4) ;
    sig_small_imm ; r2 = fsub.always(b, r0, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; rb0 = fmul.always(r4, r2) ;
    sig_none ; nop = nop(r0, r0, ra1, uni) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r1 = fmul.always(r3, b) ;
    sig_none ; ra0._16a = ftoi.always(r1, r1, nop, rb0) ; r3 = fmul.always(r0, b) ;
    sig_none ; ra0._16b = ftoi.always(r3, r3, ra1, uni) ; r1 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r2 = fmul.always(r1, b) ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = fadd.always(r2, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb0) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

const FS_ASM_CODE: [u64; 19] = qpu! {
    sig_small_imm ; tmu0_t = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_small_imm ; tmu0_s = or.ws.always(b, b, nop, _1_2) ; nop = nop(r0, r0) ;
    sig_none ; r1 = itof.always(b, b, pay_w, y_pix) ; sfu_recip = v8min.always(a, a) ;
    sig_last_thread_switch ; rb0 = itof.ws.always(b, b, uni, pay_z) ; ra0 = fmul.always(r1, a) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(r4, r4) ; nop = nop(r0, r0) ;
    sig_load_tmu0 ; nop = nop(r0, r0, uni, rb0) ; r3 = fmul.always(b, a) ;
    sig_none ; r2 = fadd.always(a, b, ra0, uni) ; nop = nop(r0, r0) ;
    sig_none ; r3 = fadd.pm.always(r4, r3) ; nop = nop(r0, r0) ;
    sig_none ; r0 = itof.pm.always(a, a, x_pix, nop) ; r3._8a = v8min.always(r3, r3) ;
    sig_none ; r2 = fadd.pm.always(r4, r2) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.pm.always(r4, r0) ; r3._8b = v8min.always(r2, r2) ;
    sig_none ; r1 = fadd.pm.always(r4, a, ra1, nop) ; r3._8c = v8min.always(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0) ; r3._8d = v8min.always(r1, r1) ;
    sig_color_load ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.ws(r0, r0) ; tlb_color_all = v8adds.always(r3, r4) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static FS_ASM: ShaderNode = ShaderNode::new(&FS_ASM_CODE);

pub fn bind(encoder: &mut CommandEncoder, t: &TextureUniform, tex1: &TextureUniform) {
    encoder.bind_shader(
        false,
        0,
        *FS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        &[],
        &[
            ShaderUniform::Texture(tex1),
            ShaderUniform::Constant(0x33800001),
        ],
        &[
            ShaderUniform::Texture(t),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
        &[
            ShaderUniform::Texture(t),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
    );
}
