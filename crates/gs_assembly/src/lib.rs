#![no_std]
#![feature(adt_const_params, generic_const_exprs, sync_unsafe_cell)]

pub mod asm;

pub mod cfg;
#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[allow(unused_imports)]
pub mod cfg_defs;

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[allow(unused_imports)]
pub mod tensix_defs;

pub mod pack;
pub mod sync;
pub mod unpack;

pub mod math;
#[allow(unused)]
pub mod noc;
pub mod noc_map;

pub mod debug;

#[repr(u32)]
pub enum SoftReset {
    Brisc = 1 << 11,
    Trisc0 = 1 << 12,
    Trisc1 = 1 << 13,
    Trisc2 = 1 << 14,
    Ncrisc = 1 << 18,
}

pub fn deassert_reset(reset: SoftReset) {
    unsafe {
        let mut base = (tensix_defs::RISCV_DEBUG_REG_SOFT_RESET_0 as *mut u32).read_volatile();
        base &= !(reset as u32);
        (tensix_defs::RISCV_DEBUG_REG_SOFT_RESET_0 as *mut u32).write_volatile(base);
    }
}

pub fn assert_reset(reset: SoftReset) {
    unsafe {
        let mut base = (tensix_defs::RISCV_DEBUG_REG_SOFT_RESET_0 as *mut u32).read_volatile();
        base |= reset as u32;
        (tensix_defs::RISCV_DEBUG_REG_SOFT_RESET_0 as *mut u32).write_volatile(base);
    }
}

#[inline]
pub fn nop() {
    unsafe {
        core::arch::asm!(".word ({value})", value = const { asm::NOP_value!() });
    }
}

pub fn reg_write(reg: usize, value: u32) {
    let regfile = tensix_defs::REGFILE_BASE as *mut u32;

    debug_assert!(reg < 64);

    unsafe { regfile.add(reg).write_volatile(value) };
}

pub fn reg_read(reg: usize) -> u32 {
    let regfile = tensix_defs::REGFILE_BASE as *mut u32;

    debug_assert!(reg < 64);

    unsafe { regfile.add(reg).read_volatile() }
}
