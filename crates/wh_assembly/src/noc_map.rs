pub struct NocNode {
    pub x: u8,
    pub y: u8,
}

pub const PCIE: NocNode = NocNode { x: 0, y: 3 };

pub const ALIGNMENT_L1_READ: usize = 16;
pub const ALIGNMENT_L1_WRITE: usize = 16;
pub const ALIGNMENT_PCIE_READ: usize = 32;
pub const ALIGNMENT_PCIE_WRITE: usize = 16;
pub const ALIGNMENT_DRAM_READ: usize = 32;
pub const ALIGNMENT_DRAM_WRITE: usize = 16;

pub unsafe fn pci_read(buf: &mut [u8], addr: u64) {
    let addr = 0x8_0000_0000 + addr;
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
    let addr = 0x8_0000_0000 + addr;
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
