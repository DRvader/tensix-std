use crate::cfg_defs;

use super::{cfg_read, cfg_rmw};

pub fn set_trisc0_reset_pc(pc: u32) {
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_SEC0_PC_ADDR32,
        cfg_defs::TRISC_RESET_PC_SEC0_PC_SHAMT,
        cfg_defs::TRISC_RESET_PC_SEC0_PC_MASK,
        pc,
    );

    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base | 0b001,
    );
}

pub fn unset_trisc0_reset_pc() {
    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base & 0b110,
    );
}

pub fn set_trisc1_reset_pc(pc: u32) {
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_SEC1_PC_ADDR32,
        cfg_defs::TRISC_RESET_PC_SEC1_PC_SHAMT,
        cfg_defs::TRISC_RESET_PC_SEC1_PC_MASK,
        pc,
    );

    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base | 0b010,
    );
}

pub fn unset_trisc1_reset_pc() {
    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base & 0b101,
    );
}

pub fn set_trisc2_reset_pc(pc: u32) {
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_SEC2_PC_ADDR32,
        cfg_defs::TRISC_RESET_PC_SEC2_PC_SHAMT,
        cfg_defs::TRISC_RESET_PC_SEC2_PC_MASK,
        pc,
    );

    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base | 0b100,
    );
}

pub fn unset_trisc2_reset_pc() {
    let base = cfg_read(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
    );
    cfg_rmw(
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::TRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        base & 0b011,
    );
}

pub fn set_ncrisc_reset_pc(pc: u32) {
    cfg_rmw(
        cfg_defs::NCRISC_RESET_PC_PC_ADDR32,
        cfg_defs::NCRISC_RESET_PC_PC_SHAMT,
        cfg_defs::NCRISC_RESET_PC_PC_MASK,
        pc,
    );

    cfg_rmw(
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        1,
    );
}

pub fn unset_ncrisc_reset_pc() {
    cfg_rmw(
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_ADDR32,
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_SHAMT,
        cfg_defs::NCRISC_RESET_PC_OVERRIDE_Reset_PC_Override_en_MASK,
        0,
    );
}
