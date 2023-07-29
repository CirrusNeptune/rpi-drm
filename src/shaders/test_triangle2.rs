#![allow(unused_imports, nonstandard_style)]
use super::ShaderNode;
use rpi_drm::{Buffer, CommandEncoder, ShaderAttribute, ShaderUniform, TextureUniform};
use vc4_drm::cl::AttributeRecord;
use vc4_drm::{glam, qpu};

const CS_ASM_CODE: [u64; 23] = qpu! {
    sig_load_imm ; vr_setup = load32.always(0x00401a00) ; nop = load32.always() ;
    sig_none ; r3 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_none ; vpm = or.always(r3, r3, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    sig_none ; vpm = or.always(r2, r2) ; sfu_recip = v8min.always(r0, r0) ;
    sig_none ; vpm = or.always(a, a, ra1, uni) ; r2 = fmul.always(r2, b) ;
    sig_none ; vpm = or.always(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; r0 = fmul.always(r0, r4) ;
    sig_small_imm ; r1 = fsub.always(b, r0, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; rb0 = fmul.always(r4, r1) ;
    sig_none ; nop = nop(r0, r0, uni, nop) ; r0 = fmul.always(r3, a) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r1 = fmul.always(r0, b) ;
    sig_none ; ra0._16a = ftoi.always(r1, r1, nop, rb0) ; r3 = fmul.always(r2, b) ;
    sig_none ; ra0._16b = ftoi.always(r3, r3, ra1, uni) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r3 = fmul.always(r2, b) ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = fadd.always(r3, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb0) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static CS_ASM: ShaderNode = ShaderNode::new(&CS_ASM_CODE);

const VS_ASM_CODE: [u64; 25] = qpu! {
    sig_load_imm ; vr_setup = load32.always(0x00401a00) ; nop = load32.always() ;
    sig_none ; ra1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb1 = or.ws.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; sfu_recip = or.always(r2, r2, uni, rb1) ; r3 = fmul.always(b, a) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; r0 = fmul.always(r2, r4) ;
    sig_small_imm ; r1 = fsub.always(b, r0, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; rb0 = fmul.always(r4, r1) ;
    sig_none ; nop = nop(r0, r0, ra1, uni) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r0 = fmul.always(r2, b) ;
    sig_none ; ra0._16a = ftoi.always(r0, r0, nop, rb0) ; r3 = fmul.always(r3, b) ;
    sig_none ; ra0._16b = ftoi.always(r3, r3, ra2, uni) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb0) ; r1 = fmul.always(r0, b) ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = fadd.always(r1, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb0) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(a, a, ra1, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb1) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(a, a, ra2, nop) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

const FS_ASM_CODE: [u64; 13] = qpu! {
    sig_none ; nop = nop(r0, r0, pay_w, vary) ; r0 = fmul.always(b, a) ;
    sig_none ; ra0 = fadd.always(r0, r5, pay_w, vary) ; r1 = fmul.always(b, a) ;
    sig_none ; r0 = fadd.always(r1, r5, pay_w, vary) ; r2 = fmul.always(b, a) ;
    sig_none ; r3 = fadd.always(r2, r5) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0) ; r1._8a = v8min.always(r3, r3) ;
    sig_none ; nop = nop.pm(r0, r0) ; r1._8b = v8min.always(r0, r0) ;
    sig_none ; nop = nop.pm(r0, r0, ra0, nop) ; r1._8c = v8min.always(a, a) ;
    sig_small_imm ; nop = nop.pm(r0, r0, nop, _1_1) ; r1._8d = v8min.always(b, b) ;
    sig_color_load ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.ws(r0, r0) ; tlb_color_all = v8adds.always(r1, r4) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static FS_ASM: ShaderNode = ShaderNode::new(&FS_ASM_CODE);

pub fn bind(encoder: &mut CommandEncoder, cs_vbo: &Buffer, vs_vbo: &Buffer) {
    encoder.bind_shader(
        true,
        3,
        *FS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        &[
            ShaderAttribute {
                buffer: cs_vbo,
                record: AttributeRecord {
                    address: 0,
                    number_of_bytes_minus_1: 15,
                    stride: 16,
                    vertex_shader_vpm_offset: 0,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: false,
                cs: true,
            },
            ShaderAttribute {
                buffer: vs_vbo,
                record: AttributeRecord {
                    address: 0,
                    number_of_bytes_minus_1: 15,
                    stride: 16,
                    vertex_shader_vpm_offset: 0,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: true,
                cs: false,
            },
        ],
        &[],
        &[
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
        &[
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
    );
}
