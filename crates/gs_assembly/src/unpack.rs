use crate::{SETADCXY_value, SETADCZW_value};

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

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct ChannelSel(pub u32);

impl ChannelSel {
    pub const CH1_Y: ChannelSel = ChannelSel(0b1000);
    pub const CH1_X: ChannelSel = ChannelSel(0b0100);
    pub const CH0_Y: ChannelSel = ChannelSel(0b0010);
    pub const CH0_X: ChannelSel = ChannelSel(0b0001);
    pub const ALL: ChannelSel = ChannelSel(0b1111);
}

impl core::ops::BitOr for ChannelSel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

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

#[inline]
pub fn set_adc_xy<
    const BLOCK_SEL: BlockSel,
    const CHANNEL_SEL: ChannelSel,
    const CH0_X: u8,
    const CH0_Y: u8,
    const CH1_X: u8,
    const CH1_Y: u8,
>() {
    #[inline]
    const fn inner(
        block_sel: BlockSel,
        channel_sel: ChannelSel,
        ch0_x: u8,
        ch0_y: u8,
        ch1_x: u8,
        ch1_y: u8,
    ) -> u32 {
        SETADCXY_value!(
            block_sel.0,
            ch1_y as u32,
            ch1_x as u32,
            ch0_y as u32,
            ch0_x as u32,
            channel_sel.0
        )
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(BLOCK_SEL, CHANNEL_SEL, CH0_X, CH0_Y, CH1_X, CH1_Y) }
        );
    }
}

#[inline]
pub fn set_adc_zw<
    const BLOCK_SEL: BlockSel,
    const CHANNEL_SEL: ChannelSel,
    const CH0_Z: u8,
    const CH0_W: u8,
    const CH1_Z: u8,
    const CH1_W: u8,
>() {
    #[inline]
    const fn inner(
        block_sel: BlockSel,
        channel_sel: ChannelSel,
        ch0_z: u8,
        ch0_w: u8,
        ch1_z: u8,
        ch1_w: u8,
    ) -> u32 {
        SETADCZW_value!(
            block_sel.0,
            ch1_w as u32,
            ch1_z as u32,
            ch0_w as u32,
            ch0_z as u32,
            channel_sel.0
        )
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(BLOCK_SEL, CHANNEL_SEL, CH0_Z, CH0_W, CH1_Z, CH1_W) }
        );
    }
}

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
            1,           /*: use context id instead of thread id*/
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
