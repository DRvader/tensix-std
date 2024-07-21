pub mod pc_reset;
pub mod addr;

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
