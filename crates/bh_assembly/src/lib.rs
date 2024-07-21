#![no_std]
#![feature(adt_const_params, generic_const_exprs, const_trait_impl)]

use tensix_defs::{INSTRN1_BUF_BASE, INSTRN2_BUF_BASE, INSTRN_BUF_BASE};

pub mod asm;

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[allow(unused_imports)]
pub mod cfg_defs;

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[allow(unused_imports)]
pub mod tensix_defs;

pub mod cfg;
pub mod math;
pub mod pack;
pub mod unpack;
pub mod sync;
#[allow(unused)]
pub mod noc;
pub mod noc_map;

const INSTR_BUF_LOOKUP: [u32; 3] = [INSTRN_BUF_BASE, INSTRN1_BUF_BASE, INSTRN2_BUF_BASE];

pub fn brisc_push_instrn<const CORE: Core>(instr: u32) {
    unsafe { (INSTR_BUF_LOOKUP[CORE as usize] as *mut u32).write_volatile(instr) }
}

pub fn push_instrn(instr: u32) {
    unsafe { (INSTR_BUF_LOOKUP[0] as *mut u32).write_volatile(instr) }
}

// TODO: Not accurate for BH
#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum DataFormat {
    Float32 = 0,
    Float16 = 1,
    Bfp8 = 2,
    Bfp4 = 3,
    Bfp2 = 11,
    Float16B = 5,
    Bfp8B = 6,
    Bfp4B = 7,
    Bfp2B = 15,
    Lf8 = 10,
    Int8 = 14,
    Int32 = 8,
    Int16 = 9,
    Tf32 = 4,
    Lf8Plus = 128,
    Uint8 = 129,

    // testMan7 = 130,
    // testMan2 = 138
}

impl DataFormat {
    pub fn width(&self) -> u32 {
        match self {
            DataFormat::Float32 => 4,
            DataFormat::Float16 => 2,
            DataFormat::Bfp8 => 1,
            DataFormat::Bfp4 => 1,
            DataFormat::Bfp2 => 1,
            DataFormat::Float16B => 2,
            DataFormat::Bfp8B => 1,
            DataFormat::Bfp4B => 1,
            DataFormat::Bfp2B => 1,
            DataFormat::Lf8 => 1,
            DataFormat::Int8 => 1,
            DataFormat::Int32 => 4,
            DataFormat::Int16 => 2,
            DataFormat::Tf32 => 4,
            DataFormat::Lf8Plus => 1,
            DataFormat::Uint8 => 1,
        }
    }
}

#[repr(u32)]
pub enum WrCfg {
    B32 = 0,
    B128 = 1,
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum Core {
    Unpack = 0,
    Math = 1,
    Pack = 2,
}

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
