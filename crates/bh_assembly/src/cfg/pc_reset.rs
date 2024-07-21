use crate::tensix_defs;

pub fn set_trisc0_reset_pc(pc: u32) {
    let addr = tensix_defs::RISCV_DEBUG_REG_TRISC0_RESET_PC as *mut u32;
    unsafe {
        addr.write_volatile(pc);
    }

    let base = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        base.write_volatile(base.read_volatile() | 0b001);
    }
}

pub fn unset_trisc0_reset_pc() {
    let base = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        base.write_volatile(base.read_volatile() & 0b110);
    }
}

pub fn set_trisc1_reset_pc(pc: u32) {
    let addr = tensix_defs::RISCV_DEBUG_REG_TRISC1_RESET_PC as *mut u32;
    unsafe {
        addr.write_volatile(pc);
    }

    let ovr = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(ovr.read_volatile() | 0b010);
    }
}

pub fn unset_trisc1_reset_pc() {
    let ovr = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(ovr.read_volatile() & 0b101);
    }
}

pub fn set_trisc2_reset_pc(pc: u32) {
    let addr = tensix_defs::RISCV_DEBUG_REG_TRISC2_RESET_PC as *mut u32;
    unsafe {
        addr.write_volatile(pc);
    }

    let ovr = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(ovr.read_volatile() | 0b100);
    }
}

pub fn unset_trisc2_reset_pc() {
    let ovr = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(ovr.read_volatile() & 0b110);
    }
}

pub fn set_ncrisc_reset_pc(pc: u32) {
    let addr = tensix_defs::RISCV_DEBUG_REG_NCRISC_RESET_PC as *mut u32;
    unsafe {
        addr.write_volatile(pc);
    }

    let ovr = tensix_defs::RISCV_DEBUG_REG_NCRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(1);
    }
}

pub fn unset_ncrisc_reset_pc() {
    let ovr = tensix_defs::RISCV_DEBUG_REG_TRISC_RESET_PC_OVERRIDE as *mut u32;
    unsafe {
        ovr.write_volatile(0);
    }
}
