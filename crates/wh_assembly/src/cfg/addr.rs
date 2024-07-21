use crate::{
    cfg_defs::{
        ADDR_MOD_AB_SEC0_SrcAIncr_ADDR32, ADDR_MOD_AB_SEC1_SrcAIncr_ADDR32,
        ADDR_MOD_AB_SEC2_SrcAIncr_ADDR32, ADDR_MOD_AB_SEC3_SrcAIncr_ADDR32,
        ADDR_MOD_DST_SEC0_DestIncr_ADDR32, ADDR_MOD_DST_SEC1_DestIncr_ADDR32,
        ADDR_MOD_DST_SEC2_DestIncr_ADDR32, ADDR_MOD_DST_SEC3_DestIncr_ADDR32,
    },
    SETC16_value,
};

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
#[repr(u32)]
pub enum AddrSel {
    Mod0 = 0,
    Mod1 = 1,
    Mod2 = 2,
    Mod3 = 3,
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct SrcValid(pub u32);

impl SrcValid {
    pub const NONE: SrcValid = SrcValid(0);
    pub const SRC_A: SrcValid = SrcValid(1);
    pub const SRC_B: SrcValid = SrcValid(2);
    pub const BOTH: SrcValid = SrcValid(3);
}

impl core::ops::BitOr for SrcValid {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[derive(core::marker::ConstParamTy, Default, PartialEq, Eq, Clone, Copy)]
pub struct SrcIncr {
    pub incr: u8,
    pub clr: bool,
    pub cr: bool,
}

impl SrcIncr {
    pub const fn val(&self) -> u8 {
        (self.incr & 0xF) | ((self.cr as u8) << 4) | ((self.clr as u8) << 5)
    }
}

#[derive(core::marker::ConstParamTy, Default, PartialEq, Eq, Clone, Copy)]
pub struct DestIncr {
    pub incr: u8,
    pub clr: bool,
    pub cr: bool,
    pub c_to_cr: bool,
}

impl DestIncr {
    pub const fn val(&self) -> u16 {
        (self.incr & 0xFF) as u16
            | ((self.cr as u16) << 8)
            | ((self.clr as u16) << 9)
            | ((self.c_to_cr as u16) << 10)
    }
}

#[derive(core::marker::ConstParamTy, Default, PartialEq, Eq, Clone, Copy)]
pub struct FidelityIncr {
    pub incr: u8,
    pub clr: bool,
}

impl FidelityIncr {
    pub const fn val(&self) -> u16 {
        ((self.incr as u16) & 0x3) | ((self.clr as u16) << 2)
    }
}

#[derive(core::marker::ConstParamTy, Default, PartialEq, Eq, Clone, Copy)]
pub struct AddrIncr {
    pub src_a: SrcIncr,
    pub src_b: SrcIncr,
    pub fidelity: FidelityIncr,
    pub dest: DestIncr,
}

impl AddrIncr {
    pub fn program<const ME: AddrIncr, const ADDR: AddrSel>() {
        const fn src_inner(me: AddrIncr, addr: AddrSel) -> u32 {
            const SRC_ADDERS: [u32; 4] = [
                ADDR_MOD_AB_SEC0_SrcAIncr_ADDR32,
                ADDR_MOD_AB_SEC1_SrcAIncr_ADDR32,
                ADDR_MOD_AB_SEC2_SrcAIncr_ADDR32,
                ADDR_MOD_AB_SEC3_SrcAIncr_ADDR32,
            ];

            SETC16_value!(
                SRC_ADDERS[addr as usize],
                me.src_a.val() as u32
                    | ((me.src_b.val() as u32) << 6)
                    | ((me.fidelity.val() as u32) << 12)
            )
        }

        const fn dst_inner(me: AddrIncr, addr: AddrSel) -> u32 {
            const DST_ADDERS: [u32; 4] = [
                ADDR_MOD_DST_SEC0_DestIncr_ADDR32,
                ADDR_MOD_DST_SEC1_DestIncr_ADDR32,
                ADDR_MOD_DST_SEC2_DestIncr_ADDR32,
                ADDR_MOD_DST_SEC3_DestIncr_ADDR32,
            ];

            SETC16_value!(DST_ADDERS[addr as usize], me.dest.val() as u32)
        }

        unsafe {
            core::arch::asm!(
                ".word ({src})",
                ".word ({dst})",
                src = const { src_inner(ME, ADDR) },
                dst = const { dst_inner(ME, ADDR) }
            );
        }
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct PackedVal {
    pub incr: u8,
    pub clr: bool,
    pub cr: bool,
}

impl PackedVal {
    pub const fn zero() -> Self {
        PackedVal {
            incr: 0,
            clr: false,
            cr: false,
        }
    }

    const fn val(&self) -> u8 {
        (self.incr & 0xF) | (self.cr as u8) << 4 | (self.clr as u8) << 5
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct ReducedVal {
    pub incr: u8,
    pub clr: bool,
}

impl ReducedVal {
    pub const fn zero() -> Self {
        ReducedVal {
            incr: 0,
            clr: false,
        }
    }

    const fn val(&self) -> u8 {
        (self.incr & 0x1) | (self.clr as u8) << 1
    }
}

#[derive(core::marker::ConstParamTy, PartialEq, Eq)]
pub struct PackedAddr {
    pub y_src: PackedVal,
    pub y_dst: PackedVal,
    pub z_src: ReducedVal,
    pub z_dst: ReducedVal,
}

impl PackedAddr {
    pub const fn zero() -> Self {
        PackedAddr {
            y_src: PackedVal::zero(),
            y_dst: PackedVal::zero(),

            z_src: ReducedVal::zero(),
            z_dst: ReducedVal::zero(),
        }
    }

    pub const fn val(&self) -> u16 {
        self.y_src.val() as u16
            | (self.y_dst.val() as u16) << 6
            | (self.z_src.val() as u16) << 12
            | (self.z_dst.val() as u16) << 14
    }
}
