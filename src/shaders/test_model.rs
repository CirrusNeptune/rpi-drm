use super::ShaderNode;
use rpi_drm::{Buffer, CommandEncoder, ShaderAttribute, ShaderUniform, TextureUniform};
use std::io::Read;
use std::sync;
use vc4_drm::cl::{AttributeRecord, CompareFunction, IndexType, PrimitiveMode, TextureConfigUniform, TextureDataType, TextureMagFilterType, TextureMinFilterType, TextureWrapType};
use vc4_drm::glam::UVec2;
use vc4_drm::{glam, qpu};

// Ys|Xs, Zs, 1/Wc, Varyings...

// Ys|Xs: vpm = ??
// Zs: vpm = ??
// 1/Wc: vpm = ??
// Uc:
// Vc:

// r0 = r1X * X
// r1 = r1Y * Y
// r0 = r0 + r1
// r1 = r1Z * Z
// r0 = r0 + r1
// r0 = r0 + uni

// r1 = r1X * X
// r2 = r1Y * Y
// r1 = r1 + r2
// r2 = r1Z * Z
// r1 = r1 + r2

// r2 = r1X * X
// r3 = r1Y * Y
// r2 = r2 + r3
// r3 = r1Z * Z
// r2 = r2 + r3

const VS_ASM_CODE: [u64; 62] = qpu! {
    sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(8, 1, 0)) ; nop = load32.always() ;

    // Load XYZ into regfile A
    sig_none ; ra0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;

    // TODO: Can this be merged with above?
    // Load XF row 0 into regfile B
    sig_none ; rb0 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb1 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb2 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb3 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 1 into regfile B
    sig_none ; rb4 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb5 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb6 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb7 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 2 into regfile B
    sig_none ; rb8 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb9 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb10 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb11 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 3 into regfile B
    sig_none ; rb12 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb13 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb14 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb15 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Dot product row 0
    sig_none ; nop = nop(r0, r0, ra0, rb0) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb1) ; r1 = fmul.always(a, b) ;
    sig_none ; r0 = fadd.always(r0, r1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb2) ; r1 = fmul.always(a, b) ;
    sig_none ; r0 = fadd.always(r0, r1) ; nop = nop(r0, r0) ;
    // X Translation
    sig_none ; ra0 = fadd.always(r0, b, nop, rb3) ; nop = nop(r0, r0) ;

    // Dot product row 1
    sig_none ; nop = nop(r0, r0, ra0, rb4) ; r1 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb5) ; r2 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r2) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb6) ; r2 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r2) ; nop = nop(r0, r0) ;
    // Y Translation
    sig_none ; r1 = fadd.always(r1, b, nop, rb7) ; nop = nop(r0, r0) ;

    // Dot product row 2
    sig_none ; nop = nop(r0, r0, ra0, rb8) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb9) ; r3 = fmul.always(a, b) ;
    sig_none ; r2 = fadd.always(r2, r3) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb10) ; r3 = fmul.always(a, b) ;
    sig_none ; r2 = fadd.always(r2, r3) ; nop = nop(r0, r0) ;
    // Z Translation
    sig_none ; r2 = fadd.always(r2, b, nop, rb11) ; nop = nop(r0, r0) ;

    // Dot product row 3
    sig_none ; nop = nop(r0, r0, ra0, rb12) ; r3 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb13) ; r0 = fmul.always(a, b) ;
    sig_none ; r3 = fadd.always(r3, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb14) ; r0 = fmul.always(a, b) ;
    sig_none ; r3 = fadd.always(r3, r0) ; nop = nop(r0, r0) ;
    // W Translation
    sig_none ; r3 = fadd.always(r3, b, nop, pay_z) ; nop = nop(r0, r0) ;
    sig_none ; sfu_recip = or.always(r3, r3) ; nop = nop(r0, r0) ;

    // Restore X from regfile
    sig_none ; r0 = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;

    //sig_none ; r0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    // Read Y; Mul X * WindowScaleX
    // r1 = vpm; r0 = r0 * uni
    sig_none ; /*r1 = or.always(a, a, vpm_read, uni)*/ nop = nop(r0, r0, nop, uni) ; r0 = fmul.always(r0, b) ;
    // Read Z; Mul Y * WindowScaleY
    // r2 = vpm; r1 = r1 * uni
    sig_none ; /*r2 = or.always(a, a, vpm_read, uni)*/ nop = nop(r0, r0, nop, uni) ; r1 = fmul.always(r1, b) ;
    // Convert Mul X * WindowScaleX to int
    // ; ra0._16a = r0
    sig_none ; ra0._16a = ftoi.always(r0, r0) ; nop = nop(r0, r0) ;
    // Convert Mul Y * WindowScaleY to int
    // ; ra0._16b = r1
    sig_none ; ra0._16b = ftoi.always(r1, r1) ; nop = nop(r0, r0) ;
    // Write Ys|Xs = Y|X scaled
    // Ys|Xs: vpm = ra0
    sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    // Write Zs = Z
    // Zs: vpm = r2
    sig_none ; vpm = or.always(r2, r2) ; nop = nop(r0, r0) ;
    // Write 1/Wc = 1.0; Read Nx
    // 1/Wc: vpm = r4; r0 = vpm
    sig_none ; vpm = or.always(r4, r4, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    // Write Varying0; Read Ny
    // Varying0: vpm = r0; r0 = vpm
    sig_none ; vpm = or.always(r0, r0, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    // Write Varying1; Read Nz
    // Varying1: vpm = r0; r0 = vpm
    sig_none ; vpm = or.always(r0, r0, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    // Write Varying2; Read U
    // Varying2: vpm = r0; r0 = vpm
    sig_none ; vpm = or.always(r0, r0, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    // Write Varying3; Read V
    // Varying3: vpm = r0; r0 = vpm
    sig_none ; vpm = or.always(r0, r0, vpm_read, nop) ; r0 = v8min.always(a, a) ;
    // Write Varying4
    // Varying4: vpm = r0
    sig_none ; vpm = or.always(r0, r0) ; nop = nop(r0, r0) ;
    // END
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static VS_ASM: ShaderNode = ShaderNode::new(&VS_ASM_CODE);

// Xc, Yc, Zc, Wc, Ys|Xs, Zs, 1/Wc

// Xc: vpm = ??
// Yc: vpm = ??
// Zc: vpm = ??
// Wc: vpm = ??
// Ys|Xs: vpm = ??
// Zs: vpm = ??
// 1/Wc: vpm = ??

const CS_ASM_CODE: [u64; 59] = qpu! {
    sig_load_imm ; vr_setup = load32.always(qpu::vpm_block_read_horizontal_32(3, 1, 0)) ; nop = load32.always() ;

    // Load XYZ into regfile A
    sig_none ; ra0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra1 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    sig_none ; ra2 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;

    // TODO: Can this be merged with above?
    // Load XF row 0 into regfile B
    sig_none ; rb0 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb1 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb2 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb3 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 1 into regfile B
    sig_none ; rb4 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb5 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb6 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb7 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 2 into regfile B
    sig_none ; rb8 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb9 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb10 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb11 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Load XF row 3 into regfile B
    sig_none ; rb12 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb13 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb14 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;
    sig_none ; rb15 = or.ws.always(a, a, uni, nop) ; nop = nop(r0, r0) ;

    // Dot product row 0
    sig_none ; nop = nop(r0, r0, ra0, rb0) ; r0 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb1) ; r1 = fmul.always(a, b) ;
    sig_none ; r0 = fadd.always(r0, r1) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb2) ; r1 = fmul.always(a, b) ;
    sig_none ; r0 = fadd.always(r0, r1) ; nop = nop(r0, r0) ;
    // X Translation
    sig_none ; ra0 = fadd.always(r0, b, nop, rb3) ; nop = nop(r0, r0) ;

    // Dot product row 1
    sig_none ; nop = nop(r0, r0, ra0, rb4) ; r1 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb5) ; r2 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r2) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb6) ; r2 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.always(r1, r2) ; nop = nop(r0, r0) ;
    // Y Translation
    sig_none ; r1 = fadd.always(r1, b, nop, rb7) ; nop = nop(r0, r0) ;

    // Dot product row 2
    sig_none ; nop = nop(r0, r0, ra0, rb8) ; r2 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb9) ; r3 = fmul.always(a, b) ;
    sig_none ; r2 = fadd.always(r2, r3) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb10) ; r3 = fmul.always(a, b) ;
    sig_none ; r2 = fadd.always(r2, r3) ; nop = nop(r0, r0) ;
    // Z Translation
    sig_none ; r2 = fadd.always(r2, b, nop, rb11) ; nop = nop(r0, r0) ;

    // Dot product row 3
    sig_none ; nop = nop(r0, r0, ra0, rb12) ; r3 = fmul.always(a, b) ;
    sig_none ; nop = nop(r0, r0, ra1, rb13) ; r0 = fmul.always(a, b) ;
    sig_none ; r3 = fadd.always(r3, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0, ra2, rb14) ; r0 = fmul.always(a, b) ;
    sig_none ; r3 = fadd.always(r3, r0) ; nop = nop(r0, r0) ;
    // W Translation
    sig_none ; r3 = fadd.always(r3, b, nop, pay_z) ; nop = nop(r0, r0) ;

    // Restore X from regfile
    sig_none ; r0 = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;

    //sig_none ; r0 = or.always(a, a, vpm_read, nop) ; nop = nop(r0, r0) ;
    // Write Xc = X; Read Y
    // Xc: vpm = r0; r1 = vpm
    sig_load_imm ; vw_setup = load32.always.ws(qpu::vpm_block_write_horizontal_32(1, 0)) ; nop = load32.always() ;
    sig_none ; vpm = or.always(r0, r0) ; /*r1 = v8min.always(a, a)*/ nop = nop(r0, r0) ;
    // Write Yc = Y; Read Z
    // Yc: vpm = r1; r2 = vpm
    sig_none ; vpm = or.always(r1, r1) ; /*r2 = v8min.always(a, a)*/ nop = nop(r0, r0) ;
    // Write Zc = Z; Mul X * WindowScaleX
    // Zc: vpm = r2; r0 = r0 * uni
    sig_none ; vpm = or.always(r2, r2, uni, nop) ; r0 = fmul.always(r0, a) ;
    // Convert Mul X * WindowScaleX to int; Mul Y * WindowScaleY
    // ra0._16a = r0; r1 = r1 * uni
    sig_none ; ra0._16a = ftoi.always(r0, r0, uni, nop) ; r1 = fmul.always(r1, a) ;
    // Convert Mul Y * WindowScaleY to int
    // ; ra0._16b = r1
    sig_none ; ra0._16b = ftoi.always(r1, r1) ; nop = nop(r0, r0) ;
    // Write Wc = W
    // Wc: vpm = r3
    sig_none ; vpm = or.always(r3, r3) ; nop = nop(r0, r0) ;
    sig_none ; sfu_recip = or.always(r3, r3) ; nop = nop(r0, r0) ;
    // Write Ys|Xs = Y|X scaled
    // Ys|Xs: vpm = ra0
    sig_none ; vpm = or.always(a, a, ra0, nop) ; nop = nop(r0, r0) ;
    // Write Zs = Z
    // Zs: vpm = r2
    sig_none ; vpm = or.always(r2, r2) ; nop = nop(r0, r0) ;
    // Write 1/Wc = 1.0
    // 1/Wc: vpm = imm(1.0)
    sig_none ; vpm = or.always(r4, r4) ; nop = nop(r0, r0) ;
    // END
    sig_end ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(r0, r0) ; nop = nop(r0, r0) ;
};
pub static CS_ASM: ShaderNode = ShaderNode::new(&CS_ASM_CODE);

const FS_ASM_CODE: [u64; 21] = qpu! {
    sig_none ; nop = nop(a, a, vary, nop) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(a, a, vary, nop) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(a, a, vary, nop) ; nop = nop(r0, r0) ;
    sig_none ; nop = nop(a, a, pay_w, vary) ; r0 = fmul.always(a, b) ;
    sig_none ; r0 = fadd.pm.always(r0, r5, pay_w, vary) ; r1 = fmul.always(a, b) ;
    sig_none ; r1 = fadd.pm.always(r1, r5) ; nop = nop(r0, r0) ;
    //write texture addresses (x, y)
    //writing tmu0_s signals that all coordinates are written
    sig_none ; tmu0_t = or.always(r1, r1) ; nop = nop(r0, r0) ;
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
pub static FS_ASM: ShaderNode = ShaderNode::new(&FS_ASM_CODE);

pub struct Model {
    cs_vbo: Buffer,
    vs_vbo: Buffer,
    ibo: Buffer,
    num_indices: u32,
    num_vertices: u32,
}

impl Model {
    fn open() -> Self {
        let mut file = std::fs::File::open("/home/citrus/citrus_assets_geo_node.cit").unwrap();
        let mut head_data: [u8; 12] = [0; 12];
        file.read(&mut head_data).unwrap();
        assert_eq!(
            u32::from_le_bytes(head_data[0..4].try_into().unwrap()),
            0x005072C1
        );
        let num_vertices = u32::from_le_bytes(head_data[4..8].try_into().unwrap());
        let num_indices = u32::from_le_bytes(head_data[8..12].try_into().unwrap());

        let mut read_buf = |size: u32| {
            let buffer = Buffer::new(size);
            {
                let mut mapping = buffer.mmap();
                file.read(mapping.as_mut()[0..size as usize].as_mut())
                    .unwrap();
            }
            buffer
        };

        let cs_vbo = read_buf(num_vertices * 12);
        let vs_vbo = read_buf(num_vertices * 32);
        let ibo = read_buf(num_indices * 2);

        Self {
            cs_vbo,
            vs_vbo,
            ibo,
            num_indices,
            num_vertices,
        }
    }
}

pub fn get_model() -> &'static Model {
    use sync::OnceLock;
    static CARD: OnceLock<Model> = OnceLock::new();
    CARD.get_or_init(|| Model::open())
}

pub struct Texture {
    bo: Buffer,
    size: (u16, u16),
}

impl Texture {
    fn open() -> Self {
        use vc4_drm::image::{Translator, TranslatorTrait};

        let decoder = png::Decoder::new(std::fs::File::open("/home/citrus/citrus_normals.png").unwrap());
        let mut reader = decoder.read_info().unwrap();
        let size = reader.info().size();
        assert_eq!(reader.info().bit_depth, png::BitDepth::Eight);
        assert_eq!(reader.info().color_type, png::ColorType::Rgba);

        let (translator, alloc_size) = Translator::new_with_alloc_size(UVec2::new(size.0, size.1), 32);
        let bo = Buffer::new(alloc_size);
        {
            let mut mapping = bo.mmap();
            for y in (0..size.1).rev() {
                let row = reader.next_row().unwrap().unwrap();
                for x in 0..size.0 {
                    let xs = x as usize;
                    let offset = translator.coordinate_to_tile_address(UVec2::new(x, y)).offset as usize;
                    mapping.as_mut()[offset] = row.data()[xs * 4 + 2];
                    mapping.as_mut()[offset + 1] = row.data()[xs * 4 + 1];
                    mapping.as_mut()[offset + 2] = row.data()[xs * 4 + 0];
                    mapping.as_mut()[offset + 3] = row.data()[xs * 4 + 3];
                }
            }
        }

        Self {
            bo,
            size: (size.0 as u16, size.1 as u16),
        }
    }
}

pub fn get_texture() -> &'static Texture {
    use sync::OnceLock;
    static CARD: OnceLock<Texture> = OnceLock::new();
    CARD.get_or_init(|| Texture::open())
}

pub fn draw(encoder: &mut CommandEncoder, xf: glam::Mat4) {
    let model = get_model();
    let texture = get_texture();
    let cs_vs_uniforms = [
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(0).x)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(0).y)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(0).z)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(0).w)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(1).x)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(1).y)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(1).z)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(1).w)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(2).x)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(2).y)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(2).z)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(2).w)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(3).x)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(3).y)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(3).z)),
        ShaderUniform::Constant(qpu::transmute_f32(xf.row(3).w)),
        ShaderUniform::Constant(qpu::transmute_f32(
            (encoder.window_size().0 * 16 / 2) as f32,
        )),
        ShaderUniform::Constant(qpu::transmute_f32(
            (encoder.window_size().1 * 16 / 2) as f32,
        )),
    ];
    encoder.bind_shader(
        false,
        5,
        *FS_ASM.handle.get().unwrap(),
        *VS_ASM.handle.get().unwrap(),
        *CS_ASM.handle.get().unwrap(),
        &[
            ShaderAttribute {
                buffer: model.vs_vbo.clone(),
                record: AttributeRecord {
                    address: 0,
                    number_of_bytes_minus_1: 31,
                    stride: 32,
                    vertex_shader_vpm_offset: 0,
                    coordinate_shader_vpm_offset: 0,
                },
                vs: true,
                cs: false,
            },
            ShaderAttribute {
                buffer: model.cs_vbo.clone(),
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
        ],
        &[ShaderUniform::Texture(TextureUniform {
            buffer: texture.bo.clone(),
            config: TextureConfigUniform {
                base_address: 0,
                cache_swizzle: 0,
                cube_map: false,
                flip_y: false,
                data_type: TextureDataType::RGBA8888,
                num_mips: 1,
                height: texture.size.1,
                etc_flip: false,
                width: texture.size.0,
                mag_filt: TextureMagFilterType::Linear,
                min_filt: TextureMinFilterType::Linear,
                wrap_t: TextureWrapType::Repeat,
                wrap_s: TextureWrapType::Repeat,
            },
        })],
        &cs_vs_uniforms,
        &cs_vs_uniforms,
    );
    encoder.set_cull_test(true, false);
    encoder.draw_indexed_primitives(
        model.ibo.clone(),
        IndexType::_16bit,
        PrimitiveMode::Triangles,
        0,
        model.num_indices,
        model.num_vertices - 1,
    );
}
