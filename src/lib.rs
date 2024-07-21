#![no_std]

pub mod reset;

#[cfg(target_env = "grayskull")]
pub use gs_assembly as target;

#[cfg(target_env = "wormhole")]
pub use wh_assembly as target;

#[cfg(target_env = "blackhole")]
pub use bh_assembly as target;

pub use runtime::{
    entry, get_postcode_brisc, get_postcode_ncrisc, get_postcode_trisc0, get_postcode_trisc1,
    get_postcode_trisc2, get_stack_size_brisc, set_postcode_brisc, set_postcode_ncrisc,
    set_postcode_trisc0, set_postcode_trisc1, set_postcode_trisc2, set_stack_size_brisc,
    set_stack_size_ncrisc, update_program_counter_brisc, update_stack_size_brisc, CoreState,
    SCRATCH_BASE, SCRATCH_TOP,
};
