use rpi_drm::{Buffer, CommandEncoder, TextureUniform};
use std::fs;
use std::io::{Read, BufReader};
use std::sync::OnceLock;
use flate2::read::ZlibDecoder;
use vc4_drm::cl::{CompareFunction, IndexType, PrimitiveMode, TextureConfigUniform, TextureDataType, TextureMinFilterType};
use vc4_drm::glam::{Mat4, UVec2};

pub struct Model {
    cs_vbo: Buffer,
    vs_vbo: Buffer,
    ibo: Buffer,
    num_indices: u32,
    num_vertices: u32,
}

impl Model {
    fn open() -> Self {
        let mut file = BufReader::new(fs::File::open("resources/citrus_assets_geo_node.cit").unwrap());
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
    static CARD: OnceLock<Model> = OnceLock::new();
    CARD.get_or_init(|| Model::open())
}

pub fn get_texture() -> &'static TextureUniform {
    static TEX: OnceLock<TextureUniform> = OnceLock::new();
    TEX.get_or_init(|| {
        use vc4_drm::vc4_image_addr::{Translator, TranslatorTrait};
        use flate2::read::ZlibDecoder;

        let mut ctx_f = fs::File::open("resources/generated/citrus_normals.ctx").unwrap();

        let mut header_data = [0_u8; 16];
        ctx_f.read(&mut header_data).unwrap();
        assert_eq!(
            u32::from_le_bytes(header_data[0..4].try_into().unwrap()),
            0x005072C2_u32
        );
        let total_size = u32::from_le_bytes(header_data[4..8].try_into().unwrap());
        let num_mips = u16::from_le_bytes(header_data[8..10].try_into().unwrap());
        let mip0_page_offset = u16::from_le_bytes(header_data[10..12].try_into().unwrap());
        let width = u16::from_le_bytes(header_data[12..14].try_into().unwrap());
        let height = u16::from_le_bytes(header_data[14..16].try_into().unwrap());

        let bo = Buffer::new(total_size);
        {
            let mut mapping = bo.mmap();
            let mut d = ZlibDecoder::new(ctx_f);
            d.read_exact(mapping.as_mut()).unwrap();
        }

        TextureUniform {
            buffer: bo,
            config: TextureConfigUniform {
                base_address: mip0_page_offset as _,
                cache_swizzle: 0,
                cube_map: false,
                flip_y: false,
                data_type: TextureDataType::RGBA8888,
                num_mips: num_mips as _,
                height,
                etc_flip: false,
                width,
                mag_filt: Default::default(),
                min_filt: TextureMinFilterType::LinearMipLinear,
                wrap_t: Default::default(),
                wrap_s: Default::default(),
            },
        }
    })
}

pub fn draw(encoder: &mut CommandEncoder, xf: &Mat4) {
    let model = get_model();
    let texture = get_texture();
    super::generated::test_model::bind(encoder, &model.cs_vbo, &model.vs_vbo, xf, texture);
    encoder.set_depth_test(true, CompareFunction::LEqual, true);
    encoder.set_cull_test(true, false);
    encoder.draw_indexed_primitives(
        &model.ibo,
        IndexType::_16bit,
        PrimitiveMode::Triangles,
        0,
        model.num_indices,
        model.num_vertices,
    );
}
