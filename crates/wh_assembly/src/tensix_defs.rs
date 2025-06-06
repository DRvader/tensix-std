/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const MAX_THREADS: u32 = 3;
pub const MAX_PACKERS: u32 = 4;
pub const TEST_MAILBOX_ADDRESS: u32 = 4;
pub const WALL_CLOCK_MAILBOX_ADDRESS: u32 = 96;
pub const DEBUG_MAILBOX_ADDRESS: u32 = 112;
pub const DEBUG_MAILBOX_SIZE: u32 = 64;
pub const CQ_MAILBOX_ADDRESS: u32 = 368;
pub const CQ_MAILBOX_SIZE: u32 = 4;
pub const MEMORY_WORD_SIZE_IN_BYTES: u32 = 16;
pub const MEMORY_WORD_SHIFT_BITS: u32 = 4;
pub const STALLWAIT_COMPUTE: u32 = 0;
pub const STALLWAIT_TDMA: u32 = 1;
pub const STALLWAIT_FOR_TC: u32 = 1;
pub const STALLWAIT_FOR_UNP0: u32 = 2;
pub const STALLWAIT_FOR_UNP1: u32 = 4;
pub const STALLWAIT_FOR_PACK: u32 = 8;
pub const L0_BASE: u32 = 4290772992;
pub const L1_BASE: u32 = 0;
pub const LOCAL_MEM_SIZE: u32 = 4096;
pub const REGFILE_BASE: u32 = 4292870144;
pub const INSTRN_BUF_BASE: u32 = 4293132288;
pub const INSTRN1_BUF_BASE: u32 = 4293197824;
pub const INSTRN2_BUF_BASE: u32 = 4293263360;
pub const PC_BUF_BASE: u32 = 4293394432;
pub const PC1_BUF_BASE: u32 = 4293459968;
pub const PC2_BUF_BASE: u32 = 4293525504;
pub const TENSIX_MAILBOX0_BASE: u32 = 4293656576;
pub const TENSIX_MAILBOX1_BASE: u32 = 4293660672;
pub const TENSIX_MAILBOX2_BASE: u32 = 4293664768;
pub const TENSIX_MAILBOX3_BASE: u32 = 4293668864;
pub const TENSIX_CFG_BASE: u32 = 4293853184;
pub const TENSIX_MOP_CFG_BASE: u32 = 4290248704;
pub const L1_KERNEL_BASE: u32 = 126976;
pub const L1_L0_DUMP: u32 = 118784;
pub const LOCAL_MEM_BASE_ADDR: u32 = 4289724416;
pub const RISCV_TDMA_REGS_START_ADDR: u32 = 4289794048;
pub const RISCV_TDMA_REG_XMOV_SRC_ADDR: u32 = 4289794048;
pub const RISCV_TDMA_REG_XMOV_DST_ADDR: u32 = 4289794052;
pub const RISCV_TDMA_REG_XMOV_SIZE: u32 = 4289794056;
pub const RISCV_TDMA_REG_XMOV_DIRECTION: u32 = 4289794060;
pub const RISCV_TDMA_REG_COMMAND_ADDR: u32 = 4289794064;
pub const RISCV_TDMA_REG_STATUS: u32 = 4289794068;
pub const RISCV_TDMA_REG_PACKED_SIZE: u32 = 4289794072;
pub const RISCV_TDMA_REG_ACC_PACKED_SIZE: u32 = 4289794076;
pub const RISCV_TDMA_REG_INITIAL_PACK_ACC: u32 = 4289794076;
pub const RISCV_TDMA_REG_CLK_GATE_EN: u32 = 4289794084;
pub const RISCV_TDMA_REG_CLK_GATE_HYST: u32 = 4289794088;
pub const RISCV_TDMA_REG_XMOV_L1_BASE_ADDR: u32 = 4289794092;
pub const RISCV_TDMA_REG_FIFO_PACKED_TILE_STATUS: u32 = 4289794104;
pub const RISCV_TDMA_STATUS_FLAG_MOVER0_BUSY_MASK: u32 = 1;
pub const RISCV_TDMA_STATUS_FLAG_MOVER1_BUSY_MASK: u32 = 2;
pub const RISCV_TDMA_STATUS_FLAG_FIFO_FULL_MASK: u32 = 4;
pub const RISCV_TDMA_STATUS_FLAG_FIFO_EMPTY_MASK: u32 = 8;
pub const RISCV_TDMA_STATUS_FLAG_ERROR_MASK: u32 = 16;
pub const RISCV_DEBUG_REGS_START_ADDR: u32 = 4289798144;
pub const RISCV_DEBUG_REG_SOFT_RESET_0: u32 = 4289798576;
pub const RISCV_DEBUG_REG_WDT: u32 = 4289798624;
pub const RISCV_DEBUG_REG_WDT_CNTL: u32 = 4289798628;
pub const RISCV_DEBUG_REG_WDT_STATUS: u32 = 4289798632;
pub const RISCV_DEBUG_REG_BREAKPOINT_CTRL: u32 = 4289798592;
pub const RISCV_DEBUG_REG_BREAKPOINT_STATUS: u32 = 4289798596;
pub const RISCV_DEBUG_REG_BREAKPOINT_DATA: u32 = 4289798600;
pub const RISCV_DEBUG_REG_PERF_CNT_INSTRN_THREAD0: u32 = 4289798144;
pub const RISCV_DEBUG_REG_PERF_CNT_INSTRN_THREAD1: u32 = 4289798148;
pub const RISCV_DEBUG_REG_PERF_CNT_INSTRN_THREAD2: u32 = 4289798152;
pub const RISCV_DEBUG_REG_PERF_CNT_FPU0: u32 = 4289798168;
pub const RISCV_DEBUG_REG_PERF_CNT_FPU1: u32 = 4289798172;
pub const RISCV_DEBUG_REG_PERF_CNT_FPU2: u32 = 4289798176;
pub const RISCV_DEBUG_REG_PERF_CNT_L1_0: u32 = 4289798192;
pub const RISCV_DEBUG_REG_PERF_CNT_L1_1: u32 = 4289798196;
pub const RISCV_DEBUG_REG_PERF_CNT_L1_2: u32 = 4289798200;
pub const RISCV_DEBUG_REG_PERF_CNT_ALL: u32 = 4289798204;
pub const RISCV_DEBUG_REG_DBG_L1_MEM_REG0: u32 = 4289798216;
pub const RISCV_DEBUG_REG_DBG_L1_MEM_REG1: u32 = 4289798220;
pub const RISCV_DEBUG_REG_DBG_L1_MEM_REG2: u32 = 4289798224;
pub const RISCV_DEBUG_REG_DBG_BUS_CNTL_REG: u32 = 4289798228;
pub const RISCV_DEBUG_REG_CFGREG_RD_CNTL: u32 = 4289798232;
pub const RISCV_DEBUG_REG_DBG_RD_DATA: u32 = 4289798236;
pub const RISCV_DEBUG_REG_DBG_ARRAY_RD_EN: u32 = 4289798240;
pub const RISCV_DEBUG_REG_DBG_ARRAY_RD_CMD: u32 = 4289798244;
pub const RISCV_DEBUG_REG_DBG_ARRAY_RD_DATA: u32 = 4289798252;
pub const RISCV_DEBUG_REG_DBG_FEATURE_DISABLE: u32 = 4289798248;
pub const RISCV_DEBUG_REG_CG_CTRL_HYST0: u32 = 4289798256;
pub const RISCV_DEBUG_REG_CG_CTRL_HYST1: u32 = 4289798260;
pub const RISCV_DEBUG_REG_CG_CTRL_HYST2: u32 = 4289798268;
pub const RISCV_DEBUG_REG_CFGREG_RDDATA: u32 = 4289798264;
pub const RISCV_DEBUG_REG_TRISC_PC_BUF_OVERRIDE: u32 = 4289798288;
pub const RISCV_DEBUG_REG_INSTRN_BUF_CTRL0: u32 = 4289798304;
pub const RISCV_DEBUG_REG_INSTRN_BUF_CTRL1: u32 = 4289798308;
pub const RISCV_DEBUG_REG_INSTRN_BUF_STATUS: u32 = 4289798312;
pub const RISCV_DEBUG_REG_PERF_CNT_TDMA_PACK0: u32 = 4289798384;
pub const RISCV_DEBUG_REG_PERF_CNT_TDMA_PACK1: u32 = 4289798388;
pub const RISCV_DEBUG_REG_PERF_CNT_TDMA_PACK2: u32 = 4289798392;
pub const RISCV_DEBUG_REG_WALL_CLOCK_L: u32 = 4289798640;
pub const RISCV_DEBUG_REG_WALL_CLOCK_H: u32 = 4289798648;
pub const RISCV_DEBUG_REG_TIMESTAMP: u32 = 4289798652;
pub const RISCV_DEBUG_REG_TIMESTAMP_STATUS: u32 = 4289798660;
pub const SOFT_RESET_MOVER: u32 = 64;
pub const SOFT_RESET_SEARCH: u32 = 128;
pub const SOFT_RESET_GLUE: u32 = 256;
pub const SOFT_RESET_THCON: u32 = 512;
pub const SOFT_RESET_FPU: u32 = 1024;
pub const SOFT_RESET_SRCA_REG: u32 = 32768;
pub const SOFT_RESET_SRCB_REG: u32 = 65536;
pub const SOFT_RESET_DEST_REG: u32 = 131072;
pub const INSTRN_RSTDMA: u32 = 1140850688;
pub const TENSIX_UNHALT_VAL: u32 = 1073741824;
pub const INSTRN_SEL_L0: u32 = 0;
pub const INSTRN_SEL_L1: u32 = 1;
pub const INSTRN_SEL_SIZE_16B: u32 = 0;
pub const INSTRN_SEL_SIZE_4B: u32 = 1;
pub const INSTRN_SEL_SIZE_2B: u32 = 2;
pub const INSTRN_SEL_SIZE_1B: u32 = 3;
pub const INSTRN_SEL_AUTO_INC_NONE: u32 = 0;
pub const INSTRN_SEL_AUTO_INC_2B: u32 = 1;
pub const INSTRN_SEL_AUTO_INC_4B: u32 = 2;
pub const INSTRN_SEL_AUTO_INC_16B: u32 = 3;
pub const INSTRN_SEL_RD_PTR: u32 = 0;
pub const INSTRN_SEL_WR_PTR: u32 = 1;
pub const REG2FLOP_TARGET_TDMA: u32 = 0;
pub const REG2FLOP_TARGET_LOCAL_REGS: u32 = 1;
pub const REG2FLOP_TARGET_ADDR_CNTRS: u32 = 2;
pub const BYTE_OFFSET_ZERO: u32 = 0;
pub const BYTE_OFFSET_ONE: u32 = 1;
pub const BYTE_OFFSET_TWO: u32 = 2;
pub const BYTE_OFFSET_THREE: u32 = 3;
pub const ADDR_16K: u32 = 16384;
pub const ADDR_32K: u32 = 32768;
pub const ADDR_64K: u32 = 65536;
pub const ADDR_128K: u32 = 131072;
pub const ADDR_256K: u32 = 262144;
pub const ADDR_512K: u32 = 524288;
pub const ONE_16B: u32 = 16;
pub const ONE_32b: u32 = 4;
pub const R0: u32 = 0;
pub const R1: u32 = 1;
pub const R2: u32 = 2;
pub const R3: u32 = 3;
pub const R4: u32 = 4;
pub const R5: u32 = 5;
pub const R6: u32 = 6;
pub const R7: u32 = 7;
pub const R8: u32 = 8;
pub const R9: u32 = 9;
pub const R10: u32 = 10;
pub const R11: u32 = 11;
pub const R12: u32 = 12;
pub const R13: u32 = 13;
pub const R14: u32 = 14;
pub const R15: u32 = 15;
pub const R16: u32 = 16;
pub const R17: u32 = 17;
pub const R18: u32 = 18;
pub const R19: u32 = 19;
pub const R20: u32 = 20;
pub const R21: u32 = 21;
pub const R22: u32 = 22;
pub const R23: u32 = 23;
pub const R24: u32 = 24;
pub const R25: u32 = 25;
pub const R26: u32 = 26;
pub const R27: u32 = 27;
pub const R28: u32 = 28;
pub const R29: u32 = 29;
pub const R30: u32 = 30;
pub const R31: u32 = 31;
pub const R32: u32 = 32;
pub const R33: u32 = 33;
pub const R34: u32 = 34;
pub const R35: u32 = 35;
pub const R36: u32 = 36;
pub const R37: u32 = 37;
pub const R38: u32 = 38;
pub const R39: u32 = 39;
pub const R40: u32 = 40;
pub const R41: u32 = 41;
pub const R42: u32 = 42;
pub const R43: u32 = 43;
pub const R44: u32 = 44;
pub const R45: u32 = 45;
pub const R46: u32 = 46;
pub const R47: u32 = 47;
pub const R48: u32 = 48;
pub const R49: u32 = 49;
pub const R50: u32 = 50;
pub const R51: u32 = 51;
pub const R52: u32 = 52;
pub const R53: u32 = 53;
pub const R54: u32 = 54;
pub const R55: u32 = 55;
pub const R56: u32 = 56;
pub const R57: u32 = 57;
pub const R58: u32 = 58;
pub const R59: u32 = 59;
pub const R60: u32 = 60;
pub const R61: u32 = 61;
pub const R62: u32 = 62;
pub const R63: u32 = 63;
pub const R0_LO: u32 = 0;
pub const R0_HI: u32 = 1;
pub const R1_LO: u32 = 2;
pub const R1_HI: u32 = 3;
pub const R2_LO: u32 = 4;
pub const R2_HI: u32 = 5;
pub const R3_LO: u32 = 6;
pub const R3_HI: u32 = 7;
pub const R4_LO: u32 = 8;
pub const R4_HI: u32 = 9;
pub const R5_LO: u32 = 10;
pub const R5_HI: u32 = 11;
pub const R6_LO: u32 = 12;
pub const R6_HI: u32 = 13;
pub const R7_LO: u32 = 14;
pub const R7_HI: u32 = 15;
pub const R8_LO: u32 = 16;
pub const R8_HI: u32 = 17;
pub const R9_LO: u32 = 18;
pub const R9_HI: u32 = 19;
pub const R10_LO: u32 = 20;
pub const R10_HI: u32 = 21;
pub const R11_LO: u32 = 22;
pub const R11_HI: u32 = 23;
pub const R12_LO: u32 = 24;
pub const R12_HI: u32 = 25;
pub const R13_LO: u32 = 26;
pub const R13_HI: u32 = 27;
pub const R14_LO: u32 = 28;
pub const R14_HI: u32 = 29;
pub const R15_LO: u32 = 30;
pub const R15_HI: u32 = 31;
pub const R16_LO: u32 = 32;
pub const R16_HI: u32 = 33;
pub const R17_LO: u32 = 34;
pub const R17_HI: u32 = 35;
pub const R18_LO: u32 = 36;
pub const R18_HI: u32 = 37;
pub const R19_LO: u32 = 38;
pub const R19_HI: u32 = 39;
pub const R20_LO: u32 = 40;
pub const R20_HI: u32 = 41;
pub const R21_LO: u32 = 42;
pub const R21_HI: u32 = 43;
pub const R22_LO: u32 = 44;
pub const R22_HI: u32 = 45;
pub const R23_LO: u32 = 46;
pub const R23_HI: u32 = 47;
pub const R24_LO: u32 = 48;
pub const R24_HI: u32 = 49;
pub const R25_LO: u32 = 50;
pub const R25_HI: u32 = 51;
pub const R26_LO: u32 = 52;
pub const R26_HI: u32 = 53;
pub const R27_LO: u32 = 54;
pub const R27_HI: u32 = 55;
pub const R28_LO: u32 = 56;
pub const R28_HI: u32 = 57;
pub const R29_LO: u32 = 58;
pub const R29_HI: u32 = 59;
pub const R30_LO: u32 = 60;
pub const R30_HI: u32 = 61;
pub const R31_LO: u32 = 62;
pub const R31_HI: u32 = 63;
pub const R32_LO: u32 = 64;
pub const R32_HI: u32 = 65;
pub const R33_LO: u32 = 66;
pub const R33_HI: u32 = 67;
pub const R34_LO: u32 = 68;
pub const R34_HI: u32 = 69;
pub const R35_LO: u32 = 70;
pub const R35_HI: u32 = 71;
pub const R36_LO: u32 = 72;
pub const R36_HI: u32 = 73;
pub const R37_LO: u32 = 74;
pub const R37_HI: u32 = 75;
pub const R38_LO: u32 = 76;
pub const R38_HI: u32 = 77;
pub const R39_LO: u32 = 78;
pub const R39_HI: u32 = 79;
pub const R40_LO: u32 = 80;
pub const R40_HI: u32 = 81;
pub const R41_LO: u32 = 82;
pub const R41_HI: u32 = 83;
pub const R42_LO: u32 = 84;
pub const R42_HI: u32 = 85;
pub const R43_LO: u32 = 86;
pub const R43_HI: u32 = 87;
pub const R44_LO: u32 = 88;
pub const R44_HI: u32 = 89;
pub const R45_LO: u32 = 90;
pub const R45_HI: u32 = 91;
pub const R46_LO: u32 = 92;
pub const R46_HI: u32 = 93;
pub const R47_LO: u32 = 94;
pub const R47_HI: u32 = 95;
pub const R48_LO: u32 = 96;
pub const R48_HI: u32 = 97;
pub const R49_LO: u32 = 98;
pub const R49_HI: u32 = 99;
pub const R50_LO: u32 = 100;
pub const R50_HI: u32 = 101;
pub const R51_LO: u32 = 102;
pub const R51_HI: u32 = 103;
pub const R52_LO: u32 = 104;
pub const R52_HI: u32 = 105;
pub const R53_LO: u32 = 106;
pub const R53_HI: u32 = 107;
pub const R54_LO: u32 = 108;
pub const R54_HI: u32 = 109;
pub const R55_LO: u32 = 110;
pub const R55_HI: u32 = 111;
pub const R56_LO: u32 = 112;
pub const R56_HI: u32 = 113;
pub const R57_LO: u32 = 114;
pub const R57_HI: u32 = 115;
pub const R58_LO: u32 = 116;
pub const R58_HI: u32 = 117;
pub const R59_LO: u32 = 118;
pub const R59_HI: u32 = 119;
pub const R60_LO: u32 = 120;
pub const R60_HI: u32 = 121;
pub const R61_LO: u32 = 122;
pub const R61_HI: u32 = 123;
pub const R62_LO: u32 = 124;
pub const R62_HI: u32 = 125;
pub const R63_LO: u32 = 126;
pub const R63_HI: u32 = 127;
pub const TENSIX_MAX_KERNEL_LOOP_COUNT: u32 = 65535;
pub type __u_char = ::core::ffi::c_uchar;
pub type __u_short = ::core::ffi::c_ushort;
pub type __u_int = ::core::ffi::c_uint;
pub type __u_long = ::core::ffi::c_ulong;
pub type __int8_t = ::core::ffi::c_schar;
pub type __uint8_t = ::core::ffi::c_uchar;
pub type __int16_t = ::core::ffi::c_short;
pub type __uint16_t = ::core::ffi::c_ushort;
pub type __int32_t = ::core::ffi::c_int;
pub type __uint32_t = ::core::ffi::c_uint;
pub type __int64_t = ::core::ffi::c_long;
pub type __uint64_t = ::core::ffi::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::core::ffi::c_long;
pub type __u_quad_t = ::core::ffi::c_ulong;
pub type __intmax_t = ::core::ffi::c_long;
pub type __uintmax_t = ::core::ffi::c_ulong;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::core::ffi::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    const UNINIT: ::core::mem::MaybeUninit<__fsid_t> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::core::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim_t = ::core::ffi::c_ulong;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __daddr_t = ::core::ffi::c_int;
pub type __key_t = ::core::ffi::c_int;
pub type __clockid_t = ::core::ffi::c_int;
pub type __timer_t = *mut ::core::ffi::c_void;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __blkcnt64_t = ::core::ffi::c_long;
pub type __fsblkcnt_t = ::core::ffi::c_ulong;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __fsword_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __syscall_ulong_t = ::core::ffi::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::core::ffi::c_char;
pub type __intptr_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type __sig_atomic_t = ::core::ffi::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::core::ffi::c_schar;
pub type int_fast16_t = ::core::ffi::c_long;
pub type int_fast32_t = ::core::ffi::c_long;
pub type int_fast64_t = ::core::ffi::c_long;
pub type uint_fast8_t = ::core::ffi::c_uchar;
pub type uint_fast16_t = ::core::ffi::c_ulong;
pub type uint_fast32_t = ::core::ffi::c_ulong;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub const L1_MATH_KERNEL_BASE: u32 = 122880;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct riscv_debug_reg_dbg_dbus_cntl_t {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_riscv_debug_reg_dbg_dbus_cntl_t() {
    assert_eq!(
        ::core::mem::size_of::<riscv_debug_reg_dbg_dbus_cntl_t>(),
        4usize,
        concat!("Size of: ", stringify!(riscv_debug_reg_dbg_dbus_cntl_t))
    );
    assert_eq!(
        ::core::mem::align_of::<riscv_debug_reg_dbg_dbus_cntl_t>(),
        4usize,
        concat!("Alignment of ", stringify!(riscv_debug_reg_dbg_dbus_cntl_t))
    );
}
impl riscv_debug_reg_dbg_dbus_cntl_t {
    #[inline]
    pub fn dbg_sig_sel(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_sig_sel(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn dbg_daisy_sel(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_daisy_sel(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn dbg_rd_sel(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(24usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_rd_sel(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(24usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn dbg_reg_ovrd_en(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_reg_ovrd_en(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn dbg_daisy_en(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_daisy_en(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn dbg_reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(30usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_dbg_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(30usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        dbg_sig_sel: u32,
        dbg_daisy_sel: u32,
        dbg_rd_sel: u32,
        dbg_reg_ovrd_en: u32,
        dbg_daisy_en: u32,
        dbg_reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let dbg_sig_sel: u32 = unsafe { ::core::mem::transmute(dbg_sig_sel) };
            dbg_sig_sel as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let dbg_daisy_sel: u32 = unsafe { ::core::mem::transmute(dbg_daisy_sel) };
            dbg_daisy_sel as u64
        });
        __bindgen_bitfield_unit.set(24usize, 4u8, {
            let dbg_rd_sel: u32 = unsafe { ::core::mem::transmute(dbg_rd_sel) };
            dbg_rd_sel as u64
        });
        __bindgen_bitfield_unit.set(28usize, 1u8, {
            let dbg_reg_ovrd_en: u32 = unsafe { ::core::mem::transmute(dbg_reg_ovrd_en) };
            dbg_reg_ovrd_en as u64
        });
        __bindgen_bitfield_unit.set(29usize, 1u8, {
            let dbg_daisy_en: u32 = unsafe { ::core::mem::transmute(dbg_daisy_en) };
            dbg_daisy_en as u64
        });
        __bindgen_bitfield_unit.set(30usize, 2u8, {
            let dbg_reserved: u32 = unsafe { ::core::mem::transmute(dbg_reserved) };
            dbg_reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union riscv_debug_reg_dbg_dbus_cntl_u {
    pub val: u32,
    pub f: riscv_debug_reg_dbg_dbus_cntl_t,
}
#[test]
fn bindgen_test_layout_riscv_debug_reg_dbg_dbus_cntl_u() {
    const UNINIT: ::core::mem::MaybeUninit<riscv_debug_reg_dbg_dbus_cntl_u> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<riscv_debug_reg_dbg_dbus_cntl_u>(),
        4usize,
        concat!("Size of: ", stringify!(riscv_debug_reg_dbg_dbus_cntl_u))
    );
    assert_eq!(
        ::core::mem::align_of::<riscv_debug_reg_dbg_dbus_cntl_u>(),
        4usize,
        concat!("Alignment of ", stringify!(riscv_debug_reg_dbg_dbus_cntl_u))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(riscv_debug_reg_dbg_dbus_cntl_u),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).f) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(riscv_debug_reg_dbg_dbus_cntl_u),
            "::",
            stringify!(f)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct riscv_debug_reg_dbg_l1_mem_reg2_t {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_riscv_debug_reg_dbg_l1_mem_reg2_t() {
    assert_eq!(
        ::core::mem::size_of::<riscv_debug_reg_dbg_l1_mem_reg2_t>(),
        4usize,
        concat!("Size of: ", stringify!(riscv_debug_reg_dbg_l1_mem_reg2_t))
    );
    assert_eq!(
        ::core::mem::align_of::<riscv_debug_reg_dbg_l1_mem_reg2_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(riscv_debug_reg_dbg_l1_mem_reg2_t)
        )
    );
}
impl riscv_debug_reg_dbg_l1_mem_reg2_t {
    #[inline]
    pub fn mem_dump_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_mem_dump_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn skip_cycles(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_skip_cycles(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn mem_write(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_mem_write(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn mem_read(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_mem_read(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 18u8) as u32) }
    }
    #[inline]
    pub fn set_reserved(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 18u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_dump_mode: u32,
        skip_cycles: u32,
        mem_write: u32,
        mem_read: u32,
        reserved: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let mem_dump_mode: u32 = unsafe { ::core::mem::transmute(mem_dump_mode) };
            mem_dump_mode as u64
        });
        __bindgen_bitfield_unit.set(4usize, 8u8, {
            let skip_cycles: u32 = unsafe { ::core::mem::transmute(skip_cycles) };
            skip_cycles as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let mem_write: u32 = unsafe { ::core::mem::transmute(mem_write) };
            mem_write as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let mem_read: u32 = unsafe { ::core::mem::transmute(mem_read) };
            mem_read as u64
        });
        __bindgen_bitfield_unit.set(14usize, 18u8, {
            let reserved: u32 = unsafe { ::core::mem::transmute(reserved) };
            reserved as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union riscv_debug_reg_dbg_l1_mem_reg2_u {
    pub val: u32,
    pub f: riscv_debug_reg_dbg_l1_mem_reg2_t,
}
#[test]
fn bindgen_test_layout_riscv_debug_reg_dbg_l1_mem_reg2_u() {
    const UNINIT: ::core::mem::MaybeUninit<riscv_debug_reg_dbg_l1_mem_reg2_u> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<riscv_debug_reg_dbg_l1_mem_reg2_u>(),
        4usize,
        concat!("Size of: ", stringify!(riscv_debug_reg_dbg_l1_mem_reg2_u))
    );
    assert_eq!(
        ::core::mem::align_of::<riscv_debug_reg_dbg_l1_mem_reg2_u>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(riscv_debug_reg_dbg_l1_mem_reg2_u)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(riscv_debug_reg_dbg_l1_mem_reg2_u),
            "::",
            stringify!(val)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).f) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(riscv_debug_reg_dbg_l1_mem_reg2_u),
            "::",
            stringify!(f)
        )
    );
}
pub const cnt_id_t_UNP0: cnt_id_t = 1;
pub const cnt_id_t_UNP1: cnt_id_t = 2;
pub const cnt_id_t_PCK0: cnt_id_t = 4;
pub type cnt_id_t = ::core::ffi::c_uint;
