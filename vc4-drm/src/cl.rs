#![allow(dead_code)]
use std::io::{Result, Write};

fn gen_u8(v: u8, start: usize, end: usize) -> u8 {
    if cfg!(debug_assertions) {
        let width = end - start + 1;
        assert!(width < u8::BITS as usize);
        let max: u8 = (1 << width) - 1;
        assert!(v <= max);
    }
    v << start
}

fn gen_u32(v: u32, start: usize, end: usize) -> u32 {
    if cfg!(debug_assertions) {
        let width = end - start + 1;
        assert!(width < u32::BITS as usize);
        let max: u32 = (1 << width) - 1;
        assert!(v <= max);
    }
    v << start
}

fn div_round_up(n: u16, d: u16) -> u16 {
    if cfg!(debug_assertions) {
        n.checked_add(d)
            .unwrap()
            .checked_sub(1)
            .unwrap()
            .checked_div(d)
            .unwrap()
    } else {
        (n + d - 1) / d
    }
}

use fmt::Debug;
use num::cast::AsPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::marker::Copy;

fn checked_into<T: AsPrimitive<U>, U: 'static + Copy + TryFrom<T>>(v: T) -> U
where
    <U as TryFrom<T>>::Error: Debug,
{
    if cfg!(debug_assertions) {
        v.try_into().unwrap()
    } else {
        v.as_()
    }
}

pub trait BinClStructure {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TileBlockSize {
    #[default]
    Size32 = 0,
    Size64 = 1,
    Size128 = 2,
    Size256 = 3,
}

#[derive(Default, Debug)]
pub struct TileBinningModeConfiguration {
    pub tile_allocation_memory_address: u32,
    pub tile_allocation_memory_size: u32,
    pub tile_state_data_array_address: u32,
    pub width_in_tiles: u8,
    pub height_in_tiles: u8,
    pub multisample_mode_4x: bool,
    pub tile_buffer_64_bit_color_depth: bool,
    pub auto_initialise_tile_state_data_array: bool,
    pub tile_allocation_initial_block_size: TileBlockSize,
    pub tile_allocation_block_size: TileBlockSize,
    pub double_buffer_in_non_ms_mode: bool,
}

impl TileBinningModeConfiguration {
    pub fn set_size_in_pixels(&mut self, width: u16, height: u16) {
        let mut tile_size_w: u16 = 64;
        let mut tile_size_h: u16 = 64;

        if self.multisample_mode_4x {
            tile_size_w >>= 1;
            tile_size_h >>= 1;
        }

        if self.tile_buffer_64_bit_color_depth {
            tile_size_h >>= 1;
        }

        self.width_in_tiles = checked_into(div_round_up(width, tile_size_w));
        self.height_in_tiles = checked_into(div_round_up(height, tile_size_h));
    }
}

impl BinClStructure for TileBinningModeConfiguration {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_TILE_BINNING_MODE_CONFIGURATION: u8 = 112;
        let mut buf = [0_u8; 16];
        buf[0] = V3D21_TILE_BINNING_MODE_CONFIGURATION;
        buf[1..5].copy_from_slice(&self.tile_allocation_memory_address.to_le_bytes());
        buf[5..9].copy_from_slice(&self.tile_allocation_memory_size.to_le_bytes());
        buf[9..13].copy_from_slice(&self.tile_state_data_array_address.to_le_bytes());
        buf[13] = self.width_in_tiles;
        buf[14] = self.height_in_tiles;
        buf[15] = gen_u8(self.double_buffer_in_non_ms_mode as u8, 7, 7)
            | gen_u8(self.tile_allocation_block_size as u8, 5, 6)
            | gen_u8(self.tile_allocation_initial_block_size as u8, 3, 4)
            | gen_u8(self.auto_initialise_tile_state_data_array as u8, 2, 2)
            | gen_u8(self.tile_buffer_64_bit_color_depth as u8, 1, 1)
            | gen_u8(self.multisample_mode_4x as u8, 0, 0);
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct StartTileBinning;

impl BinClStructure for StartTileBinning {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_START_TILE_BINNING: u8 = 6;
        writer.write_all(&[V3D21_START_TILE_BINNING])
    }
}

#[derive(Default, Debug)]
pub struct IncrementSemaphore;

impl BinClStructure for IncrementSemaphore {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_INCREMENT_SEMAPHORE: u8 = 7;
        writer.write_all(&[V3D21_INCREMENT_SEMAPHORE])
    }
}

#[derive(Default, Debug)]
pub struct Flush;

impl BinClStructure for Flush {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_FLUSH: u8 = 4;
        writer.write_all(&[V3D21_FLUSH])
    }
}

#[derive(Default, Debug)]
pub struct LineWidth {
    pub line_width: f32,
}

impl BinClStructure for LineWidth {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_LINE_WIDTH: u8 = 99;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_LINE_WIDTH;
        buf[1..5].copy_from_slice(&self.line_width.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct ClipWindow {
    pub clip_window_left_pixel_coordinate: u16,
    pub clip_window_bottom_pixel_coordinate: u16,
    pub clip_window_width_in_pixels: u16,
    pub clip_window_height_in_pixels: u16,
}

impl BinClStructure for ClipWindow {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_CLIP_WINDOW: u8 = 102;
        let mut buf = [0_u8; 9];
        buf[0] = V3D21_CLIP_WINDOW;
        buf[1..3].copy_from_slice(&self.clip_window_left_pixel_coordinate.to_le_bytes());
        buf[3..5].copy_from_slice(&self.clip_window_bottom_pixel_coordinate.to_le_bytes());
        buf[5..7].copy_from_slice(&self.clip_window_width_in_pixels.to_le_bytes());
        buf[7..9].copy_from_slice(&self.clip_window_height_in_pixels.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct ClipperXYScaling {
    pub viewport_half_width_in_1_16th_of_pixel: f32,
    pub viewport_half_height_in_1_16th_of_pixel: f32,
}

impl BinClStructure for ClipperXYScaling {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_CLIPPER_XY_SCALING: u8 = 105;
        let mut buf = [0_u8; 9];
        buf[0] = V3D21_CLIPPER_XY_SCALING;
        buf[1..5].copy_from_slice(&self.viewport_half_width_in_1_16th_of_pixel.to_le_bytes());
        buf[5..9].copy_from_slice(&self.viewport_half_height_in_1_16th_of_pixel.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct ViewportOffset {
    pub viewport_centre_x_coordinate_12_4: u16,
    pub viewport_centre_y_coordinate_12_4: u16,
}

impl BinClStructure for ViewportOffset {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_VIEWPORT_OFFSET: u8 = 103;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_VIEWPORT_OFFSET;
        buf[1..3].copy_from_slice(&self.viewport_centre_x_coordinate_12_4.to_le_bytes());
        buf[3..5].copy_from_slice(&self.viewport_centre_y_coordinate_12_4.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum CompareFunction {
    #[default]
    Never = 0,
    Less = 1,
    Equal = 2,
    LEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GEqual = 6,
    Always = 7,
}

#[derive(Default, Debug)]
pub struct ConfigurationBits {
    pub enable_forward_facing_primitive: bool,
    pub enable_reverse_facing_primitive: bool,
    pub clockwise_primitives: bool,
    pub enable_depth_offset: bool,
    pub antialiased_points_and_lines: bool,
    pub coverage_read_type: bool,
    pub rasteriser_oversample_mode: u8,
    pub coverage_pipe_select: bool,
    pub coverage_update_mode: u8,
    pub coverage_read_mode: bool,
    pub depth_test_function: CompareFunction,
    pub z_updates_enable: bool,
    pub early_z_enable: bool,
    pub early_z_updates_enable: bool,
}

impl BinClStructure for ConfigurationBits {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_CONFIGURATION_BITS: u8 = 96;
        writer.write_all(&[
            V3D21_CONFIGURATION_BITS,
            gen_u8(self.rasteriser_oversample_mode, 6, 7)
                | gen_u8(self.coverage_read_type as u8, 5, 5)
                | gen_u8(self.antialiased_points_and_lines as u8, 4, 4)
                | gen_u8(self.enable_depth_offset as u8, 3, 3)
                | gen_u8(self.clockwise_primitives as u8, 2, 2)
                | gen_u8(self.enable_reverse_facing_primitive as u8, 1, 1)
                | gen_u8(self.enable_forward_facing_primitive as u8, 0, 0),
            gen_u8(self.z_updates_enable as u8, 7, 7)
                | gen_u8(self.depth_test_function as u8, 4, 6)
                | gen_u8(self.coverage_read_mode as u8, 3, 3)
                | gen_u8(self.coverage_update_mode, 1, 2)
                | gen_u8(self.coverage_pipe_select as u8, 0, 0),
            gen_u8(self.early_z_updates_enable as u8, 1, 1)
                | gen_u8(self.early_z_enable as u8, 0, 0),
        ])
    }
}

#[derive(Default, Debug)]
pub struct DepthOffset {
    pub depth_offset_factor: u16,
    pub depth_offset_units: u16,
}

impl BinClStructure for DepthOffset {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_DEPTH_OFFSET: u8 = 101;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_DEPTH_OFFSET;
        buf[1..3].copy_from_slice(&self.depth_offset_factor.to_le_bytes());
        buf[3..5].copy_from_slice(&self.depth_offset_units.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct ClipperZScaleAndOffset {
    pub viewport_z_scale_zc_to_zs: f32,
    pub viewport_z_offset_zc_to_zs: f32,
}

impl BinClStructure for ClipperZScaleAndOffset {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_CLIPPER_Z_SCALE_AND_OFFSET: u8 = 106;
        let mut buf = [0_u8; 9];
        buf[0] = V3D21_CLIPPER_Z_SCALE_AND_OFFSET;
        buf[1..5].copy_from_slice(&self.viewport_z_scale_zc_to_zs.to_le_bytes());
        buf[5..9].copy_from_slice(&self.viewport_z_offset_zc_to_zs.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct PointSize {
    pub point_size: f32,
}

impl BinClStructure for PointSize {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_POINT_SIZE: u8 = 98;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_POINT_SIZE;
        buf[1..5].copy_from_slice(&self.point_size.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct FlatShadeFlags {
    pub flat_shading_flags: u32,
}

impl BinClStructure for FlatShadeFlags {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_FLAT_SHADE_FLAGS: u8 = 97;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_FLAT_SHADE_FLAGS;
        buf[1..5].copy_from_slice(&self.flat_shading_flags.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum PrimitiveMode {
    #[default]
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

#[derive(Default, Debug)]
pub struct VertexArrayPrimitives {
    pub primitive_mode: PrimitiveMode,
    pub length: u32,
    pub index_of_first_vertex: u32,
}

impl BinClStructure for VertexArrayPrimitives {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_VERTEX_ARRAY_PRIMITIVES: u8 = 33;
        let mut buf = [0_u8; 10];
        buf[0] = V3D21_VERTEX_ARRAY_PRIMITIVES;
        buf[1] = self.primitive_mode as u8;
        buf[2..6].copy_from_slice(&self.length.to_le_bytes());
        buf[6..10].copy_from_slice(&self.index_of_first_vertex.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct GlShaderState {
    pub address: u32,
    pub extended_shader_record: bool,
    pub number_of_attribute_arrays: u8,
}

impl BinClStructure for GlShaderState {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        const V3D21_GL_SHADER_STATE: u8 = 64;
        let mut buf = [0_u8; 5];
        buf[0] = V3D21_GL_SHADER_STATE;
        buf[1..5].copy_from_slice(
            &(self.address
                | ((self.extended_shader_record as u32) << 3)
                | self.number_of_attribute_arrays as u32)
                .to_le_bytes(),
        );
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct GlShaderRecord {
    pub fragment_shader_is_single_threaded: bool,
    pub point_size_included_in_shaded_vertex_data: bool,
    pub enable_clipping: bool,
    pub fragment_shader_number_of_uniforms_not_used_currently: u16,
    pub fragment_shader_number_of_varyings: u8,
    pub fragment_shader_code_address_offset: u32,
    pub fragment_shader_uniforms_address: u32,
    pub vertex_shader_number_of_uniforms_not_used_currently: u16,
    pub vertex_shader_attribute_array_select_bits: u8,
    pub vertex_shader_total_attributes_size: u8,
    pub vertex_shader_code_address_offset: u32,
    pub vertex_shader_uniforms_address: u32,
    pub coordinate_shader_number_of_uniforms_not_used_currently: u16,
    pub coordinate_shader_attribute_array_select_bits: u8,
    pub coordinate_shader_total_attributes_size: u8,
    pub coordinate_shader_code_address_offset: u32,
    pub coordinate_shader_uniforms_address: u32,
}

impl BinClStructure for GlShaderRecord {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut buf = [0_u8; 36];
        buf[0] = gen_u8(self.fragment_shader_is_single_threaded as u8, 0, 0)
            | gen_u8(self.point_size_included_in_shaded_vertex_data as u8, 1, 1)
            | gen_u8(self.enable_clipping as u8, 2, 2);
        buf[2] = self.fragment_shader_number_of_uniforms_not_used_currently as u8;
        buf[3] = self.fragment_shader_number_of_varyings;
        buf[4..8].copy_from_slice(&self.fragment_shader_code_address_offset.to_le_bytes());
        buf[8..12].copy_from_slice(&self.fragment_shader_uniforms_address.to_le_bytes());
        buf[12..14].copy_from_slice(
            &self
                .vertex_shader_number_of_uniforms_not_used_currently
                .to_le_bytes(),
        );
        buf[14] = self.vertex_shader_attribute_array_select_bits;
        buf[15] = self.vertex_shader_total_attributes_size;
        // Uniform and code overlap in C implementation??? Cannot use vertex shader code offset.
        //buf[16..20].copy_from_slice(&self.vertex_shader_code_address_offset.to_le_bytes());
        buf[16..20].copy_from_slice(&self.vertex_shader_uniforms_address.to_le_bytes());
        buf[24..26].copy_from_slice(
            &self
                .coordinate_shader_number_of_uniforms_not_used_currently
                .to_le_bytes(),
        );
        buf[26] = self.coordinate_shader_attribute_array_select_bits;
        buf[27] = self.coordinate_shader_total_attributes_size;
        buf[28..32].copy_from_slice(&self.coordinate_shader_code_address_offset.to_le_bytes());
        buf[32..36].copy_from_slice(&self.coordinate_shader_uniforms_address.to_le_bytes());
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug)]
pub struct AttributeRecord {
    pub address: u32,
    pub number_of_bytes_minus_1: u8,
    pub stride: u8,
    pub vertex_shader_vpm_offset: u8,
    pub coordinate_shader_vpm_offset: u8,
}

impl BinClStructure for AttributeRecord {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut buf = [0_u8; 8];
        buf[0..4].copy_from_slice(&self.address.to_le_bytes());
        buf[4] = self.number_of_bytes_minus_1;
        buf[5] = self.stride;
        buf[6] = self.vertex_shader_vpm_offset;
        buf[7] = self.coordinate_shader_vpm_offset;
        writer.write_all(&buf)
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TextureDataType {
    #[default]
    RGBA8888 = 0,
    RGBX8888 = 1,
    RGBA4444 = 2,
    RGBA5551 = 3,
    RGB565 = 4,
    Luminance = 5,
    Alpha = 6,
    LumAlpha = 7,
    ETC1 = 8,
    S16F = 9,
    S8 = 10,
    S16 = 11,
    BW1 = 12,
    A4 = 13,
    A1 = 14,
    RGBA64 = 15,
    RGBA32R = 16,
    YUYV422R = 17,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TextureMagFilterType {
    #[default]
    Linear = 0,
    Nearest = 1,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TextureMinFilterType {
    #[default]
    Linear = 0,
    Nearest = 1,
    NearestMipNearest = 2,
    NearestMipLinear = 3,
    LinearMipNearest = 4,
    LinearMipLinear = 5,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum TextureWrapType {
    #[default]
    Repeat = 0,
    Clamp = 1,
    Mirror = 2,
    Border = 3,
}

#[derive(Default, Debug)]
pub struct TextureConfigUniform {
    pub base_address: u32,
    pub cache_swizzle: u8,
    pub cube_map: bool,
    pub flip_y: bool,
    pub data_type: TextureDataType,
    pub num_mips: u8,

    pub height: u16,
    pub etc_flip: bool,
    pub width: u16,
    pub mag_filt: TextureMagFilterType,
    pub min_filt: TextureMinFilterType,
    pub wrap_t: TextureWrapType,
    pub wrap_s: TextureWrapType,
}

impl TextureConfigUniform {
    pub fn get_1d_word(&self) -> u32 {
        ((self.num_mips as u32 - 1) & 0xf)
            | (((self.data_type as u32) & 0xf) << 4)
            | ((self.cube_map as u32) << 9)
            | ((self.base_address & 0xfffff) << 12)
    }

    pub fn get_2d_word(&self) -> u32 {
        ((self.wrap_s as u32) & 0x3)
            | (((self.wrap_t as u32) & 0x3) << 2)
            | (((self.min_filt as u32) & 0x7) << 4)
            | (((self.mag_filt as u32) & 0x1) << 7)
            | (((self.width as u32) & 0x7ff) << 8)
            | (((self.height as u32) & 0x7ff) << 20)
            | ((((self.data_type as u32) & 0x10) >> 4) << 31)
    }
}
