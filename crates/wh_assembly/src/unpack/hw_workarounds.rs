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
        DataFormat::Lf8 | DataFormat::Int8 => numel,

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

// For legacy hardware reasons, if using BFP formats you actually might
// need to set a "wrong" output format for the unpacker. Also, because of
// some vestigial tail in the RTL, you need to add a magic offset to your
// base src register.
pub fn weird_unpacr_workarounds(
    l1_fmt: DataFormat,
    out_fmt: DataFormat,
) -> (crate::DataFormat, u32) {
    //BFP format conversion table for unpacker. Any rows not shown in this
    //table may not be legal format conversions in the first place.
    // | L1 fmt | Src reg fmt | What you must program into the "out_fmt" register |
    // ---------------------------------------------------------------------------|
    // | bfp8_b | fp16_b      | bfp8_b                                            |
    // | bfp8_b | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|
    // | bfp8_a | fp16_a      | bfp8_a                                            |
    // | bfp8_a | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|
    // | bfp4_b | fp16_b      | bfp4_b                                            |
    // | bfp4_b | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|
    // | bfp4_a | fp16_a      | bfp4_a                                            |
    // | bfp4_a | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|
    // | bfp2_b | fp16_b      | bfp2_b                                            |
    // | bfp2_b | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|
    // | bfp2_a | fp16_a      | bfp2_a                                            |
    // | bfp2_a | tf32        | tf32                                              |
    // ---------------------------------------------------------------------------|

    match l1_fmt {
        DataFormat::Float32
        | DataFormat::Tf32
        | DataFormat::Float16
        | DataFormat::Float16B
        | DataFormat::Int32
        | DataFormat::Int16
        | DataFormat::Int8 => {
            (
                out_fmt,
                if out_fmt == DataFormat::Tf32 {
                    tile_size(l1_fmt, 4 * 16)
                } else {
                    tile_size(out_fmt, 4 * 16) //No idea whhy this is the case...
                },
            )
        }

        DataFormat::Lf8 => (
            if out_fmt == DataFormat::Tf32 {
                out_fmt
            } else {
                DataFormat::Lf8
            },
            4 * 16 * 1,
        ),
        DataFormat::Bfp8
        | DataFormat::Bfp8B
        | DataFormat::Bfp4
        | DataFormat::Bfp4B
        | DataFormat::Bfp2
        | DataFormat::Bfp2B => {
            (
                if out_fmt == DataFormat::Tf32 {
                    out_fmt
                } else {
                    l1_fmt
                },
                4 * 16 * 1, //Don't look at me, I have no idea why you don't multiply this by the size of out_fmt datums...
            )
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
        | DataFormat::Bfp2B => 1,
    }
}
