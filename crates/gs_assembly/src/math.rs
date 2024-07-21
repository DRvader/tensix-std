use crate::{cfg_defs::DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, MOVA2D_value};

pub mod fpu;

pub fn select_math_dest<const USE_A: bool>() {
    const fn inner(use_a: bool) -> u32 {
        let val: u32 = if use_a { 0x200 } else { 0x0 };
        crate::SETC16_value!(DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, val)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(USE_A) });
    }
}

pub fn movea2d<
    const DST: u16,
    const SRC: u8,
    const ADDR: crate::cfg::addr::AddrSel,
    const MODIFIER: u8,
>() {
    const fn inner(dst: u16, src: u8, addr: crate::cfg::addr::AddrSel, modifier: u8) -> u32 {
        MOVA2D_value!(modifier as u32, addr as u32, src as u32, dst as u32)
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(DST, SRC, ADDR, MODIFIER) }
        );
    }
}
