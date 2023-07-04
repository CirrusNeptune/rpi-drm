use super::ShaderNode;
use crate::{CommandRecorder, ShaderAttribute, ShaderUniform, TextureUniform};
use vc4_drm::cl::{
    AttributeRecord, TextureConfigUniform, TextureDataType, TextureMagFilterType,
    TextureMinFilterType, TextureWrapType,
};
use vc4_drm::qpu;

const VS_ASM_CODE: [u64; 14] = qpu! {
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
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

const CS_ASM_CODE: [u64; 18] = qpu! {
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

pub fn bind(
    recorder: &mut CommandRecorder,
    vbo: vc4_drm::card::Buffer,
    tex: vc4_drm::card::Buffer,
) {
    let vs_uniforms = [
        ShaderUniform::Constant(u32::from_le_bytes(1.0_f32.to_le_bytes())),
        ShaderUniform::Constant(u32::from_le_bytes(
            ((recorder.window_size.0 * 16 / 2) as f32).to_le_bytes(),
        )),
        ShaderUniform::Constant(u32::from_le_bytes(
            ((recorder.window_size.1 * 16 / 2) as f32).to_le_bytes(),
        )),
        ShaderUniform::Constant(u32::from_le_bytes(1.0_f32.to_le_bytes())),
    ];
    recorder.bind_shader(
        false,
        *FS_ASM_TEX.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        &[ShaderAttribute {
            handle: vbo,
            record: AttributeRecord {
                address: 0,
                number_of_bytes_minus_1: 7,
                stride: 8,
                vertex_shader_vpm_offset: 0,
                coordinate_shader_vpm_offset: 0,
            },
        }],
        &[ShaderUniform::Texture(TextureUniform {
            handle: tex,
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
        })],
        &vs_uniforms,
        &vs_uniforms,
    )
}
