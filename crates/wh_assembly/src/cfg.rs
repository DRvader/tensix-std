pub mod addr;
pub mod pc_reset;

use crate::asm;
pub use crate::cfg_defs;

static CFG_STATE_ID: u8 = 0;

pub fn cfg_rmw(cfg_addr32: u32, cfg_shamt: u32, cfg_mask: u32, val: u32) {
    let mut wrdata = val;

    // Avoid multiplication of variables!
    //const uint32_t addr = (cfg_state_id * CFG_STATE_SIZE * 4) + cfg_addr32;
    let addr = if CFG_STATE_ID == 0 {
        cfg_addr32
    } else {
        (cfg_defs::CFG_STATE_SIZE * 4) + cfg_addr32
    };

    // Declared here instead of globally to prevent direct access, which might ignore current state ID
    let cfg_base = crate::tensix_defs::TENSIX_CFG_BASE as *mut u32;

    let mut cfg_data = unsafe { cfg_base.add(addr as usize).read_volatile() };

    // Shift and mask wrdata to properly align withn 32-bit DWORD
    wrdata <<= cfg_shamt;
    wrdata &= cfg_mask;

    // Zero-out relevant bits in cfg data
    cfg_data &= !cfg_mask;

    // Or new data bits
    cfg_data |= wrdata;

    //Update cfg regs
    unsafe {
        cfg_base.add(addr as usize).write_volatile(cfg_data);
    }
}

pub fn cfg_read(cfg_addr32: u32, cfg_shamt: u32, cfg_mask: u32) -> u32 {
    // Avoid multiplication of variables!
    //const uint32_t addr = (cfg_state_id * CFG_STATE_SIZE * 4) + cfg_addr32;
    let addr = if CFG_STATE_ID == 0 {
        cfg_addr32
    } else {
        (cfg_defs::CFG_STATE_SIZE * 4) + cfg_addr32
    };

    // Declared here instead of globally to prevent direct access, which might ignore current state ID
    let cfg_base = crate::tensix_defs::TENSIX_CFG_BASE as *mut u32;

    let rd_data = unsafe { cfg_base.add(addr as usize).read_volatile() };

    (rd_data & cfg_mask) >> cfg_shamt
}

pub fn cfg_write(cfg_addr32: u32, cfg_shamt: u32, cfg_mask: u32, val: u32) {
    // Avoid multiplication of variables!
    //const uint32_t addr = (cfg_state_id * CFG_STATE_SIZE * 4) + cfg_addr32;
    let addr = if CFG_STATE_ID == 0 {
        cfg_addr32
    } else {
        (cfg_defs::CFG_STATE_SIZE * 4) + cfg_addr32
    };

    // Declared here instead of globally to prevent direct access, which might ignore current state ID
    let cfg_base = crate::tensix_defs::TENSIX_CFG_BASE as *mut u32;

    let mut wrdata = val;
    wrdata <<= cfg_shamt;
    wrdata &= cfg_mask;

    unsafe { cfg_base.add(addr as usize).write_volatile(wrdata) };
}

pub fn t6_reg_cfg_rmw(addr: u32, shamt: u32, mut mask: u32, val: u32)
{
    let mut wrval = val << shamt;

    let b_mask = mask & 0xff;
    if b_mask != 0 {
        asm::RMWCIB0d!(addr, b_mask, wrval & 0xff);
    }

    wrval >>= 8;
    mask >>= 8;

    let b_mask = mask & 0xff;
    if b_mask != 0 {
        asm::RMWCIB1d!(addr, b_mask, wrval & 0xff);
    }

    wrval >>= 8;
    mask >>= 8;

    let b_mask = mask & 0xff;
    if b_mask != 0 {
        asm::RMWCIB2d!(addr, b_mask, wrval & 0xff);
    }

    wrval >>= 8;
    mask >>= 8;

    let b_mask = mask & 0xff;
    if b_mask != 0 {
        asm::RMWCIB3d!(addr, b_mask, wrval & 0xff);
    }
}

pub fn const_t6_reg_cfg_rmw<const ADDR: u32, const SHAMT: u32, const MASK: u32, const VAL: u32>()
where
    [(); { ((VAL << SHAMT) & 0xFF) as u8 } as usize]:,
    [(); { (MASK & 0xff) as u8 } as usize]:,

    [(); { (((VAL << SHAMT) >> 8) & 0xFF) as u8 } as usize]:,
    [(); { ((MASK >> 8) & 0xff) as u8 } as usize]:,

    [(); { (((VAL << SHAMT) >> 16) & 0xFF) as u8 } as usize]:,
    [(); { ((MASK >> 16) & 0xff) as u8 } as usize]:,

    [(); { (((VAL << SHAMT) >> 24) & 0xFF) as u8 } as usize]:,
    [(); { ((MASK >> 24) & 0xff) as u8 } as usize]:,
{
    #[inline]
    pub fn outer<const ADDR: u32, const MASK: u8, const VAL: u8>() {
        if MASK != 0 {
            #[inline]
            pub const fn inner(addr: u32, mask: u8, val: u8) -> u32 {
                asm::RMWCIB0_value!(mask as u32, val as u32, addr)
            }

            unsafe {
                core::arch::asm!(".word ({value})", value = const { inner(ADDR, MASK, VAL) });
            }
        }
    }

    outer::<{ ADDR }, { (MASK & 0xff) as u8 }, { ((VAL << SHAMT) & 0xFF) as u8 }>();
    outer::<{ ADDR }, { ((MASK >> 8) & 0xff) as u8 }, { (((VAL << SHAMT) >> 8) & 0xFF) as u8 }>();
    outer::<{ ADDR }, { ((MASK >> 8) & 0xff) as u8 }, { (((VAL << SHAMT) >> 16) & 0xFF) as u8 }>();
    outer::<{ ADDR }, { ((MASK >> 8) & 0xff) as u8 }, { (((VAL << SHAMT) >> 24) & 0xFF) as u8 }>();
}
