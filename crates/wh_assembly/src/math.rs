use crate::{asm, cfg_defs::DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, Core, DataFormat};

macro_rules! t6_reg_cfg_rmw {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::t6_reg_cfg_rmw($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
        }
    };
}

pub fn set_data_format(fmt0: DataFormat, fmt1: DataFormat) {
    t6_reg_cfg_rmw!(ALU_FORMAT_SPEC_REG0_SrcA, fmt0 as u32);
    t6_reg_cfg_rmw!(ALU_FORMAT_SPEC_REG1_SrcB, fmt1 as u32);
}

pub fn set_dest_base(addr: u32) {
    asm::SETC16d!(DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, addr);
}

pub fn elwadd(dst: u32, addr_mode: u32, bcast: u32, accum: bool, clr_valid: bool) {
    asm::ELWADDd!(
        dst,
        addr_mode,
        bcast,
        accum as u32,
        clr_valid as u32
    )
}
