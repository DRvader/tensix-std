use crate::DataFormat;

/////////////
// Helpers //
/////////////
//Size in bytes of a tile with format fmt and numel elements
pub fn tile_size(fmt: crate::DataFormat, numel: u16) -> u32 {
    let numel = numel as u32;

    match fmt {
        DataFormat::Float32 | DataFormat::Tf32 | DataFormat::Int32 => 4 * numel,
        DataFormat::Float16 | DataFormat::Float16B | DataFormat::Int16 => 2 * numel,
        DataFormat::Lf8 | DataFormat::Lf8Plus | DataFormat::Int8 | DataFormat::Uint8 => numel,

        DataFormat::Bfp8 | DataFormat::Bfp8B => {
            assert!(
                numel % 16 == 0,
                "BFP requires multiple-of-16 number of datums"
            );
            1*numel + // Shared exponents
                8*numel/8 // Signs + significands
        }
        DataFormat::Bfp4 | DataFormat::Bfp4B => {
            assert!(
                numel % 16 == 0,
                "BFP requires multiple-of-16 number of datums"
            );
            1*numel + // Shared exponents
                4*numel/8 // Signs + significands
        }
        DataFormat::Bfp2 | DataFormat::Bfp2B => {
            assert!(
                numel % 16 == 0,
                "BFP requires multiple-of-16 number of datums"
            );
            1*numel + // Shared exponents
                2*numel/8 // Signs + significands
        }
    }
}

// Returns data format multiplier you need to add to your x/y/z/w
// strides
pub fn data_fmt_mult(fmt: DataFormat) -> u32 {
    match fmt {
        DataFormat::Float32 | DataFormat::Int32 | DataFormat::Tf32 => 4,

        DataFormat::Float16 | DataFormat::Float16B | DataFormat::Int16 => 2,

        DataFormat::Int8
        | DataFormat::Lf8
        | DataFormat::Bfp8
        | DataFormat::Bfp8B
        | DataFormat::Bfp4
        | DataFormat::Bfp4B
        | DataFormat::Bfp2
        | DataFormat::Bfp2B
        | DataFormat::Uint8
        | DataFormat::Lf8Plus => 1,
    }
}

#[derive(Clone)]
pub struct ConfigSettings {
    pub in_format: u8,
    pub out_format: u8,
    /// In BH, format = {Round_10b_mant, Read_int8, Read_unsigned, Read_32b_data}
    pub pack_dst_rd_ctrl: u8,
    /// If true, need to program exponent section size according to Z-dim
    pub is_bfp: bool,
    pub set_lf8plus: bool,
    pub clear_lf8plus: bool,
}

impl ConfigSettings {
    pub const INVALID: Option<Self> = None;

    pub const fn new(
        input: u8,
        output: u8,
        ctrl: u8,
        bfp: bool,
        set_lf8plus: bool,
        clear_lf8plus: bool,
    ) -> Option<Self> {
        Some(Self {
            in_format: input,
            out_format: output,
            pack_dst_rd_ctrl: ctrl,
            is_bfp: bfp,
            set_lf8plus,
            clear_lf8plus,
        })
    }
}

macro_rules! dt_to_int {
    ($name:ident, $fmt:ident) => {
        paste::paste! {
        static $name: u8 = $crate::tensix_defs::[<DataFormat_ $fmt>] as u8;
        }
    };
}

dt_to_int!(F32, Float32);
dt_to_int!(HFA, Float16);
dt_to_int!(HFB, Float16_b);
dt_to_int!(B8A, Bfp8);
dt_to_int!(B8B, Bfp8_b);
dt_to_int!(B4A, Bfp4);
dt_to_int!(B4B, Bfp4_b);
dt_to_int!(B2A, Bfp2);
dt_to_int!(B2B, Bfp2_b);
dt_to_int!(LF8, Lf8);
dt_to_int!(T32, Tf32);
dt_to_int!(I32, Int32);
dt_to_int!(I16, Int16);
dt_to_int!(I08, Int8);

fn df_to_lut(fmt: DataFormat) -> Option<usize> {
    let idx = match fmt {
        DataFormat::Float32 => 0,
        DataFormat::Float16B => 1,
        DataFormat::Float16 => 2,
        DataFormat::Int32 => 3,
        DataFormat::Uint8 => 4,
        DataFormat::Int8 => 5,
        DataFormat::Int16 => 6,
        DataFormat::Bfp8B => 7,
        DataFormat::Bfp8 => 8,
        DataFormat::Bfp4B => 9,
        DataFormat::Bfp4 => 10,
        DataFormat::Bfp2B => 11,
        DataFormat::Bfp2 => 12,
        DataFormat::Lf8 => 13,
        DataFormat::Lf8Plus => 14,
        DataFormat::Tf32 => 15,
    };

    Some(idx)
}

static SETTINGS_LOOKUP: [[Option<ConfigSettings>; 16]; 7] = [
    // dest_fmt == F32
    [
        // FP32
        ConfigSettings::new(F32, F32, 0b0001, false, false, false),
        // FP16B
        ConfigSettings::new(HFB, HFB, 0b0001, false, false, false),
        // FP16A
        ConfigSettings::new(HFA, HFA, 0b1001, false, false, false),
        // INT32
        None,
        // UINT8
        None,
        // INT8
        None,
        // INT16
        None,
        // BFP8B
        ConfigSettings::new(B8B, B8B, 0b0001, true, false, false),
        // BFP8A
        ConfigSettings::new(B8B, B8A, 0b0001, true, false, false),
        // BFP4B
        ConfigSettings::new(B8B, B4B, 0b0001, true, false, false),
        // BHP4A
        ConfigSettings::new(B8B, B4A, 0b0001, true, false, false),
        // BFP2B
        ConfigSettings::new(B8B, B2B, 0b0001, true, false, false),
        // BFP2A
        ConfigSettings::new(B8B, B2A, 0b0001, true, false, false),
        // LF8
        ConfigSettings::new(B8B, LF8, 0b0001, false, true, false),
        // LF8+
        ConfigSettings::new(HFA, LF8, 0b1001, false, true, false),
        // TF32
        ConfigSettings::new(T32, T32, 0b0001, false, false, false),
    ],
    // dest_fmt == FP16B
    [
        // FP32
        ConfigSettings::new(HFB, F32, 0b0000, false, false, false),
        // FP16B
        ConfigSettings::new(HFB, HFB, 0b0000, false, false, false),
        // FP16A
        ConfigSettings::new(HFB, HFA, 0b1000, false, false, false),
        // INT32
        None,
        // UINT8
        None,
        // INT8
        None,
        // INT16
        None,
        // BFP8B
        ConfigSettings::new(B8B, B8B, 0b0000, true, false, false),
        // BFP8A
        ConfigSettings::new(B8B, B8A, 0b0000, true, false, false),
        // BFP4B
        ConfigSettings::new(B8B, B4B, 0b0000, true, false, false),
        // BHP4A
        ConfigSettings::new(B8B, B4A, 0b0000, true, false, false),
        // BFP2B
        ConfigSettings::new(B8B, B2B, 0b0000, true, false, false),
        // BFP2A
        ConfigSettings::new(B8B, B2A, 0b0000, true, false, false),
        // LF8
        ConfigSettings::new(B8B, LF8, 0b0000, false, true, false),
        // LF8+
        ConfigSettings::new(HFA, LF8, 0b1000, false, true, false),
        // TF32
        None,
    ],
    // dest_fmt == FP16A
    [
        // FP32
        ConfigSettings::new(HFA, F32, 0b0000, false, false, false),
        // FP16B
        ConfigSettings::new(HFA, HFB, 0b0000, false, false, false),
        // FP16A
        ConfigSettings::new(HFA, HFA, 0b1000, false, false, false),
        // INT32
        None,
        // UINT8
        None,
        // INT8
        None,
        // INT16
        None,
        // BFP8B
        ConfigSettings::new(B8A, B8B, 0b0000, true, false, false),
        // BFP8A
        ConfigSettings::new(B8A, B8A, 0b0000, true, false, false),
        // BFP4B
        ConfigSettings::new(B8A, B4B, 0b0000, true, false, false),
        // BHP4A
        ConfigSettings::new(B8A, B4A, 0b0000, true, false, false),
        // BFP2B
        ConfigSettings::new(B8A, B2B, 0b0000, true, false, false),
        // BFP2A
        ConfigSettings::new(B8A, B2A, 0b0000, true, false, false),
        // LF8
        ConfigSettings::new(LF8, LF8, 0b0000, false, false, true),
        // LF8+
        ConfigSettings::new(HFA, LF8, 0b0000, false, true, false),
        // TF32
        None,
    ],
    // dest_fmt == INT32
    [
        // FP32
        None,
        // FP16B
        None,
        // FP16A
        None,
        // INT32
        ConfigSettings::new(I32, I32, 0b0001, false, false, false),
        // UINT8
        ConfigSettings::new(I08, I08, 0b0011, false, false, false),
        // INT8
        ConfigSettings::new(I08, I08, 0b0001, false, false, false),
        // INT16
        None,
        // BFP8B
        None,
        // BFP8A
        None,
        // BFP4B
        None,
        // BHP4A
        None,
        // BFP2B
        None,
        // BFP2A
        None,
        // LF8
        None,
        // LF8+
        None,
        // TF32
        None,
    ],
    // dest_fmt == uint8
    [
        // FP32
        None,
        // FP16B
        None,
        // FP16A
        None,
        // INT32
        None,
        // UINT8
        ConfigSettings::new(I08, I08, 0b0110, false, false, false),
        // INT8
        None,
        // INT16
        None,
        // BFP8B
        None,
        // BFP8A
        None,
        // BFP4B
        None,
        // BHP4A
        None,
        // BFP2B
        None,
        // BFP2A
        None,
        // LF8
        None,
        // LF8+
        None,
        // TF32
        None,
    ],
    // dest_fmt == int8
    [
        // FP32
        None,
        // FP16B
        None,
        // FP16A
        None,
        // INT32
        None,
        // UINT8
        None,
        // INT8
        ConfigSettings::new(I08, I08, 0b0100, false, false, false),
        // INT16
        None,
        // BFP8B
        None,
        // BFP8A
        None,
        // BFP4B
        None,
        // BHP4A
        None,
        // BFP2B
        None,
        // BFP2A
        None,
        // LF8
        None,
        // LF8+
        None,
        // TF32
        None,
    ],
    // dest_fmt == int16
    [
        // FP32
        None,
        // FP16B
        None,
        // FP16A
        None,
        // INT32
        None,
        // UINT8
        None,
        // INT8
        None,
        // INT16
        ConfigSettings::new(I16, I16, 0b0000, false, false, false),
        // BFP8B
        None,
        // BFP8A
        None,
        // BFP4B
        None,
        // BHP4A
        None,
        // BFP2B
        None,
        // BFP2A
        None,
        // LF8
        None,
        // LF8+
        None,
        // TF32
        None,
    ],
];

pub fn lookup_cfg_settings(
    dest_fmt: DataFormat,
    l1_fmt: DataFormat,
) -> Option<&'static ConfigSettings> {
    if let (Some(dest_idx), Some(l1_idx)) = (df_to_lut(dest_fmt), df_to_lut(l1_fmt)) {
        SETTINGS_LOOKUP[dest_idx as usize][l1_idx as usize].as_ref()
    } else {
        None
    }
}
