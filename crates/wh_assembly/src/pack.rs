use crate::{
    asm, cfg::{cfg_rmw, cfg_write}, cfg_defs::{
        PCK0_ADDR_CTRL_XY_REG_0_Xstride_ADDR32, PCK0_ADDR_CTRL_XY_REG_0_Xstride_SHAMT,
        PCK0_ADDR_CTRL_XY_REG_0_Ystride_SHAMT, PCK0_ADDR_CTRL_ZW_REG_0_Wstride_SHAMT,
        PCK0_ADDR_CTRL_ZW_REG_0_Zstride_ADDR32, PCK0_ADDR_CTRL_ZW_REG_0_Zstride_SHAMT,
        PCK_DEST_RD_CTRL_Read_32b_data_ADDR32, PCK_DEST_RD_CTRL_Read_32b_data_MASK,
        PCK_DEST_RD_CTRL_Read_32b_data_SHAMT, THCON_SEC0_REG1_Add_l1_dest_addr_offset_ADDR32,
        THCON_SEC0_REG1_Disable_zero_compress_ADDR32, THCON_SEC0_REG1_L1_Dest_addr_ADDR32,
        THCON_SEC0_REG1_Row_start_section_size_MASK,
    }, nop, tensix_defs::{R0, R0_HI, R0_LO}, unpack::BlockSel, Core, DataFormat, SETADCXXd, SETADCXYd, SETADCZWd, SETDMAREG_value, SETDMAREGd, WRCFGd, WrCfg, SETDMAREG, WRCFG
};

mod hw_workarounds;

#[inline]
pub fn cfg_pack0_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC0_REG1_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack1_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC0_REG8_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack2_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC1_REG1_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[inline]
pub fn cfg_pack3_dest<const REG: u32>() {
    #[inline]
    pub const fn inner(reg: u32) -> u32 {
        asm::WRCFG_value!(
            reg,
            0, // 32 bit write
            crate::cfg_defs::THCON_SEC1_REG8_L1_Dest_addr_ADDR32
        )
    }

    unsafe {
        core::arch::asm!(".word ({value})", value = const { inner(REG) });
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct PackSel(u32);

impl PackSel {
    pub const PACK_1: PackSel = PackSel(0);
    pub const PACK_2_LOW: PackSel = PackSel(0x3);
    pub const PACK_2_HIGH: PackSel = PackSel(0xC);
    pub const PACK_4: PackSel = PackSel(0xF);

    pub const PACKER_0: PackSel = PackSel(0b0001);
    pub const PACKER_1: PackSel = PackSel(0b0010);
    pub const PACKER_2: PackSel = PackSel(0b0100);
    pub const PACKER_3: PackSel = PackSel(0b1000);
}

impl core::ops::BitOr for PackSel {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// typedef struct {
//     //word 0
//     uint32_t row_ptr_section_size : 16;
//     uint32_t exp_section_size : 16;
//     //word 1
//     uint32_t l1_dest_addr: 32;
//     //word 2
//     uint32_t uncompress  : 1;
//     uint32_t add_l1_dest_addr_offset  : 1;
//     uint32_t reserved_0  : 2;
//     uint32_t out_data_format  : 4;
//     uint32_t in_data_format  : 4;
//     uint32_t reserved_1  : 4;
//     uint32_t src_if_sel  : 1;
//     uint32_t pack_per_xy_plane  : 7;
//     uint32_t l1_src_addr : 8;
//     //word 3
//     uint32_t downsample_mask : 16;
//     uint32_t downsample_shift_count  : 3;
//     uint32_t read_mode : 1;
//     uint32_t exp_threshold_en  : 1;
//     uint32_t pack_l1_acc_disable_pack_zero_flag : 2;
//     uint32_t reserved_2 : 1;
//     uint32_t exp_threshold : 8;
// } pack_config_t;

pub fn write_descriptor(
    l1_dest: u32,
    disable_zero_compress: bool,
    in_data_format: DataFormat,
    out_data_format: DataFormat,
) {
    cfg_write(
        crate::cfg_defs::THCON_SEC0_REG1_Row_start_section_size_ADDR32,
        0,
        0xFFFFFFFF,
        0,
    );
    cfg_write(THCON_SEC0_REG1_L1_Dest_addr_ADDR32, 0, 0xFFFFFFFF, l1_dest);
    cfg_write(
        THCON_SEC0_REG1_Disable_zero_compress_ADDR32,
        0,
        0xFFFFFFFF,
        disable_zero_compress as u32
            | ((out_data_format as u32) << 4)
            | ((in_data_format as u32) << 8),
    );

    cfg_rmw(
        PCK_DEST_RD_CTRL_Read_32b_data_ADDR32,
        PCK_DEST_RD_CTRL_Read_32b_data_SHAMT,
        PCK_DEST_RD_CTRL_Read_32b_data_MASK,
        if in_data_format.width() > 16 { 1 } else { 0 },
    );
}

pub fn set_pack_range(pck: core::ops::Range<u32>) {
    asm::SETADCXXd!(BlockSel::PACKER0.0, pck.end.saturating_sub(1), pck.start);
}

macro_rules! cfg_write {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::cfg_write($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
        }
    };
}

macro_rules! cfg_rmw {
    ($name:ident, $val:expr) => {
        paste::paste! {
            $crate::cfg::cfg_rmw($crate::cfg_defs::[<$name _ADDR32>], $crate::cfg_defs::[<$name _SHAMT>], $crate::cfg_defs::[<$name _MASK>], $val)
        }
    };
}

// typedef struct {
//     //word 0
//     uint32_t row_ptr_section_size : 16;
//     uint32_t exp_section_size : 16;
//     //word 1
//     uint32_t l1_dest_addr: 32;
//     //word 2
//     uint32_t uncompress  : 1;
//     uint32_t add_l1_dest_addr_offset  : 1;
//     uint32_t reserved_0  : 2;
//     uint32_t out_data_format  : 4;
//     uint32_t in_data_format  : 4;
//     uint32_t reserved_1  : 4;
//     uint32_t src_if_sel  : 1;
//     uint32_t pack_per_xy_plane  : 7;
//     uint32_t l1_src_addr : 8;
//     //word 3
//     uint32_t downsample_mask : 16;
//     uint32_t downsample_shift_count  : 3;
//     uint32_t read_mode : 1;
//     uint32_t exp_threshold_en  : 1;
//     uint32_t pack_l1_acc_disable_pack_zero_flag : 2;
//     uint32_t reserved_2 : 1;
//     uint32_t exp_threshold : 8;
// } pack_config_t;

pub struct PackSettings {
    pub dst_addr: u32,
    pub rows_per_pack: u16,

    // Should match Math output settings?
    pub dst_format: DataFormat,

    pub l1_addr: u32,
    pub l1_format: DataFormat,
    pub compress: bool,
    pub header: bool,

    pub x_dim: u16,
    pub y_stride: u16,
    pub z_stride: u16,
    pub w_stride: u16,
}

pub fn reset_counters() {
    // Ch1 x and Ch0 x are actually endx and startx
    SETADCXYd!(0b100, 0, 0, 0, 0, 0b1010);
    SETADCZWd!(0b100, 0, 0, 0, 0, 0b1111);
}

pub fn configure(pack: PackSettings) {
    if pack.compress {
        // Setting this to 1 would disable 0 compress in the unpacker?
        // THCON_SEC0_REG1_Disable_zero_compress_ADDR32
    }

    // No idea why this is needed
    cfg_rmw!(PCK0_ADDR_BASE_REG_0_Base, 0xFFFF);

    reset_counters();

    // Set strides
    SETDMAREGd!(0, hw_workarounds::data_fmt_mult(pack.dst_format), 0, R0_LO);
    SETDMAREGd!(
        0,
        ((pack.y_stride as u32) << PCK0_ADDR_CTRL_XY_REG_0_Ystride_SHAMT) >> 16,
        0,
        R0_HI
    );
    WRCFGd!(
        0,
        crate::WrCfg::B32 as u32,
        PCK0_ADDR_CTRL_XY_REG_0_Xstride_ADDR32
    );

    SETDMAREGd!(
        0,
        ((pack.z_stride as u32) << PCK0_ADDR_CTRL_ZW_REG_0_Zstride_SHAMT) & 0xFFFF,
        0,
        R0_LO
    );
    SETDMAREGd!(
        0,
        ((pack.w_stride as u32) << PCK0_ADDR_CTRL_ZW_REG_0_Wstride_SHAMT) & 0xFFFF,
        0,
        R0_HI
    );
    WRCFGd!(
        0,
        crate::WrCfg::B32 as u32,
        PCK0_ADDR_CTRL_ZW_REG_0_Zstride_ADDR32
    );

    // Set input addr for all packers
    let addr = pack.l1_addr / 16 - if pack.header { 0 } else { 1 };
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC0_Offset, addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC1_Offset, addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC2_Offset, addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC3_Offset, addr);

    // Set dst addr for all packers
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC0_Offset, pack.dst_addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC1_Offset, pack.dst_addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC2_Offset, pack.dst_addr);
    cfg_write!(DEST_TARGET_REG_CFG_PACK_SEC3_Offset, pack.dst_addr);

    // Set rows per read
    cfg_rmw!(
        PACK_COUNTERS_SEC0_pack_xys_per_tile,
        pack.rows_per_pack as u32
    );
    cfg_rmw!(
        PACK_COUNTERS_SEC1_pack_xys_per_tile,
        pack.rows_per_pack as u32
    );
    cfg_rmw!(
        PACK_COUNTERS_SEC2_pack_xys_per_tile,
        pack.rows_per_pack as u32
    );
    cfg_rmw!(
        PACK_COUNTERS_SEC3_pack_xys_per_tile,
        pack.rows_per_pack as u32
    );

    cfg_rmw!(THCON_SEC0_REG1_In_data_format, pack.dst_format as u32);
    cfg_rmw!(THCON_SEC0_REG1_Out_data_format, pack.l1_format as u32);

    cfg_rmw!(THCON_SEC1_REG1_In_data_format, pack.dst_format as u32);
    cfg_rmw!(THCON_SEC1_REG1_Out_data_format, pack.l1_format as u32);

    cfg_rmw!(THCON_SEC0_REG8_In_data_format, pack.dst_format as u32);
    cfg_rmw!(THCON_SEC0_REG8_Out_data_format, pack.l1_format as u32);

    cfg_rmw!(THCON_SEC1_REG8_In_data_format, pack.dst_format as u32);
    cfg_rmw!(THCON_SEC1_REG8_Out_data_format, pack.l1_format as u32);

    SETADCXXd!(
        BlockSel::PACKER0.0,
        hw_workarounds::tile_size(pack.l1_format, pack.x_dim),
        0
    );
}

#[inline]
pub fn pack<
    const ADDR_MODE: super::cfg::addr::PackedAddr,
    const PACKERS: PackSel,
    const LAST: bool,
>() {
    #[inline]
    pub const fn inner(
        addr_mode: super::cfg::addr::PackedAddr,
        zero_write: bool,
        packers: PackSel,
        ovrd_thread_id: bool,
        concat: bool,
        // What does this do?
        flush: bool,
        last: bool,
    ) -> u32 {
        asm::PACR_value!(
            addr_mode.val() as u32,
            zero_write as u32,
            packers.0,
            ovrd_thread_id as u32,
            concat as u32,
            flush as u32,
            last as u32
        )
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(ADDR_MODE, true, PACKERS, false, false, false, LAST) }
        );
    }
}
