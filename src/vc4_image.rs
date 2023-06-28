use glam::{UVec2, Vec2Swizzles};
use num::Integer;
use enum_dispatch::enum_dispatch;

fn div_rem(lhs: UVec2, rhs: UVec2) -> (UVec2, UVec2) {
    let (xd, xr) = lhs.x.div_rem(&rhs.x);
    let (yd, yr) = lhs.y.div_rem(&rhs.y);
    ((xd, yd).into(), (xr, yr).into())
}

struct UVec2PowerOfTwoFactor {
    shift: UVec2,
    mask: UVec2,
}

impl UVec2PowerOfTwoFactor {
    pub fn div_rem(&self, coord: UVec2) -> (UVec2, UVec2) {
        let div = coord >> self.shift;
        let rem = coord & self.mask;
        (div, rem)
    }

    pub fn div_rem_x(&self, val: u32) -> (u32, u32) {
        let div = val >> self.shift.x;
        let rem = val & self.mask.x;
        (div, rem)
    }

    pub fn mul_add(&self, mul: UVec2, add: UVec2) -> UVec2 {
        (mul << self.shift) | add
    }

    pub fn mul_add_x(&self, mul: u32, add: u32) -> u32 {
        (mul << self.shift.x) | add
    }

    pub fn round_up_div(&self, val: UVec2) -> UVec2 {
        (val + self.mask - 1) >> self.shift
    }
}

pub struct TileAddress {
    pub offset: u32,
    pub bit: u8,
    pub lt: bool,
}

#[enum_dispatch]
pub trait TranslatorTrait {
    fn coordinate_to_tile_address(&self, coord: UVec2) -> TileAddress;
    fn tile_address_to_coordinate(&self, address: TileAddress) -> UVec2;
}

pub struct TTranslator {
    #[cfg(debug_assertions)]
    image_size: UVec2,
    utile_size: UVec2PowerOfTwoFactor,
    size_in_tile: UVec2,
    bpp_shift: u32,
}

impl TranslatorTrait for TTranslator {
    fn coordinate_to_tile_address(&self, coord: UVec2) -> TileAddress {
        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        let (utile_coord, pixel_in_utile) = self.utile_size.div_rem(coord);
        let (subtile_coord, utile_in_subtile) = div_rem(utile_coord, UVec2::splat(4));
        let (tile_coord, subtile_in_tile) = div_rem(subtile_coord, UVec2::splat(2));

        let tile_index = tile_coord.y * self.size_in_tile.x +
            if (tile_coord.y & 0x1) != 0 { self.size_in_tile.x - tile_coord.x - 1 } else { tile_coord.x };

        let subtile_index = if tile_coord.y & 0x1 != 0 {
            const EVEN_LUT: [u32; 4] = [2, 3, 1, 0];
            EVEN_LUT[((subtile_in_tile.x << 1) | subtile_in_tile.y) as usize]
        } else {
            const ODD_LUT: [u32; 4] = [0, 1, 3, 2];
            ODD_LUT[((subtile_in_tile.x << 1) | subtile_in_tile.y) as usize]
        };

        let utile_index = utile_in_subtile.y * 4 + utile_in_subtile.x;

        let pixel_index = self.utile_size.mul_add_x(pixel_in_utile.y, pixel_in_utile.x);
        let (pixel_byte, pixel_bit) = (pixel_index << self.bpp_shift).div_rem(&8);

        TileAddress {
            offset: tile_index * 4096 + subtile_index * 1024 + utile_index * 64 + pixel_byte,
            bit: pixel_bit as u8,
            lt: false,
        }
    }

    fn tile_address_to_coordinate(&self, address: TileAddress) -> UVec2 {
        let (abs_utile_index, pixel_byte) = address.offset.div_rem(&64);
        let (abs_subtile_index, utile_index) = abs_utile_index.div_rem(&16);
        let (tile_index, subtile_index) = abs_subtile_index.div_rem(&4);

        let tile_row = tile_index / self.size_in_tile.x;

        let subtile_in_tile = if tile_row & 0x1 != 0 {
            const EVEN_LUT: [UVec2; 4] = [UVec2::new(1, 1), UVec2::new(1, 0), UVec2::new(0, 0), UVec2::new(0, 1)];
            EVEN_LUT[subtile_index as usize]
        } else {
            const ODD_LUT: [UVec2; 4] = [UVec2::new(0, 0), UVec2::new(0, 1), UVec2::new(1, 1), UVec2::new(1, 0)];
            ODD_LUT[subtile_index as usize]
        };

        let tile_coord = if tile_row & 0x1 != 0 {
            let tmp = UVec2::from(tile_index.div_rem(&self.size_in_tile.x)).yx();
            UVec2::new(self.size_in_tile.x - tmp.x - 1, tmp.y)
        } else {
            UVec2::from(tile_index.div_rem(&self.size_in_tile.x)).yx()
        };

        let utile_in_subtile = UVec2::from(utile_index.div_rem(&4)).yx();

        let pixel_index = (pixel_byte * 8 + address.bit as u32) >> self.bpp_shift;
        let pixel_in_utile = UVec2::from(self.utile_size.div_rem_x(pixel_index)).yx();

        let subtile_coord = tile_coord * UVec2::splat(2) + subtile_in_tile;
        let utile_coord = subtile_coord * UVec2::splat(4) + utile_in_subtile;
        let coord = self.utile_size.mul_add(utile_coord, pixel_in_utile);

        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        coord
    }
}

pub struct LTTranslator {
    #[cfg(debug_assertions)]
    image_size: UVec2,
    utile_size: UVec2PowerOfTwoFactor,
    size_in_utile: UVec2,
    bpp_shift: u32,
}

impl TranslatorTrait for LTTranslator {
    fn coordinate_to_tile_address(&self, coord: UVec2) -> TileAddress {
        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        let (utile_coord, pixel_in_utile) = self.utile_size.div_rem(coord);

        let utile_index = utile_coord.y * self.size_in_utile.x + utile_coord.x;

        let pixel_index = self.utile_size.mul_add_x(pixel_in_utile.y, pixel_in_utile.x);
        let (pixel_byte, pixel_bit) = (pixel_index << self.bpp_shift).div_rem(&8);

        TileAddress {
            offset: utile_index * 64 + pixel_byte,
            bit: pixel_bit as u8,
            lt: true,
        }
    }

    fn tile_address_to_coordinate(&self, address: TileAddress) -> UVec2 {
        let (utile_index, pixel_byte) = address.offset.div_rem(&64);

        let pixel_index = (pixel_byte * 8 + address.bit as u32) >> self.bpp_shift;
        let pixel_in_utile = UVec2::from(self.utile_size.div_rem_x(pixel_index)).yx();

        let utile_coord = UVec2::from(utile_index.div_rem(&self.size_in_utile.x)).yx();

        let coord = self.utile_size.mul_add(utile_coord, pixel_in_utile);

        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        coord
    }
}

#[enum_dispatch(TranslatorTrait)]
pub enum Translator {
    TTranslator,
    LTTranslator,
}

impl Translator {
    pub fn new(image_size: UVec2, bpp: u32) -> Self {
        let (utile_size, bpp_shift) = match bpp {
            64 => (UVec2PowerOfTwoFactor{shift: UVec2::new(1, 2), mask: UVec2::new(0b1, 0b11)}, 6),
            32 => (UVec2PowerOfTwoFactor{shift: UVec2::new(2, 2), mask: UVec2::new(0b11, 0b11)}, 5),
            16 => (UVec2PowerOfTwoFactor{shift: UVec2::new(3, 2), mask: UVec2::new(0b111, 0b11)}, 4),
            8 => (UVec2PowerOfTwoFactor{shift: UVec2::new(3, 3), mask: UVec2::new(0b111, 0b111)}, 3),
            4 => (UVec2PowerOfTwoFactor{shift: UVec2::new(4, 3), mask: UVec2::new(0b1111, 0b111)}, 2),
            1 => (UVec2PowerOfTwoFactor{shift: UVec2::new(5, 4), mask: UVec2::new(0b11111, 0b1111)}, 0),
            _ => { panic!("Unexpected bpp"); }
        };
        const SPLAT_4: UVec2PowerOfTwoFactor = UVec2PowerOfTwoFactor{shift: UVec2::splat(2), mask: UVec2::splat(0b11)};
        const SPLAT_2: UVec2PowerOfTwoFactor = UVec2PowerOfTwoFactor{shift: UVec2::splat(1), mask: UVec2::splat(0b1)};

        if image_size.x < (4 << utile_size.shift.x) || image_size.y < (4 << utile_size.shift.y) {
            let size_in_utile = utile_size.round_up_div(image_size);
            LTTranslator {
                #[cfg(debug_assertions)]
                image_size,
                utile_size,
                size_in_utile,
                bpp_shift,
            }.into()
        } else {
            let size_in_utile = utile_size.round_up_div(image_size);
            let size_in_subtile = SPLAT_4.round_up_div(size_in_utile);
            let size_in_tile = SPLAT_2.round_up_div(size_in_subtile);
            TTranslator {
                #[cfg(debug_assertions)]
                image_size,
                utile_size,
                size_in_tile,
                bpp_shift,
            }.into()
        }
    }
}
