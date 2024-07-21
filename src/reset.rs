use crate::target::SoftReset;

pub fn start_cores() {
    extern "C" {
        fn __ncrisc_start() -> !;

        fn __trisc0_start() -> !;
        fn __trisc1_start() -> !;
        fn __trisc2_start() -> !;
    }

    crate::target::cfg::pc_reset::set_ncrisc_reset_pc(__ncrisc_start as u32);
    crate::target::cfg::pc_reset::set_trisc0_reset_pc(__trisc0_start as u32);
    crate::target::cfg::pc_reset::set_trisc1_reset_pc(__trisc1_start as u32);
    crate::target::cfg::pc_reset::set_trisc2_reset_pc(__trisc2_start as u32);

    crate::target::deassert_reset(SoftReset::Ncrisc);
    crate::target::deassert_reset(SoftReset::Trisc0);
    crate::target::deassert_reset(SoftReset::Trisc1);
    crate::target::deassert_reset(SoftReset::Trisc2);
}
