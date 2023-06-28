use rpi_drm::vc4_image::*;
use glam::UVec2;

fn assert_translate(translator: &Translator, x: u32, y: u32, expected_offset: u32) {
    let vec = UVec2::new(x, y);
    let address = translator.coordinate_to_tile_address(vec);
    assert_eq!(address.offset, expected_offset);
    assert_eq!(translator.tile_address_to_coordinate(address), vec);
}

#[test]
fn vc4_image_32bpp_utiles() {
    let translator = Translator::new((1024, 1024).into(), 32);

    assert_translate(&translator, 0, 0, 0);
    assert_translate(&translator, 3, 0, 3 * 4);
    assert_translate(&translator, 0, 3, 12 * 4);
    assert_translate(&translator, 3, 3, 15 * 4);
}

#[test]
fn vc4_image_32bpp_subtiles() {
    let translator = Translator::new((1024, 1024).into(), 32);

    assert_translate(&translator, 4, 0, 64);
    assert_translate(&translator, 12, 0, 3 * 64);
    assert_translate(&translator, 0, 12, 12 * 64);
    assert_translate(&translator, 12, 12, 15 * 64);
}

#[test]
fn vc4_image_32bpp_tiles_even() {
    let translator = Translator::new((1024, 1024).into(), 32);

    assert_translate(&translator, 0, 16, 1024);
    assert_translate(&translator, 16, 16, 2 * 1024);
    assert_translate(&translator, 16, 0, 3 * 1024);
}

#[test]
fn vc4_image_32bpp_tiles_odd() {
    let translator = Translator::new((1024, 1024).into(), 32);

    assert_translate(&translator, 16, 32 + 16, 63 * 4096);
    assert_translate(&translator, 16, 32, 63 * 4096 + 1024);
    assert_translate(&translator, 0, 32, 63 * 4096 + 2 * 1024);
    assert_translate(&translator, 0, 32 + 16, 63 * 4096 + 3 * 1024);

    assert_translate(&translator, 992 + 16, 32 + 16, 32 * 4096);
    assert_translate(&translator, 992 + 16, 32, 32 * 4096 + 1024);
    assert_translate(&translator, 992, 32, 32 * 4096 + 2 * 1024);
    assert_translate(&translator, 992, 32 + 16, 32 * 4096 + 3 * 1024);
}

#[test]
fn vc4_image_32bpp_utiles_lt() {
    let translator = Translator::new((8, 8).into(), 32);

    assert_translate(&translator, 0, 0, 0);
    assert_translate(&translator, 3, 0, 3 * 4);
    assert_translate(&translator, 0, 3, 12 * 4);
    assert_translate(&translator, 3, 3, 15 * 4);

    assert_translate(&translator, 4, 0, 64);
    assert_translate(&translator, 7, 0, 64 + 3 * 4);
    assert_translate(&translator, 0, 7, 2 * 64 + 3 * 16);
    assert_translate(&translator, 7, 7, 3 * 64 + 3 * 16 + 3 * 4);
}
