//
// Auto-generated file, do not modify!
//

#![allow(non_snake_case)]
#![allow(unused_imports)]

#[macro_export]
macro_rules! tt_op {
    ($opcode:literal) => {
        $crate::tt_op!($opcode, 0)
    };
    ($opcode:literal, $params:expr) => {
        ($opcode << 24) + $params
    };
}
#[macro_export]
macro_rules! instruction_word {
    ($asm_value:expr) => {
        // Drop 32 bits into the instruction stream.
        const value: u32 = const { $asm_value };
        unsafe { core::arch::asm!(".word ({value})", value = const value) }
    };
}
#[macro_export]
macro_rules! trisc_op_swizzle {
    ($x:expr) => {
        // Put top 2 bits, which are currently never 'b11 to bottom, indicating to Risc that they are not risc instructions
        ((($x) >> 30) & 0x3) | ((($x) & 0x3FFFFFFF) << 2)
    };
}

pub fn is_valid(val: u32, wid: u8) -> bool {
    let mask = (1 << wid) - 1;
    (val & mask) == val
}


#[macro_export]
macro_rules! ADDDMAREG_value {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x58, ($OpBisConst << 23) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! ADDDMAREG_valid {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(ResultRegIndex, 11) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! ADDDMAREG {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ADDDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! ADDDMAREGd {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::ADDDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_ADDDMAREGd {
    ($core:path, $OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ADDDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use ADDDMAREG_value;
pub use ADDDMAREG_valid;
pub use ADDDMAREG;
pub use ADDDMAREGd;
pub use brisc_ADDDMAREGd;

#[macro_export]
macro_rules! ADDRCRXY_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        { $crate::tt_op!(0x53, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6) | ($BitMask << 0)) }
    };
}
#[macro_export]
macro_rules! ADDRCRXY_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3) && $crate::asm::is_valid(BitMask, 6)
    };
}
#[macro_export]
macro_rules! ADDRCRXY {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ADDRCRXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask))); }
    };
}
#[macro_export]
macro_rules! ADDRCRXYd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::push_instrn($crate::asm::ADDRCRXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
#[macro_export]
macro_rules! brisc_ADDRCRXYd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ADDRCRXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
pub use ADDRCRXY_value;
pub use ADDRCRXY_valid;
pub use ADDRCRXY;
pub use ADDRCRXYd;
pub use brisc_ADDRCRXYd;

#[macro_export]
macro_rules! ADDRCRZW_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        { $crate::tt_op!(0x56, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6) | ($BitMask << 0)) }
    };
}
#[macro_export]
macro_rules! ADDRCRZW_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3) && $crate::asm::is_valid(BitMask, 6)
    };
}
#[macro_export]
macro_rules! ADDRCRZW {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ADDRCRZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask))); }
    };
}
#[macro_export]
macro_rules! ADDRCRZWd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::push_instrn($crate::asm::ADDRCRZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
#[macro_export]
macro_rules! brisc_ADDRCRZWd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ADDRCRZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
pub use ADDRCRZW_value;
pub use ADDRCRZW_valid;
pub use ADDRCRZW;
pub use ADDRCRZWd;
pub use brisc_ADDRCRZWd;

#[macro_export]
macro_rules! APOOL3S1_value {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x25, ($clear_dvalid << 22) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! APOOL3S1_valid {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(addr_mode, 7) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! APOOL3S1 {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::APOOL3S1_value!($clear_dvalid, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! APOOL3S1d {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::APOOL3S1_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_APOOL3S1d {
    ($core:path, $clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::APOOL3S1_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
pub use APOOL3S1_value;
pub use APOOL3S1_valid;
pub use APOOL3S1;
pub use APOOL3S1d;
pub use brisc_APOOL3S1d;

#[macro_export]
macro_rules! APOOL3S2_value {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x32, ($clear_dvalid << 22) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! APOOL3S2_valid {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(addr_mode, 7) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! APOOL3S2 {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::APOOL3S2_value!($clear_dvalid, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! APOOL3S2d {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::APOOL3S2_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_APOOL3S2d {
    ($core:path, $clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::APOOL3S2_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
pub use APOOL3S2_value;
pub use APOOL3S2_valid;
pub use APOOL3S2;
pub use APOOL3S2d;
pub use brisc_APOOL3S2d;

#[macro_export]
macro_rules! ATCAS_value {
    ($MemHierSel:expr, $SwapVal:expr, $CmpVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x64, ($MemHierSel << 23) | ($SwapVal << 18) | ($CmpVal << 14) | ($Sel32b << 12) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! ATCAS_valid {
    ($MemHierSel:expr, $SwapVal:expr, $CmpVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(MemHierSel, 1) && $crate::asm::is_valid(SwapVal, 5) && $crate::asm::is_valid(CmpVal, 4) && $crate::asm::is_valid(Sel32b, 2) && $crate::asm::is_valid(DataRegIndex, 6) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! ATCAS {
    ($MemHierSel:expr, $SwapVal:expr, $CmpVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATCAS_value!($MemHierSel, $SwapVal, $CmpVal, $Sel32b, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! ATCASd {
    ($MemHierSel:expr, $SwapVal:expr, $CmpVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::ATCAS_value!($MemHierSel, $SwapVal, $CmpVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_ATCASd {
    ($core:path, $MemHierSel:expr, $SwapVal:expr, $CmpVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATCAS_value!($MemHierSel, $SwapVal, $CmpVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use ATCAS_value;
pub use ATCAS_valid;
pub use ATCAS;
pub use ATCASd;
pub use brisc_ATCASd;

#[macro_export]
macro_rules! ATGETM_value {
    ($mutex_index:expr) => {
        { $crate::tt_op!(0xa0, ($mutex_index << 0)) }
    };
}
#[macro_export]
macro_rules! ATGETM_valid {
    ($mutex_index:expr) => {
        $crate::asm::is_valid(mutex_index, 24)
    };
}
#[macro_export]
macro_rules! ATGETM {
    ($mutex_index:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATGETM_value!($mutex_index))); }
    };
}
#[macro_export]
macro_rules! ATGETMd {
    ($mutex_index:expr) => {
    { $crate::push_instrn($crate::asm::ATGETM_value!($mutex_index)); }
    };
}
#[macro_export]
macro_rules! brisc_ATGETMd {
    ($core:path, $mutex_index:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATGETM_value!($mutex_index)); }
    };
}
pub use ATGETM_value;
pub use ATGETM_valid;
pub use ATGETM;
pub use ATGETMd;
pub use brisc_ATGETMd;

#[macro_export]
macro_rules! ATINCGET_value {
    ($MemHierSel:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x61, ($MemHierSel << 23) | ($WrapVal << 14) | ($Sel32b << 12) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! ATINCGET_valid {
    ($MemHierSel:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(MemHierSel, 1) && $crate::asm::is_valid(WrapVal, 9) && $crate::asm::is_valid(Sel32b, 2) && $crate::asm::is_valid(DataRegIndex, 6) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! ATINCGET {
    ($MemHierSel:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATINCGET_value!($MemHierSel, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! ATINCGETd {
    ($MemHierSel:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::ATINCGET_value!($MemHierSel, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_ATINCGETd {
    ($core:path, $MemHierSel:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATINCGET_value!($MemHierSel, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use ATINCGET_value;
pub use ATINCGET_valid;
pub use ATINCGET;
pub use ATINCGETd;
pub use brisc_ATINCGETd;

#[macro_export]
macro_rules! ATINCGETPTR_value {
    ($MemHierSel:expr, $NoIncr:expr, $IncrVal:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x62, ($MemHierSel << 23) | ($NoIncr << 22) | ($IncrVal << 18) | ($WrapVal << 14) | ($Sel32b << 12) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! ATINCGETPTR_valid {
    ($MemHierSel:expr, $NoIncr:expr, $IncrVal:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(MemHierSel, 1) && $crate::asm::is_valid(NoIncr, 1) && $crate::asm::is_valid(IncrVal, 4) && $crate::asm::is_valid(WrapVal, 4) && $crate::asm::is_valid(Sel32b, 2) && $crate::asm::is_valid(DataRegIndex, 6) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! ATINCGETPTR {
    ($MemHierSel:expr, $NoIncr:expr, $IncrVal:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATINCGETPTR_value!($MemHierSel, $NoIncr, $IncrVal, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! ATINCGETPTRd {
    ($MemHierSel:expr, $NoIncr:expr, $IncrVal:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::ATINCGETPTR_value!($MemHierSel, $NoIncr, $IncrVal, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_ATINCGETPTRd {
    ($core:path, $MemHierSel:expr, $NoIncr:expr, $IncrVal:expr, $WrapVal:expr, $Sel32b:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATINCGETPTR_value!($MemHierSel, $NoIncr, $IncrVal, $WrapVal, $Sel32b, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use ATINCGETPTR_value;
pub use ATINCGETPTR_valid;
pub use ATINCGETPTR;
pub use ATINCGETPTRd;
pub use brisc_ATINCGETPTRd;

#[macro_export]
macro_rules! ATRELM_value {
    ($mutex_index:expr) => {
        { $crate::tt_op!(0xa1, ($mutex_index << 0)) }
    };
}
#[macro_export]
macro_rules! ATRELM_valid {
    ($mutex_index:expr) => {
        $crate::asm::is_valid(mutex_index, 24)
    };
}
#[macro_export]
macro_rules! ATRELM {
    ($mutex_index:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATRELM_value!($mutex_index))); }
    };
}
#[macro_export]
macro_rules! ATRELMd {
    ($mutex_index:expr) => {
    { $crate::push_instrn($crate::asm::ATRELM_value!($mutex_index)); }
    };
}
#[macro_export]
macro_rules! brisc_ATRELMd {
    ($core:path, $mutex_index:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATRELM_value!($mutex_index)); }
    };
}
pub use ATRELM_value;
pub use ATRELM_valid;
pub use ATRELM;
pub use ATRELMd;
pub use brisc_ATRELMd;

#[macro_export]
macro_rules! ATSWAP_value {
    ($MemHierSel:expr, $SwapMask:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x63, ($MemHierSel << 23) | ($SwapMask << 14) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! ATSWAP_valid {
    ($MemHierSel:expr, $SwapMask:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(MemHierSel, 1) && $crate::asm::is_valid(SwapMask, 9) && $crate::asm::is_valid(DataRegIndex, 8) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! ATSWAP {
    ($MemHierSel:expr, $SwapMask:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ATSWAP_value!($MemHierSel, $SwapMask, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! ATSWAPd {
    ($MemHierSel:expr, $SwapMask:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::ATSWAP_value!($MemHierSel, $SwapMask, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_ATSWAPd {
    ($core:path, $MemHierSel:expr, $SwapMask:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ATSWAP_value!($MemHierSel, $SwapMask, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use ATSWAP_value;
pub use ATSWAP_valid;
pub use ATSWAP;
pub use ATSWAPd;
pub use brisc_ATSWAPd;

#[macro_export]
macro_rules! BITWOPDMAREG_value {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x5b, ($OpBisConst << 23) | ($OpSel << 18) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! BITWOPDMAREG_valid {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(OpSel, 5) && $crate::asm::is_valid(ResultRegIndex, 6) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! BITWOPDMAREG {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::BITWOPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! BITWOPDMAREGd {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::BITWOPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_BITWOPDMAREGd {
    ($core:path, $OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::BITWOPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use BITWOPDMAREG_value;
pub use BITWOPDMAREG_valid;
pub use BITWOPDMAREG;
pub use BITWOPDMAREGd;
pub use brisc_BITWOPDMAREGd;

#[macro_export]
macro_rules! CLEARDVALID_value {
    ($cleardvalid:expr, $reset:expr) => {
        { $crate::tt_op!(0x36, ($cleardvalid << 22) | ($reset << 0)) }
    };
}
#[macro_export]
macro_rules! CLEARDVALID_valid {
    ($cleardvalid:expr, $reset:expr) => {
        $crate::asm::is_valid(cleardvalid, 2) && $crate::asm::is_valid(reset, 22)
    };
}
#[macro_export]
macro_rules! CLEARDVALID {
    ($cleardvalid:expr, $reset:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::CLEARDVALID_value!($cleardvalid, $reset))); }
    };
}
#[macro_export]
macro_rules! CLEARDVALIDd {
    ($cleardvalid:expr, $reset:expr) => {
    { $crate::push_instrn($crate::asm::CLEARDVALID_value!($cleardvalid, $reset)); }
    };
}
#[macro_export]
macro_rules! brisc_CLEARDVALIDd {
    ($core:path, $cleardvalid:expr, $reset:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::CLEARDVALID_value!($cleardvalid, $reset)); }
    };
}
pub use CLEARDVALID_value;
pub use CLEARDVALID_valid;
pub use CLEARDVALID;
pub use CLEARDVALIDd;
pub use brisc_CLEARDVALIDd;

#[macro_export]
macro_rules! CLREXPHIST_value {
    () => {
        { $crate::tt_op!(0x21) }
    };
}
#[macro_export]
macro_rules! CLREXPHIST_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! CLREXPHIST {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::CLREXPHIST_value!())); }
    };
}
#[macro_export]
macro_rules! CLREXPHISTd {
    () => {
    { $crate::push_instrn($crate::asm::CLREXPHIST_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_CLREXPHISTd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::CLREXPHIST_value!()); }
    };
}
pub use CLREXPHIST_value;
pub use CLREXPHIST_valid;
pub use CLREXPHIST;
pub use CLREXPHISTd;
pub use brisc_CLREXPHISTd;

#[macro_export]
macro_rules! CMPDMAREG_value {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x5d, ($OpBisConst << 23) | ($OpSel << 18) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! CMPDMAREG_valid {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(OpSel, 5) && $crate::asm::is_valid(ResultRegIndex, 6) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! CMPDMAREG {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::CMPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! CMPDMAREGd {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::CMPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_CMPDMAREGd {
    ($core:path, $OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::CMPDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use CMPDMAREG_value;
pub use CMPDMAREG_valid;
pub use CMPDMAREG;
pub use CMPDMAREGd;
pub use brisc_CMPDMAREGd;

#[macro_export]
macro_rules! CONV3S1_value {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x22, ($clear_dvalid << 22) | ($rotate_weights << 17) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! CONV3S1_valid {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(rotate_weights, 5) && $crate::asm::is_valid(addr_mode, 2) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! CONV3S1 {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::CONV3S1_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! CONV3S1d {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::CONV3S1_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_CONV3S1d {
    ($core:path, $clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::CONV3S1_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst)); }
    };
}
pub use CONV3S1_value;
pub use CONV3S1_valid;
pub use CONV3S1;
pub use CONV3S1d;
pub use brisc_CONV3S1d;

#[macro_export]
macro_rules! CONV3S2_value {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x23, ($clear_dvalid << 22) | ($rotate_weights << 17) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! CONV3S2_valid {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(rotate_weights, 5) && $crate::asm::is_valid(addr_mode, 2) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! CONV3S2 {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::CONV3S2_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! CONV3S2d {
    ($clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::CONV3S2_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_CONV3S2d {
    ($core:path, $clear_dvalid:expr, $rotate_weights:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::CONV3S2_value!($clear_dvalid, $rotate_weights, $addr_mode, $dst)); }
    };
}
pub use CONV3S2_value;
pub use CONV3S2_valid;
pub use CONV3S2;
pub use CONV3S2d;
pub use brisc_CONV3S2d;

#[macro_export]
macro_rules! DMANOP_value {
    () => {
        { $crate::tt_op!(0x60) }
    };
}
#[macro_export]
macro_rules! DMANOP_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! DMANOP {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::DMANOP_value!())); }
    };
}
#[macro_export]
macro_rules! DMANOPd {
    () => {
    { $crate::push_instrn($crate::asm::DMANOP_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_DMANOPd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::DMANOP_value!()); }
    };
}
pub use DMANOP_value;
pub use DMANOP_valid;
pub use DMANOP;
pub use DMANOPd;
pub use brisc_DMANOPd;

#[macro_export]
macro_rules! DOTPV_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x29, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! DOTPV_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! DOTPV {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::DOTPV_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! DOTPVd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::DOTPV_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_DOTPVd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::DOTPV_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use DOTPV_value;
pub use DOTPV_valid;
pub use DOTPV;
pub use DOTPVd;
pub use brisc_DOTPVd;

#[macro_export]
macro_rules! ELWADD_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x28, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! ELWADD_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! ELWADD {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ELWADD_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! ELWADDd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::ELWADD_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_ELWADDd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ELWADD_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use ELWADD_value;
pub use ELWADD_valid;
pub use ELWADD;
pub use ELWADDd;
pub use brisc_ELWADDd;

#[macro_export]
macro_rules! ELWMUL_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x27, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! ELWMUL_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! ELWMUL {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ELWMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! ELWMULd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::ELWMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_ELWMULd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ELWMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use ELWMUL_value;
pub use ELWMUL_valid;
pub use ELWMUL;
pub use ELWMULd;
pub use brisc_ELWMULd;

#[macro_export]
macro_rules! ELWSUB_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x30, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! ELWSUB_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! ELWSUB {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ELWSUB_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! ELWSUBd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::ELWSUB_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_ELWSUBd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ELWSUB_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use ELWSUB_value;
pub use ELWSUB_valid;
pub use ELWSUB;
pub use ELWSUBd;
pub use brisc_ELWSUBd;

#[macro_export]
macro_rules! FLUSHDMA_value {
    ($FlushSpec:expr) => {
        { $crate::tt_op!(0x46, ($FlushSpec << 0)) }
    };
}
#[macro_export]
macro_rules! FLUSHDMA_valid {
    ($FlushSpec:expr) => {
        $crate::asm::is_valid(FlushSpec, 24)
    };
}
#[macro_export]
macro_rules! FLUSHDMA {
    ($FlushSpec:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::FLUSHDMA_value!($FlushSpec))); }
    };
}
#[macro_export]
macro_rules! FLUSHDMAd {
    ($FlushSpec:expr) => {
    { $crate::push_instrn($crate::asm::FLUSHDMA_value!($FlushSpec)); }
    };
}
#[macro_export]
macro_rules! brisc_FLUSHDMAd {
    ($core:path, $FlushSpec:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::FLUSHDMA_value!($FlushSpec)); }
    };
}
pub use FLUSHDMA_value;
pub use FLUSHDMA_valid;
pub use FLUSHDMA;
pub use FLUSHDMAd;
pub use brisc_FLUSHDMAd;

#[macro_export]
macro_rules! GAPOOL_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x34, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! GAPOOL_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! GAPOOL {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::GAPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! GAPOOLd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::GAPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_GAPOOLd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::GAPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use GAPOOL_value;
pub use GAPOOL_valid;
pub use GAPOOL;
pub use GAPOOLd;
pub use brisc_GAPOOLd;

#[macro_export]
macro_rules! GATESRCRST_value {
    ($reset_srcb_gate_control:expr, $reset_srca_gate_control:expr) => {
        { $crate::tt_op!(0x35, ($reset_srcb_gate_control << 1) | ($reset_srca_gate_control << 0)) }
    };
}
#[macro_export]
macro_rules! GATESRCRST_valid {
    ($reset_srcb_gate_control:expr, $reset_srca_gate_control:expr) => {
        $crate::asm::is_valid(reset_srcb_gate_control, 23) && $crate::asm::is_valid(reset_srca_gate_control, 1)
    };
}
#[macro_export]
macro_rules! GATESRCRST {
    ($reset_srcb_gate_control:expr, $reset_srca_gate_control:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::GATESRCRST_value!($reset_srcb_gate_control, $reset_srca_gate_control))); }
    };
}
#[macro_export]
macro_rules! GATESRCRSTd {
    ($reset_srcb_gate_control:expr, $reset_srca_gate_control:expr) => {
    { $crate::push_instrn($crate::asm::GATESRCRST_value!($reset_srcb_gate_control, $reset_srca_gate_control)); }
    };
}
#[macro_export]
macro_rules! brisc_GATESRCRSTd {
    ($core:path, $reset_srcb_gate_control:expr, $reset_srca_gate_control:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::GATESRCRST_value!($reset_srcb_gate_control, $reset_srca_gate_control)); }
    };
}
pub use GATESRCRST_value;
pub use GATESRCRST_valid;
pub use GATESRCRST;
pub use GATESRCRSTd;
pub use brisc_GATESRCRSTd;

#[macro_export]
macro_rules! GMPOOL_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x33, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! GMPOOL_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! GMPOOL {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::GMPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! GMPOOLd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::GMPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_GMPOOLd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::GMPOOL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use GMPOOL_value;
pub use GMPOOL_valid;
pub use GMPOOL;
pub use GMPOOLd;
pub use brisc_GMPOOLd;

#[macro_export]
macro_rules! INCADCXY_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
        { $crate::tt_op!(0x52, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6)) }
    };
}
#[macro_export]
macro_rules! INCADCXY_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3)
    };
}
#[macro_export]
macro_rules! INCADCXY {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::INCADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X))); }
    };
}
#[macro_export]
macro_rules! INCADCXYd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::push_instrn($crate::asm::INCADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X)); }
    };
}
#[macro_export]
macro_rules! brisc_INCADCXYd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::INCADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X)); }
    };
}
pub use INCADCXY_value;
pub use INCADCXY_valid;
pub use INCADCXY;
pub use INCADCXYd;
pub use brisc_INCADCXYd;

#[macro_export]
macro_rules! INCADCZW_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
        { $crate::tt_op!(0x55, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6)) }
    };
}
#[macro_export]
macro_rules! INCADCZW_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3)
    };
}
#[macro_export]
macro_rules! INCADCZW {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::INCADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X))); }
    };
}
#[macro_export]
macro_rules! INCADCZWd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::push_instrn($crate::asm::INCADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X)); }
    };
}
#[macro_export]
macro_rules! brisc_INCADCZWd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::INCADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X)); }
    };
}
pub use INCADCZW_value;
pub use INCADCZW_valid;
pub use INCADCZW;
pub use INCADCZWd;
pub use brisc_INCADCZWd;

#[macro_export]
macro_rules! INCRWC_value {
    ($rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr) => {
        { $crate::tt_op!(0x38, ($rwc_cr << 18) | ($rwc_d << 14) | ($rwc_b << 10) | ($rwc_a << 6)) }
    };
}
#[macro_export]
macro_rules! INCRWC_valid {
    ($rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr) => {
        $crate::asm::is_valid(rwc_cr, 6) && $crate::asm::is_valid(rwc_d, 4) && $crate::asm::is_valid(rwc_b, 4) && $crate::asm::is_valid(rwc_a, 4)
    };
}
#[macro_export]
macro_rules! INCRWC {
    ($rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::INCRWC_value!($rwc_cr, $rwc_d, $rwc_b, $rwc_a))); }
    };
}
#[macro_export]
macro_rules! INCRWCd {
    ($rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr) => {
    { $crate::push_instrn($crate::asm::INCRWC_value!($rwc_cr, $rwc_d, $rwc_b, $rwc_a)); }
    };
}
#[macro_export]
macro_rules! brisc_INCRWCd {
    ($core:path, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::INCRWC_value!($rwc_cr, $rwc_d, $rwc_b, $rwc_a)); }
    };
}
pub use INCRWC_value;
pub use INCRWC_valid;
pub use INCRWC;
pub use INCRWCd;
pub use brisc_INCRWCd;

#[macro_export]
macro_rules! LOADIND_value {
    ($SizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x49, ($SizeSel << 22) | ($OffsetIndex << 14) | ($AutoIncSpec << 12) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! LOADIND_valid {
    ($SizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(SizeSel, 2) && $crate::asm::is_valid(OffsetIndex, 8) && $crate::asm::is_valid(AutoIncSpec, 2) && $crate::asm::is_valid(DataRegIndex, 6) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! LOADIND {
    ($SizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::LOADIND_value!($SizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! LOADINDd {
    ($SizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::LOADIND_value!($SizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_LOADINDd {
    ($core:path, $SizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::LOADIND_value!($SizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use LOADIND_value;
pub use LOADIND_valid;
pub use LOADIND;
pub use LOADINDd;
pub use brisc_LOADINDd;

#[macro_export]
macro_rules! LOADREG_value {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
        { $crate::tt_op!(0x68, ($TdmaDataRegIndex << 18) | ($RegAddr << 0)) }
    };
}
#[macro_export]
macro_rules! LOADREG_valid {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
        $crate::asm::is_valid(TdmaDataRegIndex, 6) && $crate::asm::is_valid(RegAddr, 18)
    };
}
#[macro_export]
macro_rules! LOADREG {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::LOADREG_value!($TdmaDataRegIndex, $RegAddr))); }
    };
}
#[macro_export]
macro_rules! LOADREGd {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::push_instrn($crate::asm::LOADREG_value!($TdmaDataRegIndex, $RegAddr)); }
    };
}
#[macro_export]
macro_rules! brisc_LOADREGd {
    ($core:path, $TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::LOADREG_value!($TdmaDataRegIndex, $RegAddr)); }
    };
}
pub use LOADREG_value;
pub use LOADREG_valid;
pub use LOADREG;
pub use LOADREGd;
pub use brisc_LOADREGd;

#[macro_export]
macro_rules! MOP_value {
    ($mop_type:expr, $loop_count:expr, $zmask_lo16:expr) => {
        { $crate::tt_op!(0x01, ($mop_type << 23) | ($loop_count << 16) | ($zmask_lo16 << 0)) }
    };
}
#[macro_export]
macro_rules! MOP_valid {
    ($mop_type:expr, $loop_count:expr, $zmask_lo16:expr) => {
        $crate::asm::is_valid(mop_type, 1) && $crate::asm::is_valid(loop_count, 7) && $crate::asm::is_valid(zmask_lo16, 16)
    };
}
#[macro_export]
macro_rules! MOP {
    ($mop_type:expr, $loop_count:expr, $zmask_lo16:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOP_value!($mop_type, $loop_count, $zmask_lo16))); }
    };
}
#[macro_export]
macro_rules! MOPd {
    ($mop_type:expr, $loop_count:expr, $zmask_lo16:expr) => {
    { $crate::push_instrn($crate::asm::MOP_value!($mop_type, $loop_count, $zmask_lo16)); }
    };
}
#[macro_export]
macro_rules! brisc_MOPd {
    ($core:path, $mop_type:expr, $loop_count:expr, $zmask_lo16:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOP_value!($mop_type, $loop_count, $zmask_lo16)); }
    };
}
pub use MOP_value;
pub use MOP_valid;
pub use MOP;
pub use MOPd;
pub use brisc_MOPd;

#[macro_export]
macro_rules! MOP_CFG_value {
    ($zmask_hi16:expr) => {
        { $crate::tt_op!(0x03, ($zmask_hi16 << 0)) }
    };
}
#[macro_export]
macro_rules! MOP_CFG_valid {
    ($zmask_hi16:expr) => {
        $crate::asm::is_valid(zmask_hi16, 24)
    };
}
#[macro_export]
macro_rules! MOP_CFG {
    ($zmask_hi16:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOP_CFG_value!($zmask_hi16))); }
    };
}
#[macro_export]
macro_rules! MOP_CFGd {
    ($zmask_hi16:expr) => {
    { $crate::push_instrn($crate::asm::MOP_CFG_value!($zmask_hi16)); }
    };
}
#[macro_export]
macro_rules! brisc_MOP_CFGd {
    ($core:path, $zmask_hi16:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOP_CFG_value!($zmask_hi16)); }
    };
}
pub use MOP_CFG_value;
pub use MOP_CFG_valid;
pub use MOP_CFG;
pub use MOP_CFGd;
pub use brisc_MOP_CFGd;

#[macro_export]
macro_rules! MOVA2D_value {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        { $crate::tt_op!(0x12, ($instr_mod << 19) | ($addr_mode << 15) | ($src << 10) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MOVA2D_valid {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        $crate::asm::is_valid(instr_mod, 5) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(src, 5) && $crate::asm::is_valid(dst, 10)
    };
}
#[macro_export]
macro_rules! MOVA2D {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOVA2D_value!($instr_mod, $addr_mode, $src, $dst))); }
    };
}
#[macro_export]
macro_rules! MOVA2Dd {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MOVA2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MOVA2Dd {
    ($core:path, $instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOVA2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
pub use MOVA2D_value;
pub use MOVA2D_valid;
pub use MOVA2D;
pub use MOVA2Dd;
pub use brisc_MOVA2Dd;

#[macro_export]
macro_rules! MOVB2D_value {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        { $crate::tt_op!(0x13, ($instr_mod << 19) | ($addr_mode << 15) | ($src << 10) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MOVB2D_valid {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        $crate::asm::is_valid(instr_mod, 5) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(src, 5) && $crate::asm::is_valid(dst, 10)
    };
}
#[macro_export]
macro_rules! MOVB2D {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOVB2D_value!($instr_mod, $addr_mode, $src, $dst))); }
    };
}
#[macro_export]
macro_rules! MOVB2Dd {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MOVB2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MOVB2Dd {
    ($core:path, $instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOVB2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
pub use MOVB2D_value;
pub use MOVB2D_valid;
pub use MOVB2D;
pub use MOVB2Dd;
pub use brisc_MOVB2Dd;

#[macro_export]
macro_rules! MOVD2A_value {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        { $crate::tt_op!(0x08, ($instr_mod << 19) | ($addr_mode << 15) | ($src << 10) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MOVD2A_valid {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        $crate::asm::is_valid(instr_mod, 5) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(src, 5) && $crate::asm::is_valid(dst, 10)
    };
}
#[macro_export]
macro_rules! MOVD2A {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOVD2A_value!($instr_mod, $addr_mode, $src, $dst))); }
    };
}
#[macro_export]
macro_rules! MOVD2Ad {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MOVD2A_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MOVD2Ad {
    ($core:path, $instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOVD2A_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
pub use MOVD2A_value;
pub use MOVD2A_valid;
pub use MOVD2A;
pub use MOVD2Ad;
pub use brisc_MOVD2Ad;

#[macro_export]
macro_rules! MOVDBGA2D_value {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        { $crate::tt_op!(0x09, ($instr_mod << 19) | ($addr_mode << 15) | ($src << 10) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MOVDBGA2D_valid {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
        $crate::asm::is_valid(instr_mod, 5) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(src, 5) && $crate::asm::is_valid(dst, 10)
    };
}
#[macro_export]
macro_rules! MOVDBGA2D {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MOVDBGA2D_value!($instr_mod, $addr_mode, $src, $dst))); }
    };
}
#[macro_export]
macro_rules! MOVDBGA2Dd {
    ($instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MOVDBGA2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MOVDBGA2Dd {
    ($core:path, $instr_mod:expr, $addr_mode:expr, $src:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MOVDBGA2D_value!($instr_mod, $addr_mode, $src, $dst)); }
    };
}
pub use MOVDBGA2D_value;
pub use MOVDBGA2D_valid;
pub use MOVDBGA2D;
pub use MOVDBGA2Dd;
pub use brisc_MOVDBGA2Dd;

#[macro_export]
macro_rules! MPOOL3S1_value {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x24, ($clear_dvalid << 22) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MPOOL3S1_valid {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(addr_mode, 7) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! MPOOL3S1 {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MPOOL3S1_value!($clear_dvalid, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! MPOOL3S1d {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MPOOL3S1_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MPOOL3S1d {
    ($core:path, $clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MPOOL3S1_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
pub use MPOOL3S1_value;
pub use MPOOL3S1_valid;
pub use MPOOL3S1;
pub use MPOOL3S1d;
pub use brisc_MPOOL3S1d;

#[macro_export]
macro_rules! MPOOL3S2_value {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x31, ($clear_dvalid << 22) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MPOOL3S2_valid {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(addr_mode, 7) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! MPOOL3S2 {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MPOOL3S2_value!($clear_dvalid, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! MPOOL3S2d {
    ($clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MPOOL3S2_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MPOOL3S2d {
    ($core:path, $clear_dvalid:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MPOOL3S2_value!($clear_dvalid, $addr_mode, $dst)); }
    };
}
pub use MPOOL3S2_value;
pub use MPOOL3S2_valid;
pub use MPOOL3S2;
pub use MPOOL3S2d;
pub use brisc_MPOOL3S2d;

#[macro_export]
macro_rules! MULDMAREG_value {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x5a, ($OpBisConst << 23) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! MULDMAREG_valid {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(ResultRegIndex, 11) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! MULDMAREG {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MULDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! MULDMAREGd {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::MULDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_MULDMAREGd {
    ($core:path, $OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MULDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use MULDMAREG_value;
pub use MULDMAREG_valid;
pub use MULDMAREG;
pub use MULDMAREGd;
pub use brisc_MULDMAREGd;

#[macro_export]
macro_rules! MVMUL_value {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        { $crate::tt_op!(0x26, ($clear_dvalid << 22) | ($instr_mod << 19) | ($addr_mode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! MVMUL_valid {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_dvalid, 2) && $crate::asm::is_valid(instr_mod, 3) && $crate::asm::is_valid(addr_mode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! MVMUL {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::MVMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst))); }
    };
}
#[macro_export]
macro_rules! MVMULd {
    ($clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::MVMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_MVMULd {
    ($core:path, $clear_dvalid:expr, $instr_mod:expr, $addr_mode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::MVMUL_value!($clear_dvalid, $instr_mod, $addr_mode, $dst)); }
    };
}
pub use MVMUL_value;
pub use MVMUL_valid;
pub use MVMUL;
pub use MVMULd;
pub use brisc_MVMULd;

#[macro_export]
macro_rules! NOP_value {
    () => {
        { $crate::tt_op!(0x02) }
    };
}
#[macro_export]
macro_rules! NOP_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! NOP {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::NOP_value!())); }
    };
}
#[macro_export]
macro_rules! NOPd {
    () => {
    { $crate::push_instrn($crate::asm::NOP_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_NOPd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::NOP_value!()); }
    };
}
pub use NOP_value;
pub use NOP_valid;
pub use NOP;
pub use NOPd;
pub use brisc_NOPd;

#[macro_export]
macro_rules! PACR_value {
    ($AddrMode:expr, $ZeroWrite:expr, $PackSel:expr, $OvrdThreadId:expr, $Concat:expr, $Flush:expr, $Last:expr) => {
        { $crate::tt_op!(0x41, ($AddrMode << 15) | ($ZeroWrite << 12) | ($PackSel << 8) | ($OvrdThreadId << 7) | ($Concat << 4) | ($Flush << 1) | ($Last << 0)) }
    };
}
#[macro_export]
macro_rules! PACR_valid {
    ($AddrMode:expr, $ZeroWrite:expr, $PackSel:expr, $OvrdThreadId:expr, $Concat:expr, $Flush:expr, $Last:expr) => {
        $crate::asm::is_valid(AddrMode, 9) && $crate::asm::is_valid(ZeroWrite, 3) && $crate::asm::is_valid(PackSel, 4) && $crate::asm::is_valid(OvrdThreadId, 1) && $crate::asm::is_valid(Concat, 3) && $crate::asm::is_valid(Flush, 3) && $crate::asm::is_valid(Last, 1)
    };
}
#[macro_export]
macro_rules! PACR {
    ($AddrMode:expr, $ZeroWrite:expr, $PackSel:expr, $OvrdThreadId:expr, $Concat:expr, $Flush:expr, $Last:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::PACR_value!($AddrMode, $ZeroWrite, $PackSel, $OvrdThreadId, $Concat, $Flush, $Last))); }
    };
}
#[macro_export]
macro_rules! PACRd {
    ($AddrMode:expr, $ZeroWrite:expr, $PackSel:expr, $OvrdThreadId:expr, $Concat:expr, $Flush:expr, $Last:expr) => {
    { $crate::push_instrn($crate::asm::PACR_value!($AddrMode, $ZeroWrite, $PackSel, $OvrdThreadId, $Concat, $Flush, $Last)); }
    };
}
#[macro_export]
macro_rules! brisc_PACRd {
    ($core:path, $AddrMode:expr, $ZeroWrite:expr, $PackSel:expr, $OvrdThreadId:expr, $Concat:expr, $Flush:expr, $Last:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::PACR_value!($AddrMode, $ZeroWrite, $PackSel, $OvrdThreadId, $Concat, $Flush, $Last)); }
    };
}
pub use PACR_value;
pub use PACR_valid;
pub use PACR;
pub use PACRd;
pub use brisc_PACRd;

#[macro_export]
macro_rules! RAREB_value {
    () => {
        { $crate::tt_op!(0x15) }
    };
}
#[macro_export]
macro_rules! RAREB_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! RAREB {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::RAREB_value!())); }
    };
}
#[macro_export]
macro_rules! RAREBd {
    () => {
    { $crate::push_instrn($crate::asm::RAREB_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_RAREBd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::RAREB_value!()); }
    };
}
pub use RAREB_value;
pub use RAREB_valid;
pub use RAREB;
pub use RAREBd;
pub use brisc_RAREBd;

#[macro_export]
macro_rules! RDCFG_value {
    ($GprAddress:expr, $CfgReg:expr) => {
        { $crate::tt_op!(0xb1, ($GprAddress << 16) | ($CfgReg << 0)) }
    };
}
#[macro_export]
macro_rules! RDCFG_valid {
    ($GprAddress:expr, $CfgReg:expr) => {
        $crate::asm::is_valid(GprAddress, 8) && $crate::asm::is_valid(CfgReg, 16)
    };
}
#[macro_export]
macro_rules! RDCFG {
    ($GprAddress:expr, $CfgReg:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::RDCFG_value!($GprAddress, $CfgReg))); }
    };
}
#[macro_export]
macro_rules! RDCFGd {
    ($GprAddress:expr, $CfgReg:expr) => {
    { $crate::push_instrn($crate::asm::RDCFG_value!($GprAddress, $CfgReg)); }
    };
}
#[macro_export]
macro_rules! brisc_RDCFGd {
    ($core:path, $GprAddress:expr, $CfgReg:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::RDCFG_value!($GprAddress, $CfgReg)); }
    };
}
pub use RDCFG_value;
pub use RDCFG_valid;
pub use RDCFG;
pub use RDCFGd;
pub use brisc_RDCFGd;

#[macro_export]
macro_rules! REG2FLOP_value {
    ($SizeSel:expr, $TargetSel:expr, $ByteOffset:expr, $ContextId_2:expr, $FlopIndex:expr, $RegIndex:expr) => {
        { $crate::tt_op!(0x48, ($SizeSel << 22) | ($TargetSel << 20) | ($ByteOffset << 18) | ($ContextId_2 << 16) | ($FlopIndex << 6) | ($RegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! REG2FLOP_valid {
    ($SizeSel:expr, $TargetSel:expr, $ByteOffset:expr, $ContextId_2:expr, $FlopIndex:expr, $RegIndex:expr) => {
        $crate::asm::is_valid(SizeSel, 2) && $crate::asm::is_valid(TargetSel, 2) && $crate::asm::is_valid(ByteOffset, 2) && $crate::asm::is_valid(ContextId_2, 2) && $crate::asm::is_valid(FlopIndex, 10) && $crate::asm::is_valid(RegIndex, 6)
    };
}
#[macro_export]
macro_rules! REG2FLOP {
    ($SizeSel:expr, $TargetSel:expr, $ByteOffset:expr, $ContextId_2:expr, $FlopIndex:expr, $RegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::REG2FLOP_value!($SizeSel, $TargetSel, $ByteOffset, $ContextId_2, $FlopIndex, $RegIndex))); }
    };
}
#[macro_export]
macro_rules! REG2FLOPd {
    ($SizeSel:expr, $TargetSel:expr, $ByteOffset:expr, $ContextId_2:expr, $FlopIndex:expr, $RegIndex:expr) => {
    { $crate::push_instrn($crate::asm::REG2FLOP_value!($SizeSel, $TargetSel, $ByteOffset, $ContextId_2, $FlopIndex, $RegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_REG2FLOPd {
    ($core:path, $SizeSel:expr, $TargetSel:expr, $ByteOffset:expr, $ContextId_2:expr, $FlopIndex:expr, $RegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::REG2FLOP_value!($SizeSel, $TargetSel, $ByteOffset, $ContextId_2, $FlopIndex, $RegIndex)); }
    };
}
pub use REG2FLOP_value;
pub use REG2FLOP_valid;
pub use REG2FLOP;
pub use REG2FLOPd;
pub use brisc_REG2FLOPd;

#[macro_export]
macro_rules! RSTDMA_value {
    () => {
        { $crate::tt_op!(0x44) }
    };
}
#[macro_export]
macro_rules! RSTDMA_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! RSTDMA {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::RSTDMA_value!())); }
    };
}
#[macro_export]
macro_rules! RSTDMAd {
    () => {
    { $crate::push_instrn($crate::asm::RSTDMA_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_RSTDMAd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::RSTDMA_value!()); }
    };
}
pub use RSTDMA_value;
pub use RSTDMA_valid;
pub use RSTDMA;
pub use RSTDMAd;
pub use brisc_RSTDMAd;

#[macro_export]
macro_rules! SEMGET_value {
    ($sem_sel:expr) => {
        { $crate::tt_op!(0xa5, ($sem_sel << 2)) }
    };
}
#[macro_export]
macro_rules! SEMGET_valid {
    ($sem_sel:expr) => {
        $crate::asm::is_valid(sem_sel, 22)
    };
}
#[macro_export]
macro_rules! SEMGET {
    ($sem_sel:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SEMGET_value!($sem_sel))); }
    };
}
#[macro_export]
macro_rules! SEMGETd {
    ($sem_sel:expr) => {
    { $crate::push_instrn($crate::asm::SEMGET_value!($sem_sel)); }
    };
}
#[macro_export]
macro_rules! brisc_SEMGETd {
    ($core:path, $sem_sel:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SEMGET_value!($sem_sel)); }
    };
}
pub use SEMGET_value;
pub use SEMGET_valid;
pub use SEMGET;
pub use SEMGETd;
pub use brisc_SEMGETd;

#[macro_export]
macro_rules! SEMINIT_value {
    ($max_value:expr, $init_value:expr, $sem_sel:expr) => {
        { $crate::tt_op!(0xa3, ($max_value << 20) | ($init_value << 16) | ($sem_sel << 2)) }
    };
}
#[macro_export]
macro_rules! SEMINIT_valid {
    ($max_value:expr, $init_value:expr, $sem_sel:expr) => {
        $crate::asm::is_valid(max_value, 4) && $crate::asm::is_valid(init_value, 4) && $crate::asm::is_valid(sem_sel, 14)
    };
}
#[macro_export]
macro_rules! SEMINIT {
    ($max_value:expr, $init_value:expr, $sem_sel:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SEMINIT_value!($max_value, $init_value, $sem_sel))); }
    };
}
#[macro_export]
macro_rules! SEMINITd {
    ($max_value:expr, $init_value:expr, $sem_sel:expr) => {
    { $crate::push_instrn($crate::asm::SEMINIT_value!($max_value, $init_value, $sem_sel)); }
    };
}
#[macro_export]
macro_rules! brisc_SEMINITd {
    ($core:path, $max_value:expr, $init_value:expr, $sem_sel:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SEMINIT_value!($max_value, $init_value, $sem_sel)); }
    };
}
pub use SEMINIT_value;
pub use SEMINIT_valid;
pub use SEMINIT;
pub use SEMINITd;
pub use brisc_SEMINITd;

#[macro_export]
macro_rules! SEMPOST_value {
    ($sem_sel:expr) => {
        { $crate::tt_op!(0xa4, ($sem_sel << 2)) }
    };
}
#[macro_export]
macro_rules! SEMPOST_valid {
    ($sem_sel:expr) => {
        $crate::asm::is_valid(sem_sel, 22)
    };
}
#[macro_export]
macro_rules! SEMPOST {
    ($sem_sel:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SEMPOST_value!($sem_sel))); }
    };
}
#[macro_export]
macro_rules! SEMPOSTd {
    ($sem_sel:expr) => {
    { $crate::push_instrn($crate::asm::SEMPOST_value!($sem_sel)); }
    };
}
#[macro_export]
macro_rules! brisc_SEMPOSTd {
    ($core:path, $sem_sel:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SEMPOST_value!($sem_sel)); }
    };
}
pub use SEMPOST_value;
pub use SEMPOST_valid;
pub use SEMPOST;
pub use SEMPOSTd;
pub use brisc_SEMPOSTd;

#[macro_export]
macro_rules! SEMWAIT_value {
    ($stall_res:expr, $sem_sel:expr, $wait_sem_cond:expr) => {
        { $crate::tt_op!(0xa6, ($stall_res << 14) | ($sem_sel << 2) | ($wait_sem_cond << 0)) }
    };
}
#[macro_export]
macro_rules! SEMWAIT_valid {
    ($stall_res:expr, $sem_sel:expr, $wait_sem_cond:expr) => {
        $crate::asm::is_valid(stall_res, 10) && $crate::asm::is_valid(sem_sel, 12) && $crate::asm::is_valid(wait_sem_cond, 2)
    };
}
#[macro_export]
macro_rules! SEMWAIT {
    ($stall_res:expr, $sem_sel:expr, $wait_sem_cond:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SEMWAIT_value!($stall_res, $sem_sel, $wait_sem_cond))); }
    };
}
#[macro_export]
macro_rules! SEMWAITd {
    ($stall_res:expr, $sem_sel:expr, $wait_sem_cond:expr) => {
    { $crate::push_instrn($crate::asm::SEMWAIT_value!($stall_res, $sem_sel, $wait_sem_cond)); }
    };
}
#[macro_export]
macro_rules! brisc_SEMWAITd {
    ($core:path, $stall_res:expr, $sem_sel:expr, $wait_sem_cond:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SEMWAIT_value!($stall_res, $sem_sel, $wait_sem_cond)); }
    };
}
pub use SEMWAIT_value;
pub use SEMWAIT_valid;
pub use SEMWAIT;
pub use SEMWAITd;
pub use brisc_SEMWAITd;

#[macro_export]
macro_rules! SETADC_value {
    ($CntSetMask:expr, $ChannelIndex:expr, $DimensionIndex:expr, $Value:expr) => {
        { $crate::tt_op!(0x50, ($CntSetMask << 21) | ($ChannelIndex << 20) | ($DimensionIndex << 18) | ($Value << 0)) }
    };
}
#[macro_export]
macro_rules! SETADC_valid {
    ($CntSetMask:expr, $ChannelIndex:expr, $DimensionIndex:expr, $Value:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(ChannelIndex, 1) && $crate::asm::is_valid(DimensionIndex, 2) && $crate::asm::is_valid(Value, 18)
    };
}
#[macro_export]
macro_rules! SETADC {
    ($CntSetMask:expr, $ChannelIndex:expr, $DimensionIndex:expr, $Value:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETADC_value!($CntSetMask, $ChannelIndex, $DimensionIndex, $Value))); }
    };
}
#[macro_export]
macro_rules! SETADCd {
    ($CntSetMask:expr, $ChannelIndex:expr, $DimensionIndex:expr, $Value:expr) => {
    { $crate::push_instrn($crate::asm::SETADC_value!($CntSetMask, $ChannelIndex, $DimensionIndex, $Value)); }
    };
}
#[macro_export]
macro_rules! brisc_SETADCd {
    ($core:path, $CntSetMask:expr, $ChannelIndex:expr, $DimensionIndex:expr, $Value:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETADC_value!($CntSetMask, $ChannelIndex, $DimensionIndex, $Value)); }
    };
}
pub use SETADC_value;
pub use SETADC_valid;
pub use SETADC;
pub use SETADCd;
pub use brisc_SETADCd;

#[macro_export]
macro_rules! SETADCXX_value {
    ($CntSetMask:expr, $x_end2:expr, $x_start:expr) => {
        { $crate::tt_op!(0x5e, ($CntSetMask << 21) | ($x_end2 << 10) | ($x_start << 0)) }
    };
}
#[macro_export]
macro_rules! SETADCXX_valid {
    ($CntSetMask:expr, $x_end2:expr, $x_start:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(x_end2, 11) && $crate::asm::is_valid(x_start, 10)
    };
}
#[macro_export]
macro_rules! SETADCXX {
    ($CntSetMask:expr, $x_end2:expr, $x_start:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETADCXX_value!($CntSetMask, $x_end2, $x_start))); }
    };
}
#[macro_export]
macro_rules! SETADCXXd {
    ($CntSetMask:expr, $x_end2:expr, $x_start:expr) => {
    { $crate::push_instrn($crate::asm::SETADCXX_value!($CntSetMask, $x_end2, $x_start)); }
    };
}
#[macro_export]
macro_rules! brisc_SETADCXXd {
    ($core:path, $CntSetMask:expr, $x_end2:expr, $x_start:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETADCXX_value!($CntSetMask, $x_end2, $x_start)); }
    };
}
pub use SETADCXX_value;
pub use SETADCXX_valid;
pub use SETADCXX;
pub use SETADCXXd;
pub use brisc_SETADCXXd;

#[macro_export]
macro_rules! SETADCXY_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        { $crate::tt_op!(0x51, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6) | ($BitMask << 0)) }
    };
}
#[macro_export]
macro_rules! SETADCXY_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3) && $crate::asm::is_valid(BitMask, 6)
    };
}
#[macro_export]
macro_rules! SETADCXY {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask))); }
    };
}
#[macro_export]
macro_rules! SETADCXYd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::push_instrn($crate::asm::SETADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETADCXYd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETADCXY_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
pub use SETADCXY_value;
pub use SETADCXY_valid;
pub use SETADCXY;
pub use SETADCXYd;
pub use brisc_SETADCXYd;

#[macro_export]
macro_rules! SETADCZW_value {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        { $crate::tt_op!(0x54, ($CntSetMask << 21) | ($Ch1_Y << 15) | ($Ch1_X << 12) | ($Ch0_Y << 9) | ($Ch0_X << 6) | ($BitMask << 0)) }
    };
}
#[macro_export]
macro_rules! SETADCZW_valid {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
        $crate::asm::is_valid(CntSetMask, 3) && $crate::asm::is_valid(Ch1_Y, 6) && $crate::asm::is_valid(Ch1_X, 3) && $crate::asm::is_valid(Ch0_Y, 3) && $crate::asm::is_valid(Ch0_X, 3) && $crate::asm::is_valid(BitMask, 6)
    };
}
#[macro_export]
macro_rules! SETADCZW {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask))); }
    };
}
#[macro_export]
macro_rules! SETADCZWd {
    ($CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::push_instrn($crate::asm::SETADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETADCZWd {
    ($core:path, $CntSetMask:expr, $Ch1_Y:expr, $Ch1_X:expr, $Ch0_Y:expr, $Ch0_X:expr, $BitMask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETADCZW_value!($CntSetMask, $Ch1_Y, $Ch1_X, $Ch0_Y, $Ch0_X, $BitMask)); }
    };
}
pub use SETADCZW_value;
pub use SETADCZW_valid;
pub use SETADCZW;
pub use SETADCZWd;
pub use brisc_SETADCZWd;

#[macro_export]
macro_rules! SETASHRMH_value {
    ($reg_mask:expr, $halo_mask:expr) => {
        { $crate::tt_op!(0x1e, ($reg_mask << 1) | ($halo_mask << 0)) }
    };
}
#[macro_export]
macro_rules! SETASHRMH_valid {
    ($reg_mask:expr, $halo_mask:expr) => {
        $crate::asm::is_valid(reg_mask, 23) && $crate::asm::is_valid(halo_mask, 1)
    };
}
#[macro_export]
macro_rules! SETASHRMH {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETASHRMH_value!($reg_mask, $halo_mask))); }
    };
}
#[macro_export]
macro_rules! SETASHRMHd {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::push_instrn($crate::asm::SETASHRMH_value!($reg_mask, $halo_mask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETASHRMHd {
    ($core:path, $reg_mask:expr, $halo_mask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETASHRMH_value!($reg_mask, $halo_mask)); }
    };
}
pub use SETASHRMH_value;
pub use SETASHRMH_valid;
pub use SETASHRMH;
pub use SETASHRMHd;
pub use brisc_SETASHRMHd;

#[macro_export]
macro_rules! SETASHRMH0_value {
    ($reg_mask:expr, $halo_mask:expr) => {
        { $crate::tt_op!(0x1a, ($reg_mask << 1) | ($halo_mask << 0)) }
    };
}
#[macro_export]
macro_rules! SETASHRMH0_valid {
    ($reg_mask:expr, $halo_mask:expr) => {
        $crate::asm::is_valid(reg_mask, 23) && $crate::asm::is_valid(halo_mask, 1)
    };
}
#[macro_export]
macro_rules! SETASHRMH0 {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETASHRMH0_value!($reg_mask, $halo_mask))); }
    };
}
#[macro_export]
macro_rules! SETASHRMH0d {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::push_instrn($crate::asm::SETASHRMH0_value!($reg_mask, $halo_mask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETASHRMH0d {
    ($core:path, $reg_mask:expr, $halo_mask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETASHRMH0_value!($reg_mask, $halo_mask)); }
    };
}
pub use SETASHRMH0_value;
pub use SETASHRMH0_valid;
pub use SETASHRMH0;
pub use SETASHRMH0d;
pub use brisc_SETASHRMH0d;

#[macro_export]
macro_rules! SETASHRMH1_value {
    ($reg_mask:expr, $halo_mask:expr) => {
        { $crate::tt_op!(0x1b, ($reg_mask << 1) | ($halo_mask << 0)) }
    };
}
#[macro_export]
macro_rules! SETASHRMH1_valid {
    ($reg_mask:expr, $halo_mask:expr) => {
        $crate::asm::is_valid(reg_mask, 23) && $crate::asm::is_valid(halo_mask, 1)
    };
}
#[macro_export]
macro_rules! SETASHRMH1 {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETASHRMH1_value!($reg_mask, $halo_mask))); }
    };
}
#[macro_export]
macro_rules! SETASHRMH1d {
    ($reg_mask:expr, $halo_mask:expr) => {
    { $crate::push_instrn($crate::asm::SETASHRMH1_value!($reg_mask, $halo_mask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETASHRMH1d {
    ($core:path, $reg_mask:expr, $halo_mask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETASHRMH1_value!($reg_mask, $halo_mask)); }
    };
}
pub use SETASHRMH1_value;
pub use SETASHRMH1_valid;
pub use SETASHRMH1;
pub use SETASHRMH1d;
pub use brisc_SETASHRMH1d;

#[macro_export]
macro_rules! SETASHRMV_value {
    ($reg_mask2:expr) => {
        { $crate::tt_op!(0x1c, ($reg_mask2 << 0)) }
    };
}
#[macro_export]
macro_rules! SETASHRMV_valid {
    ($reg_mask2:expr) => {
        $crate::asm::is_valid(reg_mask2, 24)
    };
}
#[macro_export]
macro_rules! SETASHRMV {
    ($reg_mask2:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETASHRMV_value!($reg_mask2))); }
    };
}
#[macro_export]
macro_rules! SETASHRMVd {
    ($reg_mask2:expr) => {
    { $crate::push_instrn($crate::asm::SETASHRMV_value!($reg_mask2)); }
    };
}
#[macro_export]
macro_rules! brisc_SETASHRMVd {
    ($core:path, $reg_mask2:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETASHRMV_value!($reg_mask2)); }
    };
}
pub use SETASHRMV_value;
pub use SETASHRMV_valid;
pub use SETASHRMV;
pub use SETASHRMVd;
pub use brisc_SETASHRMVd;

#[macro_export]
macro_rules! SETC16_value {
    ($setc16_reg:expr, $setc16_value:expr) => {
        { $crate::tt_op!(0xb2, ($setc16_reg << 16) | ($setc16_value << 0)) }
    };
}
#[macro_export]
macro_rules! SETC16_valid {
    ($setc16_reg:expr, $setc16_value:expr) => {
        $crate::asm::is_valid(setc16_reg, 8) && $crate::asm::is_valid(setc16_value, 16)
    };
}
#[macro_export]
macro_rules! SETC16 {
    ($setc16_reg:expr, $setc16_value:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETC16_value!($setc16_reg, $setc16_value))); }
    };
}
#[macro_export]
macro_rules! SETC16d {
    ($setc16_reg:expr, $setc16_value:expr) => {
    { $crate::push_instrn($crate::asm::SETC16_value!($setc16_reg, $setc16_value)); }
    };
}
#[macro_export]
macro_rules! brisc_SETC16d {
    ($core:path, $setc16_reg:expr, $setc16_value:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETC16_value!($setc16_reg, $setc16_value)); }
    };
}
pub use SETC16_value;
pub use SETC16_valid;
pub use SETC16;
pub use SETC16d;
pub use brisc_SETC16d;

#[macro_export]
macro_rules! SETDMAREG_value {
    ($Payload_SigSelSize:expr, $Payload_SigSel:expr, $SetSignalsMode:expr, $RegIndex16b:expr) => {
        { $crate::tt_op!(0x45, ($Payload_SigSelSize << 22) | ($Payload_SigSel << 8) | ($SetSignalsMode << 7) | ($RegIndex16b << 0)) }
    };
}
#[macro_export]
macro_rules! SETDMAREG_valid {
    ($Payload_SigSelSize:expr, $Payload_SigSel:expr, $SetSignalsMode:expr, $RegIndex16b:expr) => {
        $crate::asm::is_valid(Payload_SigSelSize, 2) && $crate::asm::is_valid(Payload_SigSel, 14) && $crate::asm::is_valid(SetSignalsMode, 1) && $crate::asm::is_valid(RegIndex16b, 7)
    };
}
#[macro_export]
macro_rules! SETDMAREG {
    ($Payload_SigSelSize:expr, $Payload_SigSel:expr, $SetSignalsMode:expr, $RegIndex16b:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETDMAREG_value!($Payload_SigSelSize, $Payload_SigSel, $SetSignalsMode, $RegIndex16b))); }
    };
}
#[macro_export]
macro_rules! SETDMAREGd {
    ($Payload_SigSelSize:expr, $Payload_SigSel:expr, $SetSignalsMode:expr, $RegIndex16b:expr) => {
    { $crate::push_instrn($crate::asm::SETDMAREG_value!($Payload_SigSelSize, $Payload_SigSel, $SetSignalsMode, $RegIndex16b)); }
    };
}
#[macro_export]
macro_rules! brisc_SETDMAREGd {
    ($core:path, $Payload_SigSelSize:expr, $Payload_SigSel:expr, $SetSignalsMode:expr, $RegIndex16b:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETDMAREG_value!($Payload_SigSelSize, $Payload_SigSel, $SetSignalsMode, $RegIndex16b)); }
    };
}
pub use SETDMAREG_value;
pub use SETDMAREG_valid;
pub use SETDMAREG;
pub use SETDMAREGd;
pub use brisc_SETDMAREGd;

#[macro_export]
macro_rules! SETDVALID_value {
    ($setvalid:expr) => {
        { $crate::tt_op!(0x57, ($setvalid << 0)) }
    };
}
#[macro_export]
macro_rules! SETDVALID_valid {
    ($setvalid:expr) => {
        $crate::asm::is_valid(setvalid, 24)
    };
}
#[macro_export]
macro_rules! SETDVALID {
    ($setvalid:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETDVALID_value!($setvalid))); }
    };
}
#[macro_export]
macro_rules! SETDVALIDd {
    ($setvalid:expr) => {
    { $crate::push_instrn($crate::asm::SETDVALID_value!($setvalid)); }
    };
}
#[macro_export]
macro_rules! brisc_SETDVALIDd {
    ($core:path, $setvalid:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETDVALID_value!($setvalid)); }
    };
}
pub use SETDVALID_value;
pub use SETDVALID_valid;
pub use SETDVALID;
pub use SETDVALIDd;
pub use brisc_SETDVALIDd;

#[macro_export]
macro_rules! SETPKEDGOF_value {
    ($y_end:expr, $y_start:expr, $x_end:expr, $x_start:expr) => {
        { $crate::tt_op!(0x1d, ($y_end << 12) | ($y_start << 8) | ($x_end << 4) | ($x_start << 0)) }
    };
}
#[macro_export]
macro_rules! SETPKEDGOF_valid {
    ($y_end:expr, $y_start:expr, $x_end:expr, $x_start:expr) => {
        $crate::asm::is_valid(y_end, 12) && $crate::asm::is_valid(y_start, 4) && $crate::asm::is_valid(x_end, 4) && $crate::asm::is_valid(x_start, 4)
    };
}
#[macro_export]
macro_rules! SETPKEDGOF {
    ($y_end:expr, $y_start:expr, $x_end:expr, $x_start:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETPKEDGOF_value!($y_end, $y_start, $x_end, $x_start))); }
    };
}
#[macro_export]
macro_rules! SETPKEDGOFd {
    ($y_end:expr, $y_start:expr, $x_end:expr, $x_start:expr) => {
    { $crate::push_instrn($crate::asm::SETPKEDGOF_value!($y_end, $y_start, $x_end, $x_start)); }
    };
}
#[macro_export]
macro_rules! brisc_SETPKEDGOFd {
    ($core:path, $y_end:expr, $y_start:expr, $x_end:expr, $x_start:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETPKEDGOF_value!($y_end, $y_start, $x_end, $x_start)); }
    };
}
pub use SETPKEDGOF_value;
pub use SETPKEDGOF_valid;
pub use SETPKEDGOF;
pub use SETPKEDGOFd;
pub use brisc_SETPKEDGOFd;

#[macro_export]
macro_rules! SETRWC_value {
    ($clear_ab_vld:expr, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr, $BitMask:expr) => {
        { $crate::tt_op!(0x37, ($clear_ab_vld << 22) | ($rwc_cr << 18) | ($rwc_d << 14) | ($rwc_b << 10) | ($rwc_a << 6) | ($BitMask << 0)) }
    };
}
#[macro_export]
macro_rules! SETRWC_valid {
    ($clear_ab_vld:expr, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr, $BitMask:expr) => {
        $crate::asm::is_valid(clear_ab_vld, 2) && $crate::asm::is_valid(rwc_cr, 4) && $crate::asm::is_valid(rwc_d, 4) && $crate::asm::is_valid(rwc_b, 4) && $crate::asm::is_valid(rwc_a, 4) && $crate::asm::is_valid(BitMask, 6)
    };
}
#[macro_export]
macro_rules! SETRWC {
    ($clear_ab_vld:expr, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr, $BitMask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SETRWC_value!($clear_ab_vld, $rwc_cr, $rwc_d, $rwc_b, $rwc_a, $BitMask))); }
    };
}
#[macro_export]
macro_rules! SETRWCd {
    ($clear_ab_vld:expr, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr, $BitMask:expr) => {
    { $crate::push_instrn($crate::asm::SETRWC_value!($clear_ab_vld, $rwc_cr, $rwc_d, $rwc_b, $rwc_a, $BitMask)); }
    };
}
#[macro_export]
macro_rules! brisc_SETRWCd {
    ($core:path, $clear_ab_vld:expr, $rwc_cr:expr, $rwc_d:expr, $rwc_b:expr, $rwc_a:expr, $BitMask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SETRWC_value!($clear_ab_vld, $rwc_cr, $rwc_d, $rwc_b, $rwc_a, $BitMask)); }
    };
}
pub use SETRWC_value;
pub use SETRWC_valid;
pub use SETRWC;
pub use SETRWCd;
pub use brisc_SETRWCd;

#[macro_export]
macro_rules! SFPABS_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7d, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPABS_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPABS {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPABS_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPABSd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPABS_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPABSd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPABS_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPABS_value;
pub use SFPABS_valid;
pub use SFPABS;
pub use SFPABSd;
pub use brisc_SFPABSd;

#[macro_export]
macro_rules! SFPADD_value {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x85, ($lreg_src_a << 16) | ($lreg_src_b << 12) | ($lreg_src_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPADD_valid {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(lreg_src_a, 8) && $crate::asm::is_valid(lreg_src_b, 4) && $crate::asm::is_valid(lreg_src_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPADD {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPADD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPADDd {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPADD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPADDd {
    ($core:path, $lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPADD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPADD_value;
pub use SFPADD_valid;
pub use SFPADD;
pub use SFPADDd;
pub use brisc_SFPADDd;

#[macro_export]
macro_rules! SFPADDI_value {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x75, ($imm16_math << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPADDI_valid {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm16_math, 16) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPADDI {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPADDI_value!($imm16_math, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPADDId {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPADDI_value!($imm16_math, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPADDId {
    ($core:path, $imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPADDI_value!($imm16_math, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPADDI_value;
pub use SFPADDI_valid;
pub use SFPADDI;
pub use SFPADDId;
pub use brisc_SFPADDId;

#[macro_export]
macro_rules! SFPAND_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7e, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPAND_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPAND {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPAND_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPANDd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPAND_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPANDd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPAND_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPAND_value;
pub use SFPAND_valid;
pub use SFPAND;
pub use SFPANDd;
pub use brisc_SFPANDd;

#[macro_export]
macro_rules! SFPCOMPC_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x8b, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPCOMPC_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPCOMPC {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPCOMPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPCOMPCd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPCOMPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPCOMPCd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPCOMPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPCOMPC_value;
pub use SFPCOMPC_valid;
pub use SFPCOMPC;
pub use SFPCOMPCd;
pub use brisc_SFPCOMPCd;

#[macro_export]
macro_rules! SFPDIVP2_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x76, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPDIVP2_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPDIVP2 {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPDIVP2_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPDIVP2d {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPDIVP2_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPDIVP2d {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPDIVP2_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPDIVP2_value;
pub use SFPDIVP2_valid;
pub use SFPDIVP2;
pub use SFPDIVP2d;
pub use brisc_SFPDIVP2d;

#[macro_export]
macro_rules! SFPENCC_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x8a, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPENCC_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPENCC {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPENCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPENCCd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPENCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPENCCd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPENCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPENCC_value;
pub use SFPENCC_valid;
pub use SFPENCC;
pub use SFPENCCd;
pub use brisc_SFPENCCd;

#[macro_export]
macro_rules! SFPEXEXP_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x77, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPEXEXP_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPEXEXP {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPEXEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPEXEXPd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPEXEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPEXEXPd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPEXEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPEXEXP_value;
pub use SFPEXEXP_valid;
pub use SFPEXEXP;
pub use SFPEXEXPd;
pub use brisc_SFPEXEXPd;

#[macro_export]
macro_rules! SFPEXMAN_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x78, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPEXMAN_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPEXMAN {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPEXMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPEXMANd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPEXMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPEXMANd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPEXMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPEXMAN_value;
pub use SFPEXMAN_valid;
pub use SFPEXMAN;
pub use SFPEXMANd;
pub use brisc_SFPEXMANd;

#[macro_export]
macro_rules! SFPIADD_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x79, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPIADD_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPIADD {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPIADD_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPIADDd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPIADD_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPIADDd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPIADD_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPIADD_value;
pub use SFPIADD_valid;
pub use SFPIADD;
pub use SFPIADDd;
pub use brisc_SFPIADDd;

#[macro_export]
macro_rules! SFPLOAD_value {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        { $crate::tt_op!(0x70, ($lreg_ind << 20) | ($instr_mod0 << 16) | ($dest_reg_addr << 0)) }
    };
}
#[macro_export]
macro_rules! SFPLOAD_valid {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        $crate::asm::is_valid(lreg_ind, 4) && $crate::asm::is_valid(instr_mod0, 4) && $crate::asm::is_valid(dest_reg_addr, 16)
    };
}
#[macro_export]
macro_rules! SFPLOAD {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPLOAD_value!($lreg_ind, $instr_mod0, $dest_reg_addr))); }
    };
}
#[macro_export]
macro_rules! SFPLOADd {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::push_instrn($crate::asm::SFPLOAD_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPLOADd {
    ($core:path, $lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPLOAD_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
pub use SFPLOAD_value;
pub use SFPLOAD_valid;
pub use SFPLOAD;
pub use SFPLOADd;
pub use brisc_SFPLOADd;

#[macro_export]
macro_rules! SFPLOADI_value {
    ($lreg_ind:expr, $instr_mod0:expr, $imm16:expr) => {
        { $crate::tt_op!(0x71, ($lreg_ind << 20) | ($instr_mod0 << 16) | ($imm16 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPLOADI_valid {
    ($lreg_ind:expr, $instr_mod0:expr, $imm16:expr) => {
        $crate::asm::is_valid(lreg_ind, 4) && $crate::asm::is_valid(instr_mod0, 4) && $crate::asm::is_valid(imm16, 16)
    };
}
#[macro_export]
macro_rules! SFPLOADI {
    ($lreg_ind:expr, $instr_mod0:expr, $imm16:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPLOADI_value!($lreg_ind, $instr_mod0, $imm16))); }
    };
}
#[macro_export]
macro_rules! SFPLOADId {
    ($lreg_ind:expr, $instr_mod0:expr, $imm16:expr) => {
    { $crate::push_instrn($crate::asm::SFPLOADI_value!($lreg_ind, $instr_mod0, $imm16)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPLOADId {
    ($core:path, $lreg_ind:expr, $instr_mod0:expr, $imm16:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPLOADI_value!($lreg_ind, $instr_mod0, $imm16)); }
    };
}
pub use SFPLOADI_value;
pub use SFPLOADI_valid;
pub use SFPLOADI;
pub use SFPLOADId;
pub use brisc_SFPLOADId;

#[macro_export]
macro_rules! SFPLUT_value {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        { $crate::tt_op!(0x73, ($lreg_ind << 20) | ($instr_mod0 << 16) | ($dest_reg_addr << 0)) }
    };
}
#[macro_export]
macro_rules! SFPLUT_valid {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        $crate::asm::is_valid(lreg_ind, 4) && $crate::asm::is_valid(instr_mod0, 4) && $crate::asm::is_valid(dest_reg_addr, 16)
    };
}
#[macro_export]
macro_rules! SFPLUT {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPLUT_value!($lreg_ind, $instr_mod0, $dest_reg_addr))); }
    };
}
#[macro_export]
macro_rules! SFPLUTd {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::push_instrn($crate::asm::SFPLUT_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPLUTd {
    ($core:path, $lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPLUT_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
pub use SFPLUT_value;
pub use SFPLUT_valid;
pub use SFPLUT;
pub use SFPLUTd;
pub use brisc_SFPLUTd;

#[macro_export]
macro_rules! SFPLZ_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x81, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPLZ_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPLZ {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPLZ_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPLZd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPLZ_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPLZd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPLZ_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPLZ_value;
pub use SFPLZ_valid;
pub use SFPLZ;
pub use SFPLZd;
pub use brisc_SFPLZd;

#[macro_export]
macro_rules! SFPMAD_value {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x84, ($lreg_src_a << 16) | ($lreg_src_b << 12) | ($lreg_src_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPMAD_valid {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(lreg_src_a, 8) && $crate::asm::is_valid(lreg_src_b, 4) && $crate::asm::is_valid(lreg_src_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPMAD {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPMAD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPMADd {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPMAD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPMADd {
    ($core:path, $lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPMAD_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPMAD_value;
pub use SFPMAD_valid;
pub use SFPMAD;
pub use SFPMADd;
pub use brisc_SFPMADd;

#[macro_export]
macro_rules! SFPMOV_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7c, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPMOV_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPMOV {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPMOV_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPMOVd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPMOV_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPMOVd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPMOV_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPMOV_value;
pub use SFPMOV_valid;
pub use SFPMOV;
pub use SFPMOVd;
pub use brisc_SFPMOVd;

#[macro_export]
macro_rules! SFPMUL_value {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x86, ($lreg_src_a << 16) | ($lreg_src_b << 12) | ($lreg_src_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPMUL_valid {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(lreg_src_a, 8) && $crate::asm::is_valid(lreg_src_b, 4) && $crate::asm::is_valid(lreg_src_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPMUL {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPMUL_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPMULd {
    ($lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPMUL_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPMULd {
    ($core:path, $lreg_src_a:expr, $lreg_src_b:expr, $lreg_src_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPMUL_value!($lreg_src_a, $lreg_src_b, $lreg_src_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPMUL_value;
pub use SFPMUL_valid;
pub use SFPMUL;
pub use SFPMULd;
pub use brisc_SFPMULd;

#[macro_export]
macro_rules! SFPMULI_value {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x74, ($imm16_math << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPMULI_valid {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm16_math, 16) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPMULI {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPMULI_value!($imm16_math, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPMULId {
    ($imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPMULI_value!($imm16_math, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPMULId {
    ($core:path, $imm16_math:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPMULI_value!($imm16_math, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPMULI_value;
pub use SFPMULI_valid;
pub use SFPMULI;
pub use SFPMULId;
pub use brisc_SFPMULId;

#[macro_export]
macro_rules! SFPNOT_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x80, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPNOT_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPNOT {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPNOT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPNOTd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPNOT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPNOTd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPNOT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPNOT_value;
pub use SFPNOT_valid;
pub use SFPNOT;
pub use SFPNOTd;
pub use brisc_SFPNOTd;

#[macro_export]
macro_rules! SFPOR_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7f, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPOR_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPOR {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPOR_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPORd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPOR_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPORd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPOR_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPOR_value;
pub use SFPOR_valid;
pub use SFPOR;
pub use SFPORd;
pub use brisc_SFPORd;

#[macro_export]
macro_rules! SFPPOPC_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x88, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPPOPC_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPPOPC {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPPOPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPPOPCd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPPOPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPPOPCd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPPOPC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPPOPC_value;
pub use SFPPOPC_valid;
pub use SFPPOPC;
pub use SFPPOPCd;
pub use brisc_SFPPOPCd;

#[macro_export]
macro_rules! SFPPUSHC_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x87, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPPUSHC_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPPUSHC {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPPUSHC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPPUSHCd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPPUSHC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPPUSHCd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPPUSHC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPPUSHC_value;
pub use SFPPUSHC_valid;
pub use SFPPUSHC;
pub use SFPPUSHCd;
pub use brisc_SFPPUSHCd;

#[macro_export]
macro_rules! SFPSETCC_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7b, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSETCC_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPSETCC {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSETCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPSETCCd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPSETCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSETCCd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSETCC_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPSETCC_value;
pub use SFPSETCC_valid;
pub use SFPSETCC;
pub use SFPSETCCd;
pub use brisc_SFPSETCCd;

#[macro_export]
macro_rules! SFPSETEXP_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x82, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSETEXP_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPSETEXP {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSETEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPSETEXPd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPSETEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSETEXPd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSETEXP_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPSETEXP_value;
pub use SFPSETEXP_valid;
pub use SFPSETEXP;
pub use SFPSETEXPd;
pub use brisc_SFPSETEXPd;

#[macro_export]
macro_rules! SFPSETMAN_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x83, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSETMAN_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPSETMAN {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSETMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPSETMANd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPSETMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSETMANd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSETMAN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPSETMAN_value;
pub use SFPSETMAN_valid;
pub use SFPSETMAN;
pub use SFPSETMANd;
pub use brisc_SFPSETMANd;

#[macro_export]
macro_rules! SFPSETSGN_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x89, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSETSGN_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPSETSGN {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSETSGN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPSETSGNd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPSETSGN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSETSGNd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSETSGN_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPSETSGN_value;
pub use SFPSETSGN_valid;
pub use SFPSETSGN;
pub use SFPSETSGNd;
pub use brisc_SFPSETSGNd;

#[macro_export]
macro_rules! SFPSHFT_value {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        { $crate::tt_op!(0x7a, ($imm12_math << 12) | ($lreg_c << 8) | ($lreg_dest << 4) | ($instr_mod1 << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSHFT_valid {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
        $crate::asm::is_valid(imm12_math, 12) && $crate::asm::is_valid(lreg_c, 4) && $crate::asm::is_valid(lreg_dest, 4) && $crate::asm::is_valid(instr_mod1, 4)
    };
}
#[macro_export]
macro_rules! SFPSHFT {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSHFT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1))); }
    };
}
#[macro_export]
macro_rules! SFPSHFTd {
    ($imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::push_instrn($crate::asm::SFPSHFT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSHFTd {
    ($core:path, $imm12_math:expr, $lreg_c:expr, $lreg_dest:expr, $instr_mod1:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSHFT_value!($imm12_math, $lreg_c, $lreg_dest, $instr_mod1)); }
    };
}
pub use SFPSHFT_value;
pub use SFPSHFT_valid;
pub use SFPSHFT;
pub use SFPSHFTd;
pub use brisc_SFPSHFTd;

#[macro_export]
macro_rules! SFPSTORE_value {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        { $crate::tt_op!(0x72, ($lreg_ind << 20) | ($instr_mod0 << 16) | ($dest_reg_addr << 0)) }
    };
}
#[macro_export]
macro_rules! SFPSTORE_valid {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
        $crate::asm::is_valid(lreg_ind, 4) && $crate::asm::is_valid(instr_mod0, 4) && $crate::asm::is_valid(dest_reg_addr, 16)
    };
}
#[macro_export]
macro_rules! SFPSTORE {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SFPSTORE_value!($lreg_ind, $instr_mod0, $dest_reg_addr))); }
    };
}
#[macro_export]
macro_rules! SFPSTOREd {
    ($lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::push_instrn($crate::asm::SFPSTORE_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
#[macro_export]
macro_rules! brisc_SFPSTOREd {
    ($core:path, $lreg_ind:expr, $instr_mod0:expr, $dest_reg_addr:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SFPSTORE_value!($lreg_ind, $instr_mod0, $dest_reg_addr)); }
    };
}
pub use SFPSTORE_value;
pub use SFPSTORE_valid;
pub use SFPSTORE;
pub use SFPSTOREd;
pub use brisc_SFPSTOREd;

#[macro_export]
macro_rules! SHIFTDMAREG_value {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x5c, ($OpBisConst << 23) | ($OpSel << 18) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! SHIFTDMAREG_valid {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(OpSel, 5) && $crate::asm::is_valid(ResultRegIndex, 6) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! SHIFTDMAREG {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SHIFTDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! SHIFTDMAREGd {
    ($OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::SHIFTDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_SHIFTDMAREGd {
    ($core:path, $OpBisConst:expr, $OpSel:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SHIFTDMAREG_value!($OpBisConst, $OpSel, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use SHIFTDMAREG_value;
pub use SHIFTDMAREG_valid;
pub use SHIFTDMAREG;
pub use SHIFTDMAREGd;
pub use brisc_SHIFTDMAREGd;

#[macro_export]
macro_rules! SHIFTXA_value {
    ($log2_amount2:expr, $shift_mode:expr) => {
        { $crate::tt_op!(0x17, ($log2_amount2 << 2) | ($shift_mode << 0)) }
    };
}
#[macro_export]
macro_rules! SHIFTXA_valid {
    ($log2_amount2:expr, $shift_mode:expr) => {
        $crate::asm::is_valid(log2_amount2, 22) && $crate::asm::is_valid(shift_mode, 2)
    };
}
#[macro_export]
macro_rules! SHIFTXA {
    ($log2_amount2:expr, $shift_mode:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SHIFTXA_value!($log2_amount2, $shift_mode))); }
    };
}
#[macro_export]
macro_rules! SHIFTXAd {
    ($log2_amount2:expr, $shift_mode:expr) => {
    { $crate::push_instrn($crate::asm::SHIFTXA_value!($log2_amount2, $shift_mode)); }
    };
}
#[macro_export]
macro_rules! brisc_SHIFTXAd {
    ($core:path, $log2_amount2:expr, $shift_mode:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SHIFTXA_value!($log2_amount2, $shift_mode)); }
    };
}
pub use SHIFTXA_value;
pub use SHIFTXA_valid;
pub use SHIFTXA;
pub use SHIFTXAd;
pub use brisc_SHIFTXAd;

#[macro_export]
macro_rules! STALLWAIT_value {
    ($stall_res:expr, $wait_res:expr) => {
        { $crate::tt_op!(0xa2, ($stall_res << 14) | ($wait_res << 0)) }
    };
}
#[macro_export]
macro_rules! STALLWAIT_valid {
    ($stall_res:expr, $wait_res:expr) => {
        $crate::asm::is_valid(stall_res, 10) && $crate::asm::is_valid(wait_res, 14)
    };
}
#[macro_export]
macro_rules! STALLWAIT {
    ($stall_res:expr, $wait_res:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::STALLWAIT_value!($stall_res, $wait_res))); }
    };
}
#[macro_export]
macro_rules! STALLWAITd {
    ($stall_res:expr, $wait_res:expr) => {
    { $crate::push_instrn($crate::asm::STALLWAIT_value!($stall_res, $wait_res)); }
    };
}
#[macro_export]
macro_rules! brisc_STALLWAITd {
    ($core:path, $stall_res:expr, $wait_res:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::STALLWAIT_value!($stall_res, $wait_res)); }
    };
}
pub use STALLWAIT_value;
pub use STALLWAIT_valid;
pub use STALLWAIT;
pub use STALLWAITd;
pub use brisc_STALLWAITd;

#[macro_export]
macro_rules! STOREIND_value {
    ($MemHierSel:expr, $SizeSel:expr, $RegSizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        { $crate::tt_op!(0x66, ($MemHierSel << 23) | ($SizeSel << 22) | ($RegSizeSel << 21) | ($OffsetIndex << 14) | ($AutoIncSpec << 12) | ($DataRegIndex << 6) | ($AddrRegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! STOREIND_valid {
    ($MemHierSel:expr, $SizeSel:expr, $RegSizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
        $crate::asm::is_valid(MemHierSel, 1) && $crate::asm::is_valid(SizeSel, 1) && $crate::asm::is_valid(RegSizeSel, 1) && $crate::asm::is_valid(OffsetIndex, 7) && $crate::asm::is_valid(AutoIncSpec, 2) && $crate::asm::is_valid(DataRegIndex, 6) && $crate::asm::is_valid(AddrRegIndex, 6)
    };
}
#[macro_export]
macro_rules! STOREIND {
    ($MemHierSel:expr, $SizeSel:expr, $RegSizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::STOREIND_value!($MemHierSel, $SizeSel, $RegSizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex))); }
    };
}
#[macro_export]
macro_rules! STOREINDd {
    ($MemHierSel:expr, $SizeSel:expr, $RegSizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::push_instrn($crate::asm::STOREIND_value!($MemHierSel, $SizeSel, $RegSizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_STOREINDd {
    ($core:path, $MemHierSel:expr, $SizeSel:expr, $RegSizeSel:expr, $OffsetIndex:expr, $AutoIncSpec:expr, $DataRegIndex:expr, $AddrRegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::STOREIND_value!($MemHierSel, $SizeSel, $RegSizeSel, $OffsetIndex, $AutoIncSpec, $DataRegIndex, $AddrRegIndex)); }
    };
}
pub use STOREIND_value;
pub use STOREIND_valid;
pub use STOREIND;
pub use STOREINDd;
pub use brisc_STOREINDd;

#[macro_export]
macro_rules! STOREREG_value {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
        { $crate::tt_op!(0x67, ($TdmaDataRegIndex << 18) | ($RegAddr << 0)) }
    };
}
#[macro_export]
macro_rules! STOREREG_valid {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
        $crate::asm::is_valid(TdmaDataRegIndex, 6) && $crate::asm::is_valid(RegAddr, 18)
    };
}
#[macro_export]
macro_rules! STOREREG {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::STOREREG_value!($TdmaDataRegIndex, $RegAddr))); }
    };
}
#[macro_export]
macro_rules! STOREREGd {
    ($TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::push_instrn($crate::asm::STOREREG_value!($TdmaDataRegIndex, $RegAddr)); }
    };
}
#[macro_export]
macro_rules! brisc_STOREREGd {
    ($core:path, $TdmaDataRegIndex:expr, $RegAddr:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::STOREREG_value!($TdmaDataRegIndex, $RegAddr)); }
    };
}
pub use STOREREG_value;
pub use STOREREG_valid;
pub use STOREREG;
pub use STOREREGd;
pub use brisc_STOREREGd;

#[macro_export]
macro_rules! SUBDMAREG_value {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        { $crate::tt_op!(0x59, ($OpBisConst << 23) | ($ResultRegIndex << 12) | ($OpBRegIndex << 6) | ($OpARegIndex << 0)) }
    };
}
#[macro_export]
macro_rules! SUBDMAREG_valid {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
        $crate::asm::is_valid(OpBisConst, 1) && $crate::asm::is_valid(ResultRegIndex, 11) && $crate::asm::is_valid(OpBRegIndex, 6) && $crate::asm::is_valid(OpARegIndex, 6)
    };
}
#[macro_export]
macro_rules! SUBDMAREG {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::SUBDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex))); }
    };
}
#[macro_export]
macro_rules! SUBDMAREGd {
    ($OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::push_instrn($crate::asm::SUBDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
#[macro_export]
macro_rules! brisc_SUBDMAREGd {
    ($core:path, $OpBisConst:expr, $ResultRegIndex:expr, $OpBRegIndex:expr, $OpARegIndex:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::SUBDMAREG_value!($OpBisConst, $ResultRegIndex, $OpBRegIndex, $OpARegIndex)); }
    };
}
pub use SUBDMAREG_value;
pub use SUBDMAREG_valid;
pub use SUBDMAREG;
pub use SUBDMAREGd;
pub use brisc_SUBDMAREGd;

#[macro_export]
macro_rules! TRNSPSRCA_value {
    () => {
        { $crate::tt_op!(0x14) }
    };
}
#[macro_export]
macro_rules! TRNSPSRCA_valid {
    () => {
        true
    };
}
#[macro_export]
macro_rules! TRNSPSRCA {
    () => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::TRNSPSRCA_value!())); }
    };
}
#[macro_export]
macro_rules! TRNSPSRCAd {
    () => {
    { $crate::push_instrn($crate::asm::TRNSPSRCA_value!()); }
    };
}
#[macro_export]
macro_rules! brisc_TRNSPSRCAd {
    ($core:path, ) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::TRNSPSRCA_value!()); }
    };
}
pub use TRNSPSRCA_value;
pub use TRNSPSRCA_valid;
pub use TRNSPSRCA;
pub use TRNSPSRCAd;
pub use brisc_TRNSPSRCAd;

#[macro_export]
macro_rules! UNPACR_value {
    ($Unpack_block_selection:expr, $AddrMode:expr, $CfgContextCntInc:expr, $CfgContextId:expr, $AddrCntContextId:expr, $OvrdThreadId:expr, $SetDatValid:expr, $rareb_en:expr, $ZeroWrite2:expr, $AutoIncContextID:expr, $RowSearch:expr, $SearchCacheFlush:expr, $Last:expr) => {
        { $crate::tt_op!(0x42, ($Unpack_block_selection << 23) | ($AddrMode << 15) | ($CfgContextCntInc << 13) | ($CfgContextId << 10) | ($AddrCntContextId << 8) | ($OvrdThreadId << 7) | ($SetDatValid << 6) | ($rareb_en << 5) | ($ZeroWrite2 << 4) | ($AutoIncContextID << 3) | ($RowSearch << 2) | ($SearchCacheFlush << 1) | ($Last << 0)) }
    };
}
#[macro_export]
macro_rules! UNPACR_valid {
    ($Unpack_block_selection:expr, $AddrMode:expr, $CfgContextCntInc:expr, $CfgContextId:expr, $AddrCntContextId:expr, $OvrdThreadId:expr, $SetDatValid:expr, $rareb_en:expr, $ZeroWrite2:expr, $AutoIncContextID:expr, $RowSearch:expr, $SearchCacheFlush:expr, $Last:expr) => {
        $crate::asm::is_valid(Unpack_block_selection, 1) && $crate::asm::is_valid(AddrMode, 8) && $crate::asm::is_valid(CfgContextCntInc, 2) && $crate::asm::is_valid(CfgContextId, 3) && $crate::asm::is_valid(AddrCntContextId, 2) && $crate::asm::is_valid(OvrdThreadId, 1) && $crate::asm::is_valid(SetDatValid, 1) && $crate::asm::is_valid(rareb_en, 1) && $crate::asm::is_valid(ZeroWrite2, 1) && $crate::asm::is_valid(AutoIncContextID, 1) && $crate::asm::is_valid(RowSearch, 1) && $crate::asm::is_valid(SearchCacheFlush, 1) && $crate::asm::is_valid(Last, 1)
    };
}
#[macro_export]
macro_rules! UNPACR {
    ($Unpack_block_selection:expr, $AddrMode:expr, $CfgContextCntInc:expr, $CfgContextId:expr, $AddrCntContextId:expr, $OvrdThreadId:expr, $SetDatValid:expr, $rareb_en:expr, $ZeroWrite2:expr, $AutoIncContextID:expr, $RowSearch:expr, $SearchCacheFlush:expr, $Last:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::UNPACR_value!($Unpack_block_selection, $AddrMode, $CfgContextCntInc, $CfgContextId, $AddrCntContextId, $OvrdThreadId, $SetDatValid, $rareb_en, $ZeroWrite2, $AutoIncContextID, $RowSearch, $SearchCacheFlush, $Last))); }
    };
}
#[macro_export]
macro_rules! UNPACRd {
    ($Unpack_block_selection:expr, $AddrMode:expr, $CfgContextCntInc:expr, $CfgContextId:expr, $AddrCntContextId:expr, $OvrdThreadId:expr, $SetDatValid:expr, $rareb_en:expr, $ZeroWrite2:expr, $AutoIncContextID:expr, $RowSearch:expr, $SearchCacheFlush:expr, $Last:expr) => {
    { $crate::push_instrn($crate::asm::UNPACR_value!($Unpack_block_selection, $AddrMode, $CfgContextCntInc, $CfgContextId, $AddrCntContextId, $OvrdThreadId, $SetDatValid, $rareb_en, $ZeroWrite2, $AutoIncContextID, $RowSearch, $SearchCacheFlush, $Last)); }
    };
}
#[macro_export]
macro_rules! brisc_UNPACRd {
    ($core:path, $Unpack_block_selection:expr, $AddrMode:expr, $CfgContextCntInc:expr, $CfgContextId:expr, $AddrCntContextId:expr, $OvrdThreadId:expr, $SetDatValid:expr, $rareb_en:expr, $ZeroWrite2:expr, $AutoIncContextID:expr, $RowSearch:expr, $SearchCacheFlush:expr, $Last:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::UNPACR_value!($Unpack_block_selection, $AddrMode, $CfgContextCntInc, $CfgContextId, $AddrCntContextId, $OvrdThreadId, $SetDatValid, $rareb_en, $ZeroWrite2, $AutoIncContextID, $RowSearch, $SearchCacheFlush, $Last)); }
    };
}
pub use UNPACR_value;
pub use UNPACR_valid;
pub use UNPACR;
pub use UNPACRd;
pub use brisc_UNPACRd;

#[macro_export]
macro_rules! UNPACR_NOP_value {
    ($Unpack_block_selection:expr, $NoOp:expr) => {
        { $crate::tt_op!(0x43, ($Unpack_block_selection << 23) | ($NoOp << 0)) }
    };
}
#[macro_export]
macro_rules! UNPACR_NOP_valid {
    ($Unpack_block_selection:expr, $NoOp:expr) => {
        $crate::asm::is_valid(Unpack_block_selection, 1) && $crate::asm::is_valid(NoOp, 23)
    };
}
#[macro_export]
macro_rules! UNPACR_NOP {
    ($Unpack_block_selection:expr, $NoOp:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::UNPACR_NOP_value!($Unpack_block_selection, $NoOp))); }
    };
}
#[macro_export]
macro_rules! UNPACR_NOPd {
    ($Unpack_block_selection:expr, $NoOp:expr) => {
    { $crate::push_instrn($crate::asm::UNPACR_NOP_value!($Unpack_block_selection, $NoOp)); }
    };
}
#[macro_export]
macro_rules! brisc_UNPACR_NOPd {
    ($core:path, $Unpack_block_selection:expr, $NoOp:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::UNPACR_NOP_value!($Unpack_block_selection, $NoOp)); }
    };
}
pub use UNPACR_NOP_value;
pub use UNPACR_NOP_valid;
pub use UNPACR_NOP;
pub use UNPACR_NOPd;
pub use brisc_UNPACR_NOPd;

#[macro_export]
macro_rules! WRCFG_value {
    ($GprAddress:expr, $wr128b:expr, $CfgReg:expr) => {
        { $crate::tt_op!(0xb0, ($GprAddress << 16) | ($wr128b << 15) | ($CfgReg << 0)) }
    };
}
#[macro_export]
macro_rules! WRCFG_valid {
    ($GprAddress:expr, $wr128b:expr, $CfgReg:expr) => {
        $crate::asm::is_valid(GprAddress, 8) && $crate::asm::is_valid(wr128b, 1) && $crate::asm::is_valid(CfgReg, 15)
    };
}
#[macro_export]
macro_rules! WRCFG {
    ($GprAddress:expr, $wr128b:expr, $CfgReg:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::WRCFG_value!($GprAddress, $wr128b, $CfgReg))); }
    };
}
#[macro_export]
macro_rules! WRCFGd {
    ($GprAddress:expr, $wr128b:expr, $CfgReg:expr) => {
    { $crate::push_instrn($crate::asm::WRCFG_value!($GprAddress, $wr128b, $CfgReg)); }
    };
}
#[macro_export]
macro_rules! brisc_WRCFGd {
    ($core:path, $GprAddress:expr, $wr128b:expr, $CfgReg:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::WRCFG_value!($GprAddress, $wr128b, $CfgReg)); }
    };
}
pub use WRCFG_value;
pub use WRCFG_valid;
pub use WRCFG;
pub use WRCFGd;
pub use brisc_WRCFGd;

#[macro_export]
macro_rules! XMOV_value {
    ($Mov_block_selection:expr, $Last:expr) => {
        { $crate::tt_op!(0x40, ($Mov_block_selection << 23) | ($Last << 0)) }
    };
}
#[macro_export]
macro_rules! XMOV_valid {
    ($Mov_block_selection:expr, $Last:expr) => {
        $crate::asm::is_valid(Mov_block_selection, 1) && $crate::asm::is_valid(Last, 23)
    };
}
#[macro_export]
macro_rules! XMOV {
    ($Mov_block_selection:expr, $Last:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::XMOV_value!($Mov_block_selection, $Last))); }
    };
}
#[macro_export]
macro_rules! XMOVd {
    ($Mov_block_selection:expr, $Last:expr) => {
    { $crate::push_instrn($crate::asm::XMOV_value!($Mov_block_selection, $Last)); }
    };
}
#[macro_export]
macro_rules! brisc_XMOVd {
    ($core:path, $Mov_block_selection:expr, $Last:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::XMOV_value!($Mov_block_selection, $Last)); }
    };
}
pub use XMOV_value;
pub use XMOV_valid;
pub use XMOV;
pub use XMOVd;
pub use brisc_XMOVd;

#[macro_export]
macro_rules! ZEROACC_value {
    ($clear_mode:expr, $AddrMode:expr, $dst:expr) => {
        { $crate::tt_op!(0x10, ($clear_mode << 19) | ($AddrMode << 15) | ($dst << 0)) }
    };
}
#[macro_export]
macro_rules! ZEROACC_valid {
    ($clear_mode:expr, $AddrMode:expr, $dst:expr) => {
        $crate::asm::is_valid(clear_mode, 5) && $crate::asm::is_valid(AddrMode, 4) && $crate::asm::is_valid(dst, 15)
    };
}
#[macro_export]
macro_rules! ZEROACC {
    ($clear_mode:expr, $AddrMode:expr, $dst:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ZEROACC_value!($clear_mode, $AddrMode, $dst))); }
    };
}
#[macro_export]
macro_rules! ZEROACCd {
    ($clear_mode:expr, $AddrMode:expr, $dst:expr) => {
    { $crate::push_instrn($crate::asm::ZEROACC_value!($clear_mode, $AddrMode, $dst)); }
    };
}
#[macro_export]
macro_rules! brisc_ZEROACCd {
    ($core:path, $clear_mode:expr, $AddrMode:expr, $dst:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ZEROACC_value!($clear_mode, $AddrMode, $dst)); }
    };
}
pub use ZEROACC_value;
pub use ZEROACC_valid;
pub use ZEROACC;
pub use ZEROACCd;
pub use brisc_ZEROACCd;

#[macro_export]
macro_rules! ZEROSRC_value {
    ($zero_val:expr, $write_mode:expr, $bank_mask:expr, $src_mask:expr) => {
        { $crate::tt_op!(0x11, ($zero_val << 4) | ($write_mode << 3) | ($bank_mask << 2) | ($src_mask << 0)) }
    };
}
#[macro_export]
macro_rules! ZEROSRC_valid {
    ($zero_val:expr, $write_mode:expr, $bank_mask:expr, $src_mask:expr) => {
        $crate::asm::is_valid(zero_val, 20) && $crate::asm::is_valid(write_mode, 1) && $crate::asm::is_valid(bank_mask, 1) && $crate::asm::is_valid(src_mask, 2)
    };
}
#[macro_export]
macro_rules! ZEROSRC {
    ($zero_val:expr, $write_mode:expr, $bank_mask:expr, $src_mask:expr) => {
    { $crate::instruction_word!($crate::trisc_op_swizzle!($crate::asm::ZEROSRC_value!($zero_val, $write_mode, $bank_mask, $src_mask))); }
    };
}
#[macro_export]
macro_rules! ZEROSRCd {
    ($zero_val:expr, $write_mode:expr, $bank_mask:expr, $src_mask:expr) => {
    { $crate::push_instrn($crate::asm::ZEROSRC_value!($zero_val, $write_mode, $bank_mask, $src_mask)); }
    };
}
#[macro_export]
macro_rules! brisc_ZEROSRCd {
    ($core:path, $zero_val:expr, $write_mode:expr, $bank_mask:expr, $src_mask:expr) => {
    { $crate::brisc_push_instrn::<{ $core }>($crate::asm::ZEROSRC_value!($zero_val, $write_mode, $bank_mask, $src_mask)); }
    };
}
pub use ZEROSRC_value;
pub use ZEROSRC_valid;
pub use ZEROSRC;
pub use ZEROSRCd;
pub use brisc_ZEROSRCd;