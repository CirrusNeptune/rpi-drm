use flate2::write::ZlibEncoder;
use smallvec::SmallVec;
use std::fs;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use vc4_image_addr::glam::*;
use vc4_image_addr::{Translator, TranslatorTrait};

fn for_each_file_ext_in_dir<F>(dir: &PathBuf, ext: &str, mut f: F) -> Result<(), String>
where
    F: FnMut(PathBuf, fs::Metadata) -> Result<(), String>,
{
    for path_ent in fs::read_dir(&dir).unwrap() {
        if path_ent.is_err() {
            continue;
        }

        let de = path_ent.unwrap();

        let metadata = de.metadata().unwrap();
        if !metadata.is_file() {
            continue;
        }
        let path = de.path();
        if path.extension().unwrap_or("".as_ref()) != ext {
            continue;
        }

        f(path, metadata)?;
    }

    Ok(())
}

#[derive(Default, Copy, Clone)]
struct MipInfo {
    pub size: UVec2,
    pub offset: u32,
}

fn pack_texture(png_path: &PathBuf, out_path: &PathBuf) -> Result<(), String> {
    let decoder = png::Decoder::new(BufReader::new(fs::File::open(&png_path).unwrap()));

    let mut reader = decoder.read_info().unwrap();
    let size: UVec2 = reader.info().size().into();
    assert!(size.x > 0 && size.y > 0);
    assert_eq!(reader.info().bit_depth, png::BitDepth::Eight);
    assert_eq!(reader.info().color_type, png::ColorType::Rgba);

    let pot_size = UVec2::new(size.x.next_power_of_two(), size.y.next_power_of_two());
    let num_mips = u32::min(size.x.ilog2(), size.y.ilog2()) + 1;
    assert!(num_mips <= 12);

    let mut mip_infos = SmallVec::<[MipInfo; 12]>::new();
    mip_infos.resize(num_mips as _, MipInfo::default());

    let mut total_size = 0_u32;
    for level in (0..num_mips).rev() {
        let mip_info = &mut mip_infos[level as usize];
        let level_size = if level == 0 {
            size
        } else {
            UVec2::max(UVec2::splat(1), pot_size >> level)
        };

        let alloc_size = Translator::alloc_size(level_size, 32);
        mip_info.size = level_size;
        mip_info.offset = total_size;
        total_size += alloc_size;
    }

    let mut tmp_buf = Vec::<u8>::new();
    tmp_buf.resize(total_size as _, 0);
    let mip_info_0 = &mip_infos[0];
    let (mut prev_translator, mut prev_alloc_size) =
        Translator::new_with_alloc_size(mip_info_0.size, 32);

    for y in (0..mip_info_0.size.y).rev() {
        let buf_slice = &mut tmp_buf[mip_info_0.offset as usize..total_size as usize];
        let row = reader.next_row().unwrap().unwrap();
        for x in 0..mip_info_0.size.x {
            let xs = x as usize;
            let offset = prev_translator
                .coordinate_to_tile_address(UVec2::new(x, y))
                .offset as usize;
            buf_slice[offset] = row.data()[xs * 4 + 2];
            buf_slice[offset + 1] = row.data()[xs * 4 + 1];
            buf_slice[offset + 2] = row.data()[xs * 4 + 0];
            buf_slice[offset + 3] = row.data()[xs * 4 + 3];
        }
    }

    for level in 1..mip_infos.len() {
        let mip_info = &mip_infos[level];
        let prev_mip_info = &mip_infos[level - 1];
        let (translator, alloc_size) = Translator::new_with_alloc_size(mip_info.size, 32);

        for y in 0..mip_info.size.y {
            for x in 0..mip_info.size.x {
                let mut sum_pixel = [0_u16; 4];
                {
                    let buf_slice = &tmp_buf[prev_mip_info.offset as usize
                        ..(prev_mip_info.offset + prev_alloc_size) as usize];
                    const DELTAS: [UVec2; 4] = [
                        UVec2::new(0, 0),
                        UVec2::new(1, 0),
                        UVec2::new(0, 1),
                        UVec2::new(1, 1),
                    ];
                    for delta in DELTAS {
                        let offset = prev_translator
                            .coordinate_to_tile_address(UVec2::new(x, y) * 2 + delta)
                            .offset as usize;
                        for i in 0..4 {
                            sum_pixel[i] += buf_slice[offset + i] as u16;
                        }
                    }
                }

                let buf_slice =
                    &mut tmp_buf[mip_info.offset as usize..(mip_info.offset + alloc_size) as usize];
                let offset = translator
                    .coordinate_to_tile_address(UVec2::new(x, y))
                    .offset as usize;
                for i in 0..4 {
                    buf_slice[offset + i] = (sum_pixel[i] >> 2) as _;
                }
            }
        }

        prev_translator = translator;
        prev_alloc_size = alloc_size;
    }

    let mip0_page_offset = (mip_info_0.offset + 4095) / 4096;
    let mip_padding = mip0_page_offset * 4096 - mip_info_0.offset;

    let mut header_data = [0_u8; 16];
    header_data[0..4].copy_from_slice(0x005072C2_u32.to_le_bytes().as_slice());
    header_data[4..8].copy_from_slice(total_size.to_le_bytes().as_slice());
    header_data[8..10].copy_from_slice((num_mips as u16).to_le_bytes().as_slice());
    header_data[10..12].copy_from_slice((mip0_page_offset as u16).to_le_bytes().as_slice());
    header_data[12..14].copy_from_slice(((mip_info_0.size.x) as u16).to_le_bytes().as_slice());
    header_data[14..16].copy_from_slice(((mip_info_0.size.y) as u16).to_le_bytes().as_slice());

    let mut out_f = fs::File::create(out_path).unwrap();
    out_f.write(header_data.as_slice()).unwrap();
    let mut c = ZlibEncoder::new(out_f, flate2::Compression::best());
    for _ in 0..mip_padding {
        c.write_all(&[0]).unwrap();
    }
    c.write_all(tmp_buf.as_slice()).unwrap();
    c.finish().unwrap();

    Ok(())
}

fn prune_generated_dir(generated_dir: &PathBuf, resources_dir: &PathBuf) -> Result<bool, String> {
    let mut pruned_dir = false;
    for_each_file_ext_in_dir(&generated_dir, "ctx", |ctx_path, _| {
        let stem = ctx_path.file_stem().unwrap();
        if !resources_dir.join(stem).with_extension("png").exists() {
            fs::remove_file(ctx_path).ok();
            pruned_dir = true;
        }
        Ok(())
    })?;
    Ok(pruned_dir)
}

pub fn pack_textures(resources_dir: &PathBuf) -> Result<(), String> {
    let generated_dir = resources_dir.join("generated");
    fs::create_dir_all(&generated_dir).unwrap();

    prune_generated_dir(&generated_dir, &resources_dir)?;
    for_each_file_ext_in_dir(resources_dir, "png", |png_path, png_metadata| {
        let out_path = generated_dir
            .join(png_path.file_name().unwrap())
            .with_extension("ctx");
        let out_metadata = out_path.metadata();
        if out_metadata.is_err()
            || out_metadata.unwrap().modified().unwrap() < png_metadata.modified().unwrap()
        {
            pack_texture(&png_path, &out_path)?;
        }
        Ok(())
    })
}
