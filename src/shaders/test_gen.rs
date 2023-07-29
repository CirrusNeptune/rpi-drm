#![allow(unused_imports, nonstandard_style)]
use super::ShaderNode;
use rpi_drm::{Buffer, CommandEncoder, ShaderAttribute, ShaderUniform, TextureUniform};
use vc4_drm::cl::AttributeRecord;
use vc4_drm::{glam, qpu};

const CS_ASM_CODE: [u64; 45] = qpu! {
    sig_load_imm ; vr_setup = load32.always(0x00301a00) ; nop = load32.always() ;
    sig_none ; ra3 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb3 = or.ws.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra3, uni) ; r1 = fmul.always(b, a) ;
    sig_none ; nop = nop(r0, r0, uni, rb3) ; r0 = fmul.always(a, b) ;
    sig_none ; ra5 = fadd.always(r1, r0, ra3, uni) ; r2 = fmul.always(b, a) ;
    sig_none ; nop = nop(r0, r0, uni, rb3) ; r3 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, uni, rb3) ; r1 = fmul.always(a, b) ;
    sig_none ; rb2 = fadd.ws.always(r2, r1, ra3, uni) ; r0 = fmul.always(b, a) ;
    sig_none ; ra4 = fadd.always(r0, r3, uni, rb3) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra3, uni) ; r3 = fmul.always(b, a) ;
    sig_none ; ra2 = fadd.always(r3, r2, vpm_read, nop) ; r2 = v8min.always(a, a) ;
    sig_none ; nop = nop(r0, r0, uni, nop) ; r0 = fmul.always(a, r2) ;
    sig_none ; rb1 = fadd.ws.always(a, r0, ra5, uni) ; r3 = fmul.always(b, r2) ;
    sig_none ; r0 = fadd.always(a, r3, ra4, uni) ; r1 = fmul.always(b, r2) ;
    sig_none ; r3 = fadd.always(b, r1, uni, rb2) ; r2 = fmul.always(a, r2) ;
    sig_none ; rb0 = fadd.ws.always(r0, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fadd.always(a, r2, ra2, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = fadd.always(b, a, uni, rb1) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.always(r1, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = fadd.always(r2, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r1 = itof.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = fadd.always(b, a, uni, rb0) ; nop = nop(r0, r0) ;
    sig_none ; rb5 = fadd.ws.always(r2, r1) ; nop = nop(r0, r0) ;
    sig_none ; r2 = fadd.always(a, r1, ra1, nop) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.ws.always(r0, a, uni, nop) ; sfu_recip = v8min.always(r2, r2) ;
    sig_none ; r3 = fadd.always(r3, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra6 = fadd.always(r0, r1) ; nop = nop(r0, r0) ;
    sig_none ; r3 = fadd.always(r3, a, uni, nop) ; r0 = fmul.always(r2, r4) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_none ; r3 = fadd.always(r3, r1, nop, rb5) ; vpm = v8min.always(b, b) ;
    sig_small_imm ; r1 = fsub.always(b, r0, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(r3, r3) ; rb4 = fmul.always(r4, r1) ;
    sig_none ; vpm = or.always(a, a, ra6, uni) ; r3 = fmul.always(r3, b) ;
    sig_none ; vpm = or.always(r2, r2, uni, rb5) ; r1 = fmul.always(b, a) ;
    sig_none ; nop = nop(r0, r0, nop, rb4) ; r2 = fmul.always(r1, b) ;
    sig_none ; ra0._16a = ftoi.always(r2, r2, nop, rb4) ; r0 = fmul.always(r3, b) ;
    sig_none ; ra0._16b = ftoi.always(r0, r0, ra6, uni) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb4) ; r3 = fmul.always(r2, b) ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = fadd.always(r3, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(b, b, nop, rb4) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static CS_ASM: ShaderNode = ShaderNode::new(&CS_ASM_CODE);

const VS_ASM_CODE: [u64; 49] = qpu! {
    sig_load_imm ; vr_setup = load32.always(0x00801a00) ; nop = load32.always() ;
    sig_none ; rb4 = or.ws.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra5 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, uni, rb4) ; r1 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra5, uni) ; r0 = fmul.always(b, a) ;
    sig_none ; ra7 = fadd.always(r1, r0, uni, rb4) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra5, uni) ; r1 = fmul.always(b, a) ;
    sig_none ; rb5 = fadd.ws.always(r2, r1, uni, rb4) ; r3 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra5, uni) ; r2 = fmul.always(b, a) ;
    sig_none ; ra4 = fadd.always(r3, r2, uni, rb4) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra5, uni) ; r3 = fmul.always(b, a) ;
    sig_none ; rb3 = fadd.ws.always(r0, r3, vpm_read, nop) ; r3 = v8min.always(a, a) ;
    sig_none ; nop = nop(r0, r0, uni, nop) ; r0 = fmul.always(a, r3) ;
    sig_none ; ra3 = fadd.always(a, r0, ra7, uni) ; r1 = fmul.always(b, r3) ;
    sig_none ; r1 = fadd.always(b, r1, uni, rb5) ; r2 = fmul.always(a, r3) ;
    sig_none ; r0 = fadd.always(a, r2, ra4, uni) ; r3 = fmul.always(b, r3) ;
    sig_none ; r2 = fadd.always(b, r3, nop, rb3) ; nop = nop(r0, r0) ;
    sig_none ; r3 = fadd.always(r1, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb0 = fadd.ws.always(a, b, ra3, uni) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.always(r0, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = fadd.always(r3, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fadd.always(r2, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb1 = fadd.ws.always(r0, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r3 = itof.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = fadd.always(b, a, uni, rb0) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.always(r2, r3) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fadd.ws.always(r1, a, uni, nop) ; sfu_recip = v8min.always(r0, r0) ;
    sig_none ; ra6 = fadd.always(r1, r3) ; nop = nop(r0, r0) ;
    sig_none ; ra2 = fadd.always(a, r3, ra1, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb2 = fadd.ws.always(b, r3, nop, rb1) ; r1 = fmul.always(r0, r4) ;
    sig_small_imm ; r2 = fsub.always(b, r1, nop, _2_1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; rb6 = fmul.always(r4, r2) ;
    sig_none ; nop = nop(r0, r0, ra2, uni) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, nop, rb6) ; r3 = fmul.always(r2, b) ;
    sig_none ; ra0._16a = ftoi.always(r3, r3, uni, rb2) ; r0 = fmul.always(b, a) ;
    sig_none ; nop = nop(r0, r0, nop, rb6) ; r1 = fmul.always(r0, b) ;
    sig_none ; ra0._16b = ftoi.always(r1, r1, ra6, uni) ; r3 = fmul.always(a, b) ;
    sig_load_imm ; vw_setup = load32.ws.always(0x00001a00) ; nop = load32.always() ;
    sig_none ; vpm = or.always(a, a, ra0, rb6) ; r0 = fmul.always(r3, b) ;
    sig_none ; vpm = fadd.always(r0, b, vpm_read, uni) ; r0 = v8min.always(a, a) ;
    sig_none ; r1 = or.always(a, a, vpm_read, rb6) ; vpm = v8min.always(b, b) ;
    sig_none ; r2 = or.always(a, a, vpm_read, nop) ; vpm = v8min.always(r0, r0) ;
    sig_none ; r3 = or.always(a, a, vpm_read, nop) ; vpm = v8min.always(r1, r1) ;
    sig_none ; r0 = or.always(a, a, vpm_read, nop) ; vpm = v8min.always(r2, r2) ;
    sig_none ; vpm = or.always(r3, r3) ; nop = nop(r0, r0) ;
    sig_none ; vpm = or.always(r0, r0) ; nop = nop(r0, r0) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

const FS_ASM_CODE: [u64; 22] = qpu! {
    sig_none ; nop = nop(r0, r0, pay_w, vary) ; r0 = fmul.always(b, a) ;
    sig_none ; ra0 = fadd.always(r0, r5, pay_w, vary) ; r1 = fmul.always(b, a) ;
    sig_none ; ra1 = fadd.always(r1, r5, pay_w, vary) ; r2 = fmul.always(b, a) ;
    sig_none ; rb0 = fadd.ws.always(r2, r5, pay_w, vary) ; r3 = fmul.always(b, a) ;
    sig_none ; r0 = fadd.always(r3, r5, pay_w, vary) ; r1 = fmul.always(b, a) ;
    sig_none ; rb1 = fadd.ws.always(a, r0, ra0, nop) ; nop = nop(r0, r0) ;
    sig_none ; r2 = fadd.always(r1, r5) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fadd.always(b, a, uni, rb0) ; nop = nop(r0, r0) ;
    sig_none ; r3 = fadd.always(a, r2, ra1, nop) ; nop = nop(r0, r0) ;
    sig_none ; r1 = fadd.always(r1, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.pm.always(r3, a, uni, nop) ; r1._8a = v8min.always(r1, r1) ;
    sig_none ; r3 = fadd.always(b, a, uni, rb1) ; nop = nop(r0, r0) ;
    sig_none ; r0 = fadd.always(r0, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_small_imm ; r2 = fadd.always(b, a, uni, _1_1) ; nop = nop(r0, r0) ;
    sig_none ; r3 = fadd.pm.always(r3, a, uni, nop) ; r1._8b = v8min.always(r0, r0) ;
    sig_none ; r2 = fadd.pm.always(r2, a, uni, nop) ; r1._8c = v8min.always(r3, r3) ;
    sig_none ; nop = nop.pm(r0, r0) ; r1._8d = v8min.always(r2, r2) ;
    sig_color_load ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop.ws(r0, r0) ; tlb_color_all = v8adds.always(r1, r4) ;
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_unlock_score ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static FS_ASM: ShaderNode = ShaderNode::new(&FS_ASM_CODE);

pub fn bind(
    encoder: &mut CommandEncoder,
    cs_vbo: &Buffer,
    vs_vbo: &Buffer,
    uni_xf: &glam::Mat4,
    frag_vec: &glam::Vec4,
    int_uni: i32,
    frag_vec2: &glam::Vec4,
) {
    encoder.bind_shader(
        true,
        5,
        *FS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        &[
            ShaderAttribute {
                buffer: cs_vbo,
                record: AttributeRecord {
                    address: 0,
                    number_of_bytes_minus_1: 11,
                    stride: 12,
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
                    number_of_bytes_minus_1: 11,
                    stride: 32,
                    vertex_shader_vpm_offset: 0,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: true,
                cs: false,
            },
            ShaderAttribute {
                buffer: vs_vbo,
                record: AttributeRecord {
                    address: 12,
                    number_of_bytes_minus_1: 11,
                    stride: 32,
                    vertex_shader_vpm_offset: 12,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: true,
                cs: false,
            },
            ShaderAttribute {
                buffer: vs_vbo,
                record: AttributeRecord {
                    address: 24,
                    number_of_bytes_minus_1: 7,
                    stride: 32,
                    vertex_shader_vpm_offset: 24,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: true,
                cs: false,
            },
        ],
        &[
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[2])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec2[2])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[1])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[0])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec2[1])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[3])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec2[0])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec2[3])),
        ],
        &[
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[1])),
            ShaderUniform::Constant(int_uni as _),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[3])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[2])),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
        &[
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(1)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(0)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(2)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[3])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[0])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[2])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[0])),
            ShaderUniform::Constant(int_uni as _),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[3])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[2])),
            ShaderUniform::Constant(qpu::transmute_f32(uni_xf.col(3)[1])),
            ShaderUniform::Constant(qpu::transmute_f32(frag_vec[1])),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_y_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_x_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_scale())),
            ShaderUniform::Constant(qpu::transmute_f32(encoder.vp_z_offset())),
        ],
    );
}
