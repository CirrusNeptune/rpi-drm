use enum_dispatch::enum_dispatch;
use glam::{UVec2, Vec2Swizzles};
use num::Integer;

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

    pub fn round_up_div_x(&self, val: u32) -> u32 {
        (val + self.mask.x - 1) >> self.shift.x
    }
}

const SPLAT_4: UVec2PowerOfTwoFactor = UVec2PowerOfTwoFactor {
    shift: UVec2::splat(2),
    mask: UVec2::splat(0b11),
};
const SPLAT_2: UVec2PowerOfTwoFactor = UVec2PowerOfTwoFactor {
    shift: UVec2::splat(1),
    mask: UVec2::splat(0b1),
};

pub trait U32Factor {
    fn mul_add(&self, mul: u32, add: u32) -> u32;
    fn div_rem(&self, val: u32) -> (u32, u32);
    fn div(&self, val: u32) -> u32;
    fn factor(&self) -> u32;
}

pub struct U32PowerOfTwoFactor {
    shift: u32,
    mask: u32,
}

impl U32Factor for U32PowerOfTwoFactor {
    fn mul_add(&self, mul: u32, add: u32) -> u32 {
        (mul << self.shift) | add
    }

    fn div_rem(&self, val: u32) -> (u32, u32) {
        let div = val >> self.shift;
        let rem = val & self.mask;
        (div, rem)
    }

    fn div(&self, val: u32) -> u32 {
        val >> self.shift
    }

    fn factor(&self) -> u32 {
        1 << self.shift
    }
}

pub struct U32NonPowerOfTwoFactor(u32);

impl U32Factor for U32NonPowerOfTwoFactor {
    fn mul_add(&self, mul: u32, add: u32) -> u32 {
        mul * self.0 + add
    }

    fn div_rem(&self, val: u32) -> (u32, u32) {
        val.div_rem(&self.0)
    }

    fn div(&self, val: u32) -> u32 {
        val / self.0
    }

    fn factor(&self) -> u32 {
        self.0
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

pub struct TTranslator<Fac: U32Factor> {
    #[cfg(debug_assertions)]
    image_size: UVec2,
    utile_size: UVec2PowerOfTwoFactor,
    size_in_tile_x: Fac,
    bpp_shift: u32,
}

impl<Fac: U32Factor> TranslatorTrait for TTranslator<Fac> {
    fn coordinate_to_tile_address(&self, coord: UVec2) -> TileAddress {
        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        let (utile_coord, pixel_in_utile) = self.utile_size.div_rem(coord);
        let (subtile_coord, utile_in_subtile) = SPLAT_4.div_rem(utile_coord);
        let (tile_coord, subtile_in_tile) = SPLAT_2.div_rem(subtile_coord);

        let flipped_tile_coord_x = if (tile_coord.y & 0x1) != 0 {
            self.size_in_tile_x.factor() - tile_coord.x - 1
        } else {
            tile_coord.x
        };
        let tile_index = self
            .size_in_tile_x
            .mul_add(tile_coord.y, flipped_tile_coord_x);

        let subtile_index = if tile_coord.y & 0x1 != 0 {
            const EVEN_LUT: [u32; 4] = [2, 3, 1, 0];
            EVEN_LUT[((subtile_in_tile.x << 1) | subtile_in_tile.y) as usize]
        } else {
            const ODD_LUT: [u32; 4] = [0, 1, 3, 2];
            ODD_LUT[((subtile_in_tile.x << 1) | subtile_in_tile.y) as usize]
        };

        let utile_index = utile_in_subtile.y * 4 + utile_in_subtile.x;

        let pixel_index = self
            .utile_size
            .mul_add_x(pixel_in_utile.y, pixel_in_utile.x);
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

        let tile_row = self.size_in_tile_x.div(tile_index);

        let subtile_in_tile = if tile_row & 0x1 != 0 {
            const EVEN_LUT: [UVec2; 4] = [
                UVec2::new(1, 1),
                UVec2::new(1, 0),
                UVec2::new(0, 0),
                UVec2::new(0, 1),
            ];
            EVEN_LUT[subtile_index as usize]
        } else {
            const ODD_LUT: [UVec2; 4] = [
                UVec2::new(0, 0),
                UVec2::new(0, 1),
                UVec2::new(1, 1),
                UVec2::new(1, 0),
            ];
            ODD_LUT[subtile_index as usize]
        };

        let tile_coord = if tile_row & 0x1 != 0 {
            let tmp = UVec2::from(self.size_in_tile_x.div_rem(tile_index)).yx();
            UVec2::new(self.size_in_tile_x.factor() - tmp.x - 1, tmp.y)
        } else {
            UVec2::from(self.size_in_tile_x.div_rem(tile_index)).yx()
        };

        let utile_in_subtile = UVec2::from(utile_index.div_rem(&4)).yx();

        let pixel_index = (pixel_byte * 8 + address.bit as u32) >> self.bpp_shift;
        let pixel_in_utile = UVec2::from(self.utile_size.div_rem_x(pixel_index)).yx();

        let subtile_coord = SPLAT_2.mul_add(tile_coord, subtile_in_tile);
        let utile_coord = SPLAT_4.mul_add(subtile_coord, utile_in_subtile);
        let coord = self.utile_size.mul_add(utile_coord, pixel_in_utile);

        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        coord
    }
}

pub struct LTTranslator<Fac: U32Factor> {
    #[cfg(debug_assertions)]
    image_size: UVec2,
    utile_size: UVec2PowerOfTwoFactor,
    size_in_utile_x: Fac,
    bpp_shift: u32,
}

impl<Fac: U32Factor> TranslatorTrait for LTTranslator<Fac> {
    fn coordinate_to_tile_address(&self, coord: UVec2) -> TileAddress {
        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        let (utile_coord, pixel_in_utile) = self.utile_size.div_rem(coord);

        let utile_index = self.size_in_utile_x.mul_add(utile_coord.y, utile_coord.x);

        let pixel_index = self
            .utile_size
            .mul_add_x(pixel_in_utile.y, pixel_in_utile.x);
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

        let utile_coord = UVec2::from(self.size_in_utile_x.div_rem(utile_index)).yx();

        let coord = self.utile_size.mul_add(utile_coord, pixel_in_utile);

        #[cfg(debug_assertions)]
        assert!(coord.x < self.image_size.x && coord.y < self.image_size.y);

        coord
    }
}

#[enum_dispatch(TranslatorTrait)]
pub enum Translator {
    TTranslatorPOT(TTranslator<U32PowerOfTwoFactor>),
    TTranslatorNPOT(TTranslator<U32NonPowerOfTwoFactor>),
    LTTranslatorPOT(LTTranslator<U32PowerOfTwoFactor>),
    LTTranslatorNPOT(LTTranslator<U32NonPowerOfTwoFactor>),
}

impl Translator {
    pub fn new(image_size: UVec2, bpp: u32) -> Self {
        let (utile_size, bpp_shift) = match bpp {
            64 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(1, 2),
                    mask: UVec2::new(0b1, 0b11),
                },
                6,
            ),
            32 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(2, 2),
                    mask: UVec2::new(0b11, 0b11),
                },
                5,
            ),
            16 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(3, 2),
                    mask: UVec2::new(0b111, 0b11),
                },
                4,
            ),
            8 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(3, 3),
                    mask: UVec2::new(0b111, 0b111),
                },
                3,
            ),
            4 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(4, 3),
                    mask: UVec2::new(0b1111, 0b111),
                },
                2,
            ),
            1 => (
                UVec2PowerOfTwoFactor {
                    shift: UVec2::new(5, 4),
                    mask: UVec2::new(0b11111, 0b1111),
                },
                0,
            ),
            _ => {
                panic!("Unexpected bpp");
            }
        };

        if image_size.x < (4 << utile_size.shift.x) || image_size.y < (4 << utile_size.shift.y) {
            let size_in_utile_x = utile_size.round_up_div_x(image_size.x);
            if size_in_utile_x.is_power_of_two() {
                LTTranslator {
                    #[cfg(debug_assertions)]
                    image_size,
                    utile_size,
                    size_in_utile_x: U32PowerOfTwoFactor {
                        shift: size_in_utile_x.trailing_zeros(),
                        mask: size_in_utile_x - 1,
                    },
                    bpp_shift,
                }
                .into()
            } else {
                LTTranslator {
                    #[cfg(debug_assertions)]
                    image_size,
                    utile_size,
                    size_in_utile_x: U32NonPowerOfTwoFactor(size_in_utile_x),
                    bpp_shift,
                }
                .into()
            }
        } else {
            let size_in_utile_x = utile_size.round_up_div_x(image_size.x);
            let size_in_subtile_x = SPLAT_4.round_up_div_x(size_in_utile_x);
            let size_in_tile_x = SPLAT_2.round_up_div_x(size_in_subtile_x);
            if size_in_tile_x.is_power_of_two() {
                TTranslator {
                    #[cfg(debug_assertions)]
                    image_size,
                    utile_size,
                    size_in_tile_x: U32PowerOfTwoFactor {
                        shift: size_in_tile_x.trailing_zeros(),
                        mask: size_in_tile_x - 1,
                    },
                    bpp_shift,
                }
                .into()
            } else {
                TTranslator {
                    #[cfg(debug_assertions)]
                    image_size,
                    utile_size,
                    size_in_tile_x: U32NonPowerOfTwoFactor(size_in_tile_x),
                    bpp_shift,
                }
                .into()
            }
        }
    }
}
