use crate::{asm, cfg_defs::DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, DataFormat};

macro_rules! t6_reg_cfg_rmw {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::t6_reg_cfg_rmw($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
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

pub fn set_data_format(srca: DataFormat, srcb: DataFormat, dst: DataFormat) {
    if dst.width() == 4 {
        cfg_rmw!(ALU_ACC_CTRL_Fp32_enabled, 1);
    }
    t6_reg_cfg_rmw!(ALU_FORMAT_SPEC_REG0_SrcA, srca as u32);
    t6_reg_cfg_rmw!(ALU_FORMAT_SPEC_REG1_SrcB, srcb as u32);
    t6_reg_cfg_rmw!(ALU_FORMAT_SPEC_REG2_Dstacc, dst as u32);
}

pub fn set_dest_base(addr: u32) {
    asm::SETC16d!(DEST_TARGET_REG_CFG_MATH_Offset_ADDR32, addr);
}

pub fn elwadd(dst: u32, addr_mode: u32, bcast: u32, accum: bool, clr_valid: bool) {
    asm::ELWADDd!(clr_valid as u32, accum as u32, bcast, addr_mode, dst)
}
