use crate::asm;

#[inline]
pub fn cfg_pack0_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC0_REG1_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack1_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC0_REG8_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack2_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC1_REG1_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack3_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC1_REG8_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct PackSel(u32);

impl PackSel {
    pub const PACK_1: PackSel = PackSel(0);
    pub const PACK_2_LOW: PackSel = PackSel(0x3);
    pub const PACK_2_HIGH: PackSel = PackSel(0xC);
    pub const PACK_4: PackSel = PackSel(0xF);

    pub const PACKER_0: PackSel = PackSel(0b0001);
    pub const PACKER_1: PackSel = PackSel(0b0010);
    pub const PACKER_2: PackSel = PackSel(0b0100);
    pub const PACKER_3: PackSel = PackSel(0b1000);
}

impl core::ops::BitOr for PackSel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[inline]
pub fn pack<
    const ADDR_MODE: super::cfg::addr::PackedAddr,
    const PACKERS: PackSel,
    const LAST: bool,
>() {
    #[inline]
    pub const fn inner(
        addr_mode: super::cfg::addr::PackedAddr,
        zero_write: bool,
        packers: PackSel,
        ovrd_thread_id: bool,
        concat: bool,
        // What does this do?
        flush: bool,
        last: bool,
    ) -> u32 {
        asm::PACR_value!(
            addr_mode.val() as u32,
            zero_write as u32,
            packers.0,
            ovrd_thread_id as u32,
            concat as u32,
            flush as u32,
            last as u32
        )
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(ADDR_MODE, false, PACKERS, false, false, false, LAST) }
        );
    }
}
