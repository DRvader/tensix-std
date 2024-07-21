use crate::{
    cfg_defs::{
        SRCA_SET_SetOvrdWithAddr_ADDR32, THCON_SEC0_REG0_TileDescriptor_ADDR32,
        THCON_SEC1_REG0_TileDescriptor_ADDR32,
    },
    tensix_defs::REGFILE_BASE,
    DataFormat, SETADCXYd, SETADCZWd, SETC16d, WRCFGd,
};

mod hw_workarounds;

use super::asm;

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
#[repr(u32)]
pub enum UnpackSelect {
    SrcA = 0,
    SrcB = 1,
}

#[inline]
pub fn cfg_srca<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(reg, 0, crate::cfg_defs::UNP0_ADDR_BASE_REG_0_Base_ADDR32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_srcb<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(reg, 0, crate::cfg_defs::UNP1_ADDR_BASE_REG_0_Base_ADDR32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct Bits<const N: usize>(pub u32);

impl<const N: usize> Bits<N> {}

impl<const N: usize> core::ops::BitOr for Bits<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl<const N: usize> core::ops::BitAnd for Bits<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const N: usize> core::ops::Not for Bits<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 | ((1 << N) - 1))
    }
}

macro_rules! bit_impls {
    ($name:ty) => {
        impl core::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl $name {
            pub const fn cbitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }

            pub const fn cbitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }

            pub const fn cnot(self) -> Self {
                Self(self.0)
            }
        }
    };
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct ChannelSel(pub u32);

impl ChannelSel {
    pub const CH1_Y: ChannelSel = ChannelSel(0b1000);
    pub const CH1_X: ChannelSel = ChannelSel(0b0100);
    pub const CH0_Y: ChannelSel = ChannelSel(0b0010);
    pub const CH0_X: ChannelSel = ChannelSel(0b0001);
    pub const ALL: ChannelSel = ChannelSel(0b1111);
}

bit_impls!(ChannelSel);

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct BlockSel(pub u32);

impl BlockSel {
    pub const UNPCKER0: BlockSel = BlockSel(0x1);
    pub const UNPCKER1: BlockSel = BlockSel(0x2);
    pub const UNPACKER: BlockSel = BlockSel(0x3);
    pub const PACKER0: BlockSel = BlockSel(0x4);
}

impl core::ops::BitOr for BlockSel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::Not for BlockSel {
    type Output = Self;

    fn not(self) -> Self::Output {
        // Negate and then just mask off the bottom bits we actuall use
        Self(!self.0 | ((1 << 3) - 1))
    }
}

macro_rules! cfg_write {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::cfg_write($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
        }
    };
}

macro_rules! cfg_rmw {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::cfg_rmw($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
        }
    };
}

// typedef struct {
//   uint32_t data_format : 4;
//   uint32_t uncompressed: 1;
//   uint32_t reserved_0  : 3;
//   uint32_t blobs_per_xy_plane  : 4;
//   uint32_t reserved_1  : 4;
//   uint32_t x_dim       : 16;
//   uint32_t y_dim       : 16;
//   uint32_t z_dim       : 16;
//   uint32_t w_dim       : 16;
//   uint32_t blobs_y_start : 32;
//   uint32_t digest_type : 8;  // Not used
//   uint32_t digest_size : 8;  // Not used
// } tile_descriptor_t; // Unpack configuration

pub struct UnpackSettings {
    // In 16b quants
    pub l1_addr: u32,
    pub has_header: bool,

    pub l1_format: DataFormat,
    pub reg_format: DataFormat,
    pub compressed: bool,
    pub x_dim: u16,
    pub y_dim: u16,
    pub z_dim: u16,
    pub w_dim: u16,
}

impl Default for UnpackSettings {
    fn default() -> Self {
        Self {
            l1_addr: 0,
            has_header: false,
            l1_format: DataFormat::Tf32,
            reg_format: DataFormat::Tf32,
            compressed: false,
            x_dim: 1,
            y_dim: 1,
            z_dim: 1,
            w_dim: 1,
        }
    }
}

impl UnpackSettings {
    pub fn one_tile_per_row(mut self) -> Self {
        self.x_dim = self.x_dim * self.y_dim;
        self.y_dim = 1;
        self.z_dim = self.z_dim * self.w_dim;
        self.w_dim = 1;

        self
    }

    fn pack_tile_descriptor(&self) -> (u32, u32, u32, u32) {
        let fmt = 0x10
            | if self.l1_format == DataFormat::Lf8Plus {
                DataFormat::Lf8 as u32
            } else {
                self.l1_format as u32
            };

        (
            ((self.x_dim as u32) << 16) | ((self.compressed as u32) << 4) | fmt,
            ((self.z_dim as u32) << 16) | (self.y_dim as u32),
            (self.w_dim as u32),
            0,
        )
    }
}

pub fn reset_counters() {
    SETADCXYd!(0b011, 0, 0, 0, 0, 0b1111);
    SETADCZWd!(0b011, 0, 0, 0, 0, 0b1111);
}

pub fn init() {
    // Should be in an init function
    SETC16d!(SRCA_SET_SetOvrdWithAddr_ADDR32, 0b100); // This allows multiple unpacks per src bank

    reset_counters();
}

pub fn configure(srca: UnpackSettings, srcb: UnpackSettings) {
    init();

    let src_a_tile = srca.pack_tile_descriptor();
    let src_b_tile = srcb.pack_tile_descriptor();

    unsafe {
        (REGFILE_BASE as *mut u32).write_volatile(src_a_tile.0);
        (REGFILE_BASE as *mut u32)
            .add(1)
            .write_volatile(src_a_tile.1);
        (REGFILE_BASE as *mut u32)
            .add(2)
            .write_volatile(src_a_tile.2);
        (REGFILE_BASE as *mut u32)
            .add(3)
            .write_volatile(src_a_tile.3);

        WRCFGd!(0, 1, THCON_SEC0_REG0_TileDescriptor_ADDR32);
    }

    unsafe {
        (REGFILE_BASE as *mut u32).write_volatile(src_b_tile.0);
        (REGFILE_BASE as *mut u32)
            .add(1)
            .write_volatile(src_b_tile.1);
        (REGFILE_BASE as *mut u32)
            .add(2)
            .write_volatile(src_b_tile.2);
        (REGFILE_BASE as *mut u32)
            .add(3)
            .write_volatile(src_b_tile.3);

        WRCFGd!(0, 1, THCON_SEC1_REG0_TileDescriptor_ADDR32);
    }

    // Due to the hardware implementation we need to use a lookup table to figure out the value to
    // program here.
    // In addition due to some specifics of the hardware, the unpacker will subtract a "magic"
    // value from whatever we program into it. So to work around this we just add an offset based
    // on our lookup table.
    // NOTE: The offset only applies to srca
    let (real_srca_fmt, tile_offset) =
        hw_workarounds::weird_unpacr_workarounds(srca.l1_format, srca.reg_format);
    cfg_rmw!(THCON_SEC0_REG2_Out_data_format, real_srca_fmt as u32);
    cfg_rmw!(UNP0_ADDR_BASE_REG_1_Base, tile_offset);

    let (real_srcb_fmt, _tile_offset) =
        hw_workarounds::weird_unpacr_workarounds(srcb.l1_format, srcb.reg_format);
    cfg_rmw!(THCON_SEC1_REG2_Out_data_format, real_srcb_fmt as u32);

    cfg_write!(
        THCON_SEC0_REG3_Base_address,
        srca.l1_addr / 16 - if srca.has_header { 0 } else { 1 }
    );

    cfg_write!(
        THCON_SEC1_REG3_Base_address,
        srcb.l1_addr / 16 - if srcb.has_header { 0 } else { 1 }
    );

    asm::SETADCXXd!(
        BlockSel::UNPCKER0.0,
        srca.x_dim as u32 - 1,
        0
    );
    asm::SETADCXXd!(
        BlockSel::UNPCKER1.0,
        srcb.x_dim as u32 - 1,
        0
    );
}

/*
#[inline]
pub fn unpack<const UNPACKER_SELECT: UnpackSelect, const ADDRESS_MODE: u32, const LAST: bool>() {
    #[inline]
    const fn inner(unpacker_select: u32, address_mode: u32, last: bool) -> u32 {
        asm::UNPACR_value!(
            unpacker_select,
            address_mode,
            0,           /*: cfg_context_inc*/
            0,           /*: cfg_context_id (0..7)*/
            0,           /*: addr_context_id (0..2)*/
            0,           /*: use context id instead of thread id*/
            1,           /*: set data valid after write [bool]*/
            0,           /*: rareify srcb [bool]*/
            0,           /*: write zeros [bool]*/
            0,           /*: incr cfg context id after unpack [bool]*/
            0, /*: search for start and end of seleced row(s) within tile, if false saerch is done per datums(s) [bool]*/
            0, /*: flush row pointer cache in the search block [bool]*/
            last as u32  /*: Flash data accumulation buffer to memory [bool]*/
        )
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(UNPACKER_SELECT as u32, ADDRESS_MODE, LAST) }
        );
    }
}
*/

#[inline]
pub fn unpack(sel: UnpackSelect, addr_mode: u32, last: bool) {
    asm::UNPACRd!(
        sel as u32,
        addr_mode,
        0,           /*: cfg_context_inc*/
        0,           /*: cfg_context_id (0..7)*/
        0,           /*: addr_context_id (0..2)*/
        0,           /*: use context id instead of thread id*/
        1,           /*: set data valid after write [bool]*/
        0,           /*: rareify srcb [bool]*/
        0,           /*: write zeros [bool]*/
        0,           /*: incr cfg context id after unpack [bool]*/
        0, /*: search for start and end of seleced row(s) within tile, if false saerch is done per datums(s) [bool]*/
        0, /*: flush row pointer cache in the search block [bool]*/
        last as u32  /*: Flash data accumulation buffer to memory [bool]*/
    )
}
