use crate::{
    asm,
    tensix_defs::{PC1_BUF_BASE, PC2_BUF_BASE, PC_BUF_BASE},
    Core,
};

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum StallCondition {
    CFG = 1 << 13,
    MOVER = 1 << 12,
    SrcBValid = 1 << 11,
    SrcAValid = 1 << 10,
    SrcBCleared = 1 << 9,
    SrcACleared = 1 << 8,
    Math = 1 << 7,
    Packer3 = 1 << 6,
    Packer2 = 1 << 5,
    Packer1 = 1 << 4,
    Packer0 = 1 << 3,
    Unpack1 = 1 << 2,
    Unpack0 = 1 << 1,
    THCON = 1,
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Stall {
    Cfg = 1 << 8,
    Math = 1 << 7,
    THCON = 1 << 6,
    Xmov = 1 << 5,
    Xsaerch = 1 << 4,
    Unpack = 1 << 3,
    Pack = 1 << 2,
    Sync = 1 << 1,
    // Also compute?
    TDMA = 1,
}

pub fn stall<const STALL: Stall, const WAIT: StallCondition>() {
    #[inline]
    const fn inner(stall: Stall, wait: StallCondition) -> u32 {
        asm::STALLWAIT_value!(stall as u32, wait as u32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(STALL, WAIT) });
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Semaphore {
    S0 = 1,
    S1 = 1 << 1,
    S2 = 1 << 2,
    S3 = 1 << 3,
    S4 = 1 << 4,
    S5 = 1 << 5,
    S6 = 1 << 6,
    S7 = 1 << 7,
}

pub fn sem_init<const SEL: Semaphore, const INIT: u8, const MAX: u8>() {
    #[inline]
    const fn inner(sel: Semaphore, init: u8, max: u8) -> u32 {
        asm::SEMINIT_value!(max as u32, init as u32, sel as u32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(SEL, INIT, MAX) });
    }
}

// Inc
pub fn sem_post<const SEL: Semaphore>() {
    #[inline]
    const fn inner(sel: Semaphore) -> u32 {
        asm::SEMPOST_value!(sel as u32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(SEL) });
    }
}

// Dec
pub fn sem_get<const SEL: Semaphore>() {
    #[inline]
    const fn inner(sel: Semaphore) -> u32 {
        asm::SEMGET_value!(sel as u32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(SEL) });
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum WaitCondition {
    WhileZero = 1,
    WhileMax = 2,
}

pub fn sem_wait<const SEL: Semaphore, const COND: WaitCondition, const RES: Stall>() {
    #[inline]
    const fn inner(sel: Semaphore, cond: WaitCondition, resource: Stall) -> u32 {
        asm::SEMWAIT_value!(resource as u32, sel as u32, cond as u32)
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(SEL, COND, RES) });
    }
}

pub fn wait_math_semaphore() {
    sem_wait::<{ Semaphore::S1 }, { WaitCondition::WhileMax }, { Stall::Math }>();
}

pub fn set_math_semaphore() {
    stall::<{ Stall::Sync }, { StallCondition::Math }>();
    sem_post::<{ Semaphore::S1 }>();
}

pub fn packer_wait_math_semaphore() {
    sem_wait::<{ Semaphore::S1 }, { WaitCondition::WhileZero }, { Stall::TDMA }>();
}

pub fn packer_set_math_semaphore() {
    sem_get::<{ Semaphore::S1 }>();
}

const PC_BUF_LOOKUP: [u32; 3] = [PC_BUF_BASE, PC1_BUF_BASE, PC2_BUF_BASE];

pub fn brisc_mem_semaphore_read<const CORE: Core, const SEM: Semaphore>() -> u32 {
    unsafe {
        (PC_BUF_LOOKUP[CORE as usize] as *mut u32)
            .add(SEM as u32 as usize)
            .read_volatile()
    }
}

pub fn brisc_mem_semaphore_post<const CORE: Core, const SEM: Semaphore>() {
    unsafe {
        (PC_BUF_LOOKUP[CORE as usize] as *mut u32)
            .add(SEM as usize)
            .write_volatile(0);
    }
}

pub fn brisc_mem_semaphore_get<const CORE: Core, const SEM: Semaphore>() {
    unsafe {
        (PC_BUF_LOOKUP[CORE as usize] as *mut u32)
            .add(SEM as usize)
            .write_volatile(1);
    }
}

pub fn mem_semaphore_read<const SEM: Semaphore>() -> u32 {
    unsafe {
        (PC_BUF_LOOKUP[0] as *mut u32)
            .add(SEM as u32 as usize)
            .read_volatile()
    }
}

pub fn mem_semaphore_post<const SEM: Semaphore>() {
    unsafe {
        (PC_BUF_LOOKUP[0] as *mut u32)
            .add(SEM as usize)
            .write_volatile(0);
    }
}

pub fn mem_semaphore_get<const SEM: Semaphore>() {
    unsafe {
        (PC_BUF_LOOKUP[0] as *mut u32)
            .add(SEM as usize)
            .write_volatile(1);
    }
}
