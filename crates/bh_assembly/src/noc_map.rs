pub struct NocNode {
    pub x: u8,
    pub y: u8,
}

pub const PCIE: NocNode = NocNode { x: 11, y: 0 };

pub const ALIGNMENT_L1_READ: usize = 16;
pub const ALIGNMENT_L1_WRITE: usize = 16;
pub const ALIGNMENT_PCIE_READ: usize = 64;
pub const ALIGNMENT_PCIE_WRITE: usize = 16;
pub const ALIGNMENT_DRAM_READ: usize = 64;
pub const ALIGNMENT_DRAM_WRITE: usize = 16;

pub unsafe fn read(noc_id: u8, x: u8, y: u8, addr: u64, buf: &mut [u8]) {
    super::noc::noc_read(
        super::noc::NocCommandSel {
            instance: noc_id,
            buf: 0,
        },
        super::noc::NocAddr {
            offset: addr,
            x_end: x,
            y_end: y,
            ..Default::default()
        },
        buf,
        true,
    );
}

pub unsafe fn write(noc_id: u8, x: u8, y: u8, addr: u64, buf: &[u8]) {
    super::noc::noc_write(
        super::noc::NocCommandSel {
            instance: noc_id,
            buf: 0,
        },
        super::noc::NocAddr {
            offset: addr,
            x_end: x,
            y_end: y,
            ..Default::default()
        },
        buf,
        true,
    );
}

pub unsafe fn write32(noc_id: u8, x: u8, y: u8, addr: u64, value: u32) {
    super::noc::noc_write32(
        super::noc::NocCommandSel {
            instance: noc_id,
            buf: 0,
        },
        super::noc::NocAddr {
            offset: addr,
            x_end: x,
            y_end: y,
            ..Default::default()
        },
        value,
        true,
    );
}

pub unsafe fn pci_read(buf: &mut [u8], addr: u64) {
    let addr = 0x1000_0000_0000_0000 + addr;
    super::noc::noc_read(
        super::noc::NocCommandSel::default(),
        super::noc::NocAddr {
            offset: addr,
            x_end: PCIE.x,
            y_end: PCIE.y,
            ..Default::default()
        },
        buf,
        true,
    );
}

pub unsafe fn pci_write(buf: &[u8], addr: u64) {
    let addr = 0x1000_0000_0000_0000 + addr;
    super::noc::noc_write(
        super::noc::NocCommandSel::default(),
        super::noc::NocAddr {
            offset: addr,
            x_end: PCIE.x,
            y_end: PCIE.y,
            ..Default::default()
        },
        buf,
        true,
    );
}
