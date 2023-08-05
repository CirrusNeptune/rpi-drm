use rpi_drm::{Buffer, CommandEncoder, TextureUniform};
use std::io::Read;
use std::sync::OnceLock;
use vc4_drm::cl::{
    CompareFunction, IndexType, PrimitiveMode, TextureConfigUniform, TextureDataType,
};
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
    static CARD: OnceLock<Model> = OnceLock::new();
    CARD.get_or_init(|| Model::open())
}

pub fn get_texture() -> &'static TextureUniform {
    static TEX: OnceLock<TextureUniform> = OnceLock::new();
    TEX.get_or_init(|| {
        use vc4_drm::image::{Translator, TranslatorTrait};

        let decoder =
            png::Decoder::new(std::fs::File::open("/home/citrus/citrus_normals.png").unwrap());
        let mut reader = decoder.read_info().unwrap();
        let size = reader.info().size();
        assert_eq!(reader.info().bit_depth, png::BitDepth::Eight);
        assert_eq!(reader.info().color_type, png::ColorType::Rgba);

        let (translator, alloc_size) =
            Translator::new_with_alloc_size(UVec2::new(size.0, size.1), 32);
        let bo = Buffer::new(alloc_size);
        {
            let mut mapping = bo.mmap();
            for y in (0..size.1).rev() {
                let row = reader.next_row().unwrap().unwrap();
                for x in 0..size.0 {
                    let xs = x as usize;
                    let offset = translator
                        .coordinate_to_tile_address(UVec2::new(x, y))
                        .offset as usize;
                    mapping.as_mut()[offset] = row.data()[xs * 4 + 2];
                    mapping.as_mut()[offset + 1] = row.data()[xs * 4 + 1];
                    mapping.as_mut()[offset + 2] = row.data()[xs * 4 + 0];
                    mapping.as_mut()[offset + 3] = row.data()[xs * 4 + 3];
                }
            }
        }

        TextureUniform {
            buffer: bo,
            config: TextureConfigUniform {
                base_address: 0,
                cache_swizzle: 0,
                cube_map: false,
                flip_y: false,
                data_type: TextureDataType::RGBA8888,
                num_mips: 1,
                height: size.1 as _,
                etc_flip: false,
                width: size.0 as _,
                mag_filt: Default::default(),
                min_filt: Default::default(),
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
