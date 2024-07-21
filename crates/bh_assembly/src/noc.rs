///  Basic NOC API
///
///  Common arguments:
///
///   * bool linked: all sequences of function calls with “linked” set to true
///     targeting the same destination will manifest on the NoC as a single
///     multi-command packet, guaranteeing they complete in-order.  For commands
///     targeting different destinations, it is not possible to provide this
///     guarantee.
///
///     Moreover, it is not possible to provide linked ordering between different
///     unicast/multicast classes of NOC virtual channels.
///
///   * unicast addresses: all unicast address arguments are given as 40-bit
///     addresses (type uint64_t):
///        - bits [31:0] = byte offset in local L1 memory,
///        - bits [35:32]/[39:36] = X/Y coordinate prefixes.
///
///     The addresses can be provided using the above macro NOC_XY_ADDR.  For
///     example, address 0x1000 in L1 of Tensix (1, 2) can be given as
///     NOC_XY_ADDR(1, 2, 0x1000).
///
///   * multicast addresses: all multicast address arguments are given as a 48-bit
///     combination of 32-bit local address and coordinates of the upper left and
///     lower right corners of the multicast rectangle.
///
///     The address can be provided using the macro NOC_MULTICAST_ADDR above.
///     For example, using NOC_MULTICAST_ADDR(1, 4, 6, 5) will multicast to
///     12 destinations, i.e. all those with X coordinates between 1 and 6 and
///     Y-coordinates between 4 and 5 (inclusive).
///
///   All addresses are in the form of byte offsets, regardless of the minimal
///   access granularity.  Address bits below the minimal access granularity are
///   ignored.

const NOC_REG_SPACE_START_ADDR: u32 = 0xFF000000;
const NOC_REGS_START_ADDR: u32 = 0xFFB20000;

const NOC_CMD_BUF_OFFSET: u32 = 0x00000800;
const NOC_CMD_BUF_OFFSET_BIT: u32 = 11;

const NOC_INSTANCE_OFFSET: u32 = 0x00010000;
const NOC_INSTANCE_OFFSET_BIT: u32 = 16;

const NOC_REG_TARG_ADDR_LO: u32 = 0x0;
const NOC_REG_TARG_ADDR_MID: u32 = 0x4;
const NOC_REG_TARG_ADDR_HI: u32 = 0x8;

const NOC_REG_RET_ADDR_LO: u32 = 0xC;
const NOC_REG_RET_ADDR_MID: u32 = 0x10;
const NOC_REG_RET_ADDR_HI: u32 = 0x14;

const NOC_REG_PACKET_TAG_OFFSET: u32 = 0x18;
const NOC_REG_CTRL: u32 = 0x1C;
const NOC_REG_AT_LEN_BE: u32 = 0x20;
const NOC_REG_AT_LEN_BE_1: u32 = 0x24;
const NOC_REG_AT_DATA: u32 = 0x28;
const NOC_BRCST_EXCLUDE: u32 = 0x2C;
const NOC_L1_ACC_AT_INSTRN: u32 = 0x30;
const NOC_SEC_CTRL: u32 = 0x34;

const NOC_REG_CMD_CTRL: u32 = 0x40;
const NOC_REG_NODE_ID: u32 = 0x44;
const NOC_ENDPOINT_ID: u32 = 0x48;

const NOC_CTRL_STATUS_READY: u32 = 0x0;

const NOC_CMD_AT: u32 = 0x1;
const NOC_CMD_CPY: u32 = 0x0 << 0;
const NOC_CMD_RD: u32 = 0x0 << 1;
const NOC_CMD_WR: u32 = 0x1 << 1;
const NOC_CMD_WR_BE: u32 = 0x1 << 2;
const NOC_CMD_WR_INLINE: u32 = 0x1 << 3;
const NOC_CMD_RESP_MARKED: u32 = 0x1 << 4;
const NOC_CMD_BRCST_PACKET: u32 = 0x1 << 5;
const NOC_CMD_VC_LINKED: u32 = 0x1 << 6;
const NOC_CMD_VC_STATIC: u32 = 0x1 << 7;
const NOC_CMD_PATH_RESERVE: u32 = 0x1 << 8;
const NOC_CMD_MEM_RD_DROP_ACK: u32 = 0x1 << 9;
macro_rules! NOC_CMD_STATIC_VC {
    ($vc:expr) => {
        (($vc) as u32) << 13
    };
}

macro_rules! NOC_CMD_BRCST_XY {
    ($y:expr) => {
        (($y) as u32) << 16
    };
}
const NOC_CMD_BRCST_SRC_INCLUDE: u32 = 0x1 << 17;
macro_rules! NOC_CMD_ARB_PRIORITY {
    ($p:expr) => {
        (($p) as u32) << 27
    };
}
const NOC_CMD_L1_ACC_AT_EN: u32 = 0x1 << 31;

const NOC_AT_INS_NOP: u32 = 0x0;
const NOC_AT_INS_INCR_GET: u32 = 0x1;
const NOC_AT_INS_INCR_GET_PTR: u32 = 0x2;
const NOC_AT_INS_SWAP: u32 = 0x3;
const NOC_AT_INS_CAS: u32 = 0x4;
const NOC_AT_INS_GET_TILE_MAP: u32 = 0x5;
const NOC_AT_INS_STORE_IND: u32 = 0x6;
const NOC_AT_INS_SWAP_4B: u32 = 0x7;
const NOC_AT_INS_ACC: u32 = 0x9;

macro_rules! NOC_AT_IND_32 {
    ($index:expr) => {
        $index
    };
}
macro_rules! NOC_AT_IND_32_SRC {
    ($index:expr) => {
        $index << 10
    };
}
macro_rules! NOC_AT_WRAP {
    ($wrap:expr) => {
        $wrap << 2
    };
}
macro_rules! NOC_AT_INS {
    ($ins:expr) => {
        $ins << 12
    };
}
macro_rules! NOC_AT_TILE_MAP_IND {
    ($ind:expr) => {
        $ins << 2
    };
}

const NOC_DATA_WIDTH: u16 = 512 + 3;
const NOC_PAYLOAD_WIDTH: u16 = 512;
const NOC_WORD_BYTES: u16 = NOC_PAYLOAD_WIDTH / 8;
const NOC_MAX_BURST_WORDS: u16 = 256;
const NOC_MAX_BURST_SIZE: u16 = NOC_MAX_BURST_WORDS * NOC_WORD_BYTES;

const NIU_SLV_POSTED_WR_REQ_STARTED: u32 = 0x3D;
const NIU_SLV_NONPOSTED_WR_REQ_STARTED: u32 = 0x3C;
const NIU_SLV_POSTED_WR_REQ_RECEIVED: u32 = 0x3B;
const NIU_SLV_NONPOSTED_WR_REQ_RECEIVED: u32 = 0x3A;
const NIU_SLV_POSTED_WR_DATA_WORD_RECEIVED: u32 = 0x39;
const NIU_SLV_NONPOSTED_WR_DATA_WORD_RECEIVED: u32 = 0x38;
const NIU_SLV_POSTED_ATOMIC_RECEIVED: u32 = 0x37;
const NIU_SLV_NONPOSTED_ATOMIC_RECEIVED: u32 = 0x36;
const NIU_SLV_RD_REQ_RECEIVED: u32 = 0x35;

const NIU_SLV_REQ_ACCEPTED: u32 = 0x34;
const NIU_SLV_RD_DATA_WORD_SENT: u32 = 0x33;
const NIU_SLV_RD_RESP_SENT: u32 = 0x32;
const NIU_SLV_WR_ACK_SENT: u32 = 0x31;
const NIU_SLV_ATOMIC_RESP_SENT: u32 = 0x30;

const NIU_MST_NONPOSTED_ATOMIC_STARTED: u32 = 0xF;
const NIU_MST_RD_REQ_STARTED: u32 = 0xE;
const NIU_MST_POSTED_WR_REQ_STARTED: u32 = 0xD;
const NIU_MST_NONPOSTED_WR_REQ_STARTED: u32 = 0xC;
const NIU_MST_POSTED_WR_REQ_SENT: u32 = 0xB;
const NIU_MST_NONPOSTED_WR_REQ_SENT: u32 = 0xA;
const NIU_MST_POSTED_WR_DATA_WORD_SENT: u32 = 0x9;
const NIU_MST_NONPOSTED_WR_DATA_WORD_SENT: u32 = 0x8;
const NIU_MST_POSTED_ATOMIC_SENT: u32 = 0x7;
const NIU_MST_NONPOSTED_ATOMIC_SENT: u32 = 0x6;
const NIU_MST_RD_REQ_SENT: u32 = 0x5;

const NIU_MST_CMD_ACCEPTED: u32 = 0x4;
const NIU_MST_RD_DATA_WORD_RECEIVED: u32 = 0x3;
const NIU_MST_RD_RESP_RECEIVED: u32 = 0x2;
const NIU_MST_WR_ACK_RECEIVED: u32 = 0x1;
const NIU_MST_ATOMIC_RESP_RECEIVED: u32 = 0x0;

macro_rules! NOC_STATUS_REG {
    ($reg:expr) => {
        ($reg as u32 * 4) + 0x200
    };
}

pub struct NocCommands(pub u32);

#[derive(Default, Clone, Copy)]
pub struct NocCommandSel {
    pub instance: u8,
    pub buf: u8,
}

#[derive(Default, Clone, Copy)]
pub struct NocAddr {
    pub offset: u64,

    pub x_end: u8,
    pub y_end: u8,

    pub x_start: u8,
    pub y_start: u8,
}

impl NocAddr {
    pub fn lower_addr(&self) -> u32 {
        self.offset as u32
    }

    pub fn mid_addr(&self) -> u32 {
        (self.offset >> 32) as u32
    }

    pub fn upper_addr(&self) -> u32 {
        (self.x_end as u32)
            | ((self.y_end as u32) << 6)
            | ((self.x_start as u32) << 12)
            | ((self.y_start as u32) << 18)
    }

    pub fn upper_uaddr(&self) -> u32 {
        (self.x_end as u32) | ((self.y_end as u32) << 6)
    }

    pub fn local() -> Self {
        let addr = get_local_addr();
        Self {
            x_end: addr.0,
            y_end: addr.1,
            ..Default::default()
        }
    }
}

pub fn wr_cfg(sel: NocCommandSel, addr: u32, value: u32) {
    unsafe {
        ((NOC_REGS_START_ADDR
            + (sel.instance as u32 * NOC_INSTANCE_OFFSET)
            + (sel.buf as u32 * NOC_CMD_BUF_OFFSET)
            + addr as u32) as *mut u32)
            .write_volatile(value)
    };
}

pub fn rd_cfg(sel: NocCommandSel, addr: u32) -> u32 {
    unsafe {
        ((NOC_REGS_START_ADDR
            + (sel.instance as u32 * NOC_INSTANCE_OFFSET)
            + (sel.buf as u32 * NOC_CMD_BUF_OFFSET)
            + addr as u32) as *mut u32)
            .read_volatile()
    }
}

pub fn get_local_addr() -> (u8, u8) {
    let id = rd_cfg(NocCommandSel::default(), NOC_REG_NODE_ID);

    let local_x = id & ((1 << 6) - 1);
    let local_y = (id >> 6) & ((1 << 6) - 1);

    (local_x as u8, local_y as u8)
}

fn noc_command_ready(sel: NocCommandSel) -> bool {
    rd_cfg(sel, NOC_REG_CMD_CTRL) == NOC_CTRL_STATUS_READY
}

#[derive(Default, Clone, Copy)]
pub enum NocRW {
    #[default]
    Read,
    Write,
}

pub enum LenOrBE {
    Len(u16),
    BE(u32),
}

impl LenOrBE {
    pub fn to_cmd(&self) -> u32 {
        match self {
            Self::Len(_) => 0,
            Self::BE(_) => NOC_CMD_WR_BE,
        }
    }

    pub fn to_value(&self) -> u32 {
        match self {
            Self::Len(value) => *value as u32,
            Self::BE(value) => *value,
        }
    }
}

#[derive(Default, Clone)]
pub struct NocCopyCtrl {
    /// link with previous call for ordering
    pub linked: bool,
    /// if copying from a local address, avoid sending ack on the
    /// response channel
    pub posted: bool,
    /// use VC 0/1 for static request; Some to use static-vc
    pub static_vc: Option<u32>,
    pub mcast: bool,
    pub mcast_mode: u32,
    /// arbitration priority for VC allocation;
    /// set to 0 disable arbitration priority & use round-robin always
    pub vc_arb_priority: u32,

    pub direction: NocRW,
}

impl NocCopyCtrl {
    pub fn to_cmd(&self) -> u32 {
        let mut cmd = 0;
        if self.linked {
            cmd |= NOC_CMD_VC_LINKED;
        }

        if let Some(static_vc) = self.static_vc {
            cmd |= NOC_CMD_VC_STATIC;
            cmd |= NOC_CMD_STATIC_VC!(static_vc);
        }

        cmd |= NOC_CMD_CPY;
        cmd |= match self.direction {
            NocRW::Read => NOC_CMD_RD,
            NocRW::Write => NOC_CMD_WR,
        };

        if !self.posted {
            cmd |= NOC_CMD_RESP_MARKED;
        }

        if self.mcast {
            cmd |= NOC_CMD_PATH_RESERVE;
            cmd |= NOC_CMD_BRCST_PACKET;
            cmd |= NOC_CMD_BRCST_XY!(self.mcast_mode);
        } else {
            cmd |= NOC_CMD_ARB_PRIORITY!(self.vc_arb_priority);
        }

        cmd
    }
}

pub fn send_non_atomic_packet(
    sel: NocCommandSel,
    src: NocAddr,
    dest: NocAddr,
    len: LenOrBE,
    ctrl: NocCopyCtrl,
) {
    match len {
        LenOrBE::Len(len) if len == 0 => {
            return;
        }
        _ => {}
    }

    loop {
        if noc_command_ready(sel) {
            break;
        }
    }

    wr_cfg(sel, NOC_REG_TARG_ADDR_LO, src.lower_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_MID, src.mid_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_HI, src.upper_addr());

    wr_cfg(sel, NOC_REG_RET_ADDR_LO, dest.lower_addr());
    wr_cfg(sel, NOC_REG_RET_ADDR_MID, dest.mid_addr());
    wr_cfg(sel, NOC_REG_RET_ADDR_HI, dest.upper_addr());

    wr_cfg(sel, NOC_REG_AT_LEN_BE, len.to_value());
    wr_cfg(sel, NOC_REG_AT_LEN_BE_1, 0);
    wr_cfg(sel, NOC_REG_PACKET_TAG_OFFSET, 0);

    wr_cfg(sel, NOC_REG_CTRL, ctrl.to_cmd() | len.to_cmd());

    // Trigger NOC
    wr_cfg(sel, NOC_REG_CMD_CTRL, 1);
}

fn send_non_atomic_packets(
    sel: NocCommandSel,
    mut src: NocAddr,
    mut dest: NocAddr,
    len: u16,
    ctrl: NocCopyCtrl,
) -> u32 {
    let mut sent_packets = 0;

    let mut size = len as i32;
    while size > NOC_MAX_BURST_SIZE as i32 {
        send_non_atomic_packet(
            sel,
            src,
            dest,
            LenOrBE::Len(NOC_MAX_BURST_SIZE),
            ctrl.clone(),
        );
        src.offset += NOC_MAX_BURST_SIZE as u64;
        dest.offset += NOC_MAX_BURST_SIZE as u64;
        size -= NOC_MAX_BURST_SIZE as i32;

        sent_packets += 1;
    }
    if size > 0 {
        sent_packets += 1;
        send_non_atomic_packet(sel, src, dest, LenOrBE::Len(size as u16), ctrl.clone());
    }

    sent_packets
}

///  Copy data from source to destination address.  Supports narrow transfers
///  (size not a multiple of 16 bytes).  However, the alignment of source and
///  destination start addresses (i.e., bits [3:0]) must be identical.  If the
///  alignment is not identical, the one from destination address is assumed.
///
///  If copying from local memory to a remote destination, set posted=false to
///  request an ack that will increment the NIU_MST_WR_ACK_RECEIVED counter.
///  This value can be compared with NIU_MST_NONPOSTED_WR_REQ_STARTED to ensure
///  the writes are flushed.  (Note that copying more than NOC_MAX_BURST_SIZE
///  triggers multiple underlying NOC requests.)
///
///  If src_addr is remote, the request is always posted and the parameter is
///  ignored.
///
///  <src_addr> => source address (unicast)
///  <dst_addr> => destination address (unicast)
///  <size> => number of bytes to copy
pub fn noc_copy(sel: NocCommandSel, src: NocAddr, dest: NocAddr, len: u16, mut ctrl: NocCopyCtrl) {
    let local_addr = get_local_addr();
    let src_local = src.x_end == local_addr.0 && src.y_end == local_addr.1;
    if !src_local {
        ctrl.posted = true;
    }

    send_non_atomic_packets(sel, src, dest, len, ctrl);
}

pub fn noc_read(sel: NocCommandSel, remote: NocAddr, buf: &mut [u8], wait: bool) {
    let local = NocAddr {
        offset: buf.as_mut_ptr() as u64,
        ..NocAddr::local()
    };

    let pre = rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_RD_RESP_RECEIVED));

    let sent_packets = send_non_atomic_packets(
        sel,
        remote,
        local,
        buf.len() as u16,
        NocCopyCtrl {
            direction: NocRW::Read,
            posted: true,
            ..Default::default()
        },
    );

    if wait {
        while rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_RD_RESP_RECEIVED)) < pre + sent_packets {}
    }
}

pub fn noc_write(sel: NocCommandSel, remote: NocAddr, buf: &[u8], wait: bool) {
    let local = NocAddr {
        offset: buf.as_ptr() as u64,
        ..NocAddr::local()
    };

    let pre = rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_WR_ACK_RECEIVED));

    let sent_packets = send_non_atomic_packets(
        sel,
        local,
        remote,
        buf.len() as u16,
        NocCopyCtrl {
            direction: NocRW::Write,
            posted: false,
            ..Default::default()
        },
    );

    if wait {
        while rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_WR_ACK_RECEIVED)) < pre + sent_packets {}
    }
}

fn noc_inline_transfer(sel: NocCommandSel, dest: NocAddr, value: u32, be: u32, ctrl: NocCopyCtrl) {
    loop {
        if noc_command_ready(sel) {
            break;
        }
    }

    wr_cfg(sel, NOC_REG_TARG_ADDR_LO, dest.lower_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_MID, dest.upper_addr());

    // The actual noc transfer is 256 bits, so we need to adjust the be
    // to line up to where the 32bit value will appear within that.
    wr_cfg(
        sel,
        NOC_REG_AT_LEN_BE,
        be << ((dest.offset as u32) & (NOC_WORD_BYTES as u32 - 1)),
    );
    wr_cfg(sel, NOC_REG_AT_DATA, value);

    wr_cfg(sel, NOC_REG_CTRL, ctrl.to_cmd() | NOC_CMD_WR_INLINE);

    // Trigger NOC
    wr_cfg(sel, NOC_REG_CMD_CTRL, 1);
}

pub fn noc_write32(sel: NocCommandSel, remote: NocAddr, value: u32, wait: bool) {
    noc_inline_transfer(
        sel,
        remote,
        value,
        0xF,
        NocCopyCtrl {
            direction: NocRW::Write,
            posted: false,
            static_vc: Some(1),
            ..Default::default()
        },
    );

    if wait {
        while write_ack_count(sel) < rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_NONPOSTED_WR_REQ_STARTED))
        {
        }
    }
}

pub fn noc_write_be(sel: NocCommandSel, addr: u64, remote: NocAddr, be: u32, wait: bool) {
    loop {
        if noc_command_ready(sel) {
            break;
        }
    }

    let local = NocAddr {
        offset: addr,
        ..NocAddr::local()
    };

    wr_cfg(sel, NOC_REG_TARG_ADDR_LO, local.lower_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_MID, local.upper_addr());

    wr_cfg(sel, NOC_REG_RET_ADDR_LO, remote.lower_addr());
    wr_cfg(sel, NOC_REG_RET_ADDR_MID, remote.upper_addr());

    wr_cfg(sel, NOC_REG_AT_LEN_BE, be);

    wr_cfg(
        sel,
        NOC_REG_CTRL,
        NocCopyCtrl {
            direction: NocRW::Write,
            posted: false,
            ..Default::default()
        }
        .to_cmd()
            | NOC_CMD_WR_BE,
    );

    wr_cfg(sel, NOC_REG_CMD_CTRL, 1);

    if wait {
        while write_ack_count(sel) < rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_NONPOSTED_WR_REQ_STARTED))
        {
        }
    }
}

pub fn noc_atomic_incr(
    sel: NocCommandSel,
    remote: NocAddr,
    local: u64,
    incr: u32,
    wrap: u32,
    wait: bool,
) {
    let local = NocAddr {
        offset: local,
        ..NocAddr::local()
    };

    atomic_read_and_increment(sel, local, remote, incr, wrap, false);

    if wait {
        while rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_ATOMIC_RESP_RECEIVED))
            < rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_NONPOSTED_ATOMIC_STARTED))
        {}
    }
}

///  Copy a single word with byte-enables from source to destination address.
///  Works similar to noc_copy, except limited to a single-word transfer and
///  provides the option to specify partial byte-enables.
///
///  This call works only with transfers from local memory.
///
///  <src_addr> => source address (unicast, must be local memory)
///  <dst_addr> => destination address (unicast)
///  <be> => byte enable mask
///  <linked> => link with previous call for ordering
///  <posted> => if copying from a local address, avoid sending ack on the
///              response channel
///  <static_vc_alloc> => use static VC allocation
///  <static_vc> => use VC 0/1 for static request; don't-care if static_vc_alloc=0
// void noc_copy_word_be(uint64_t src_addr, uint64_t dst_addr, uint32_t be, bool linked, bool posted, bool static_vc_alloc, uint32_t static_vc);

///  Write a single 32-bit value using inline header data. (The effect is the
///  same as when writing with noc_copy, however such writes save the bandwidth
///  of an additional flit for register access.)
///
///  <dst_addr> => destination address (unicast)
///  <be> => byte enable mask
///  <linked> => link with previous call for ordering
///  <posted> => if copying from a local address, avoid sending ack on the
///              response channel
///  <static_vc_alloc> => use static VC allocation
///  <static_vc> => use VC 0/1 for static request; don't-care if static_vc_alloc=0
// void noc_write_dw_inline(uint64_t dst_addr, uint32_t val, uint8_t be, bool linked, bool posted, bool static_vc_alloc, uint32_t static_vc);

///  Copy data from source to multiple destinations via multicast.  Supports
///  narrow transfers (size not a multiple of 16 bytes).  However, the alignment
///  of source and destination start addresses (i.e., bits [3:0]) must be identical.
///  If the alignment is not identical, the one from destination address is assumed.
///
///  If copying from local memory and posted=false, a separate ack is received from
///  each destination.
///
///  <src_addr> => source address (unicast)
///  <dst_addr> => destination address (multicast)
///  <multicast_mode> => multicast direction (0 = x-direction, 1 = y-direction)
///  <size> => number of bytes to copy
///  <linked> => link with previous call for ordering
///  <posted> => if copying from a local address, avoid sending ack on the
///              response channel
///  <static_vc_alloc> => use static VC allocation
///  <static_vc> => use VC 0/1 for static request; don't-care if static_vc_alloc=0
// void noc_write_dw_inline(uint64_t dst_addr, uint32_t val, uint8_t be, bool linked, bool posted, bool static_vc_alloc, uint32_t static_vc);

///  Multicast version of noc_copy_word_be.
///
///  Like noc_copy_word_be, this call works only with transfers from local memory,
///  and is limited to single-word transfers.
///
///  <src_addr> => source address (unicast)
///  <dst_addr> => destination address (multicast)
///  <multicast_mode> => multicast direction (0 = x-direction, 1 = y-direction)
///  <be> => byte enable mask
///  <linked> => link with previous call for ordering
///  <posted> => if copying from a local address, avoid sending ack on the
///              response channel
///  <static_vc_alloc> => use static VC allocation
///  <static_vc> => use VC 0/1 for static request; don't-care if static_vc_alloc=0
// void noc_multicast_copy_word_be(uint64_t src_addr, uint64_t dst_addr, uint32_t multicast_mode, uint32_t be, bool linked, bool posted, bool static_vc_alloc, uint32_t static_vc);

///  Multicast version of noc_write_dw_inline.
///
///  <dst_addr> => destination address (unicast)
///  <multicast_mode> => multicast direction (0 = x-direction, 1 = y-direction)
///  <be> => byte enable mask
///  <linked> => link with previous call for ordering
///  <posted> => if copying from a local address, avoid sending ack on the
///              response channel
///  <static_vc_alloc> => use static VC allocation
// void noc_multicast_write_dw_inline(uint64_t dst_addr, uint32_t val, uint32_t multicast_mode, uint8_t be, bool linked, bool posted, bool static_vc_alloc, uint32_t static_vc);

///  Atomic wrapping increment of 32-bit value at destination address.  The address has
///  4-byte granularity.  The increment result wraps around the address aligned relative
///  to the specified wrapping size.   Increment is an arbitrary value, while wrapping
///  limit is calculated from the given argument as 2^(<wrap>+1).  (Therefore, for 32-bit
///  values, setting <wrap>=31 implies no wrapping except the 32-bit integer maximum.)
///
///  For example, if:
///      wrap = 7 (wrap to 0x100),
///      incr = 0x80 (increase by 0x80),
///      current value = 0x21C0,
///
///  then the aligned valud is 0x2100, and the new value is:
///      0x2100 + ((0x1C0 + 0x80) % 0x100) = 0x2140.
///
///  <addr> => counter address (unicast)
///  <incr> => increment
///  <wrap> => log2(wrapping limit)-1
///  <linked> => link with previous call for ordering
fn atomic_increment(sel: NocCommandSel, dest: NocAddr, incr: u32, wrap: u32, linked: bool) {
    while !noc_command_ready(sel) {}

    wr_cfg(sel, NOC_REG_TARG_ADDR_LO, dest.lower_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_MID, dest.upper_addr());

    wr_cfg(
        sel,
        NOC_REG_CTRL,
        if linked { NOC_CMD_VC_LINKED } else { 0 } | NOC_CMD_AT,
    );
    wr_cfg(
        sel,
        NOC_REG_AT_LEN_BE,
        NOC_AT_INS!(NOC_AT_INS_INCR_GET)
            | NOC_AT_WRAP!(wrap)
            | NOC_AT_IND_32!(((dest.offset as u32) >> 2) & 0x3)
            | NOC_AT_IND_32_SRC!(0),
    );

    wr_cfg(sel, NOC_REG_AT_DATA, incr);
    wr_cfg(sel, NOC_REG_CMD_CTRL, 1);
}

///  Performs the same operation as noc_atomic_increment and reads the previous value from the
///  destination address to <read_addr>.  The <read_addr> address also has 4-byte granularity,
///  and the return value updates only the corresponding 32 bits in local memory.
///
///  There is no alignment requirement between <addr> and <read_addr>.
///
///  This function can be used to reserve space in a remote buffer by operating on the write
///  pointer.
///
///  The status of the returned read can be determined by calling noc_atomic_read_updates_completed
///  (see below).
///
///  <addr> => counter address (unicast)
///  <incr> => increment
///  <wrap> => log2(wrapping limit)-1
///  <read_addr> => address to store the previous value
///  <linked> => link with previous call for ordering
pub fn atomic_read_and_increment(
    sel: NocCommandSel,
    local: NocAddr,
    remote: NocAddr,
    incr: u32,
    wrap: u32,
    linked: bool,
) {
    while !noc_command_ready(sel) {}

    wr_cfg(sel, NOC_REG_TARG_ADDR_LO, remote.lower_addr());
    wr_cfg(sel, NOC_REG_TARG_ADDR_MID, remote.upper_addr());

    wr_cfg(sel, NOC_REG_RET_ADDR_LO, local.lower_addr());
    wr_cfg(sel, NOC_REG_RET_ADDR_MID, local.upper_addr());

    wr_cfg(
        sel,
        NOC_REG_CTRL,
        if linked { NOC_CMD_VC_LINKED } else { 0 } | NOC_CMD_AT | NOC_CMD_RESP_MARKED,
    );

    wr_cfg(
        sel,
        NOC_REG_AT_LEN_BE,
        NOC_AT_INS!(NOC_AT_INS_INCR_GET)
            | NOC_AT_WRAP!(wrap)
            | NOC_AT_IND_32!(((remote.offset as u32) >> 2) & 0x3)
            | NOC_AT_IND_32_SRC!(0),
    );

    wr_cfg(sel, NOC_REG_AT_DATA, incr);
    wr_cfg(sel, NOC_REG_CMD_CTRL, 1);
}

///  Performs the same operation as noc_atomic_increment on multiple multicast destinations.
///
///  <addr> => counter address (multicast)
///  <multicast_mode> => multicast direction (0 = x-direction, 1 = y-direction)
///  <incr> => increment
///  <wrap> => log2(wrapping limit)-1
///  <linked> => link with previous call for ordering
// void noc_multicast_atomic_increment(uint64_t addr, uint32_t multicast_mode, uint32_t incr, uint32_t wrap, bool linked);

///  Performs the same operation as noc_atomic_read_and_increment on multiple multicast destinations.
///
///  Each destination returns the previous value, and the final value written to read_addr is undefined,
///  depending on the order in which updates are delivered.  Therefore, the 32-bit value at this address
///  must be reserved to be modified by this call, but its final value should be ignored.
///
///  The value returned by noc_atomic_read_updates_completed will increment with each returned response.
///  The intended use case for this function is to perform atomic increments at multiple destinations, and
///  subsequently call noc_atomic_read_updates_completed to ensure all the updates have completed.
///
///  <addr> => counter address (multicast)
///  <multicast_mode> => multicast direction (0 = x-direction, 1 = y-direction)
///  <incr> => increment
///  <wrap> => log2(wrapping limit)-1
///  <read_addr> => address to store the previous value
///  <linked> => link with previous call for ordering
// void noc_multicast_atomic_read_and_increment(uint64_t addr, uint32_t multicast_mode, uint32_t incr, uint32_t wrap, uint64_t read_addr, bool linked);

///  Returns ID & dateline info of the local node in the format:
///     {10'b0, i_dateline_node_y[0:0], i_dateline_node_x[0:0],
///     i_local_nocid[3:0],
///     i_noc_y_size[3:0], i_noc_x_size[3:0],
///     i_local_nodeid_y[3:0], i_local_nodeid_x[3:0]}
// uint32_t noc_local_node_id();

pub fn read_response_count(sel: NocCommandSel) -> u32 {
    rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_RD_RESP_RECEIVED))
}

pub fn write_ack_count(sel: NocCommandSel) -> u32 {
    rd_cfg(sel, NOC_STATUS_REG!(NIU_MST_WR_ACK_RECEIVED))
}

//////////////////////////////////////////////////////////////////
//////////////////////// ECC Functions ///////////////////////////
//////////////////////////////////////////////////////////////////

///  Allows for the enabling/disabling of ECC features in the NIU and Routers
///  Enabling full ECC is a two stage process. First you must call noc_ecc_cfg_stage_1 for all tensix, sync (ensuring all writes went through),
///  and then call noc_ecc_cfg_stage_2 for all tensix.
// void noc_ecc_cfg_stage_1(bool header_ckh_bits_en);

///  Allows for the enabling/disabling of ECC features in the NIU and Routers
///  Enabling full ECC is a two stage process. First you must call noc_ecc_cfg_stage_1 for all tensix, sync (ensuring all writes went through),
///  and then call noc_ecc_cfg_stage_2 for all tensix.
// void noc_ecc_cfg_stage_2(bool niu_mem_parity_en, bool router_mem_parity_en, bool header_secded_en, bool mem_parity_int_en, bool header_sec_int_en, bool header_ded_int_en);

/// Clears the corresponding ECC error interrupt and number of errors register
// void noc_ecc_clear_err(bool clear_mem_parity_err, bool clear_header_sec, bool clear_header_ded);

/// Increments the corresponding number of errors register by 1.
/// Debug use only.
// void noc_ecc_force_err(bool force_mem_parity_err, bool force_header_sec, bool force_header_ded);

/// Gets the number of memory parity errors. This is the sum of the number of parity errors in the router and niu memories (if enabled in noc_ecc_cfg()). This register indicates a fatal error in the system.
// uint32_t noc_ecc_get_num_mem_parity_errs();

/// Gets the number of single errors that were corrected in the header. This register should be treated as a warning of system instability.
// uint32_t noc_ecc_get_num_header_sec();

/// Gets the number of double errors detected in the header. This register indicates a fatal error in the system.
// uint32_t noc_ecc_get_num_header_ded();
fn dead() {}
