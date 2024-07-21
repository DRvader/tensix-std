use crate::asm;

#[inline]
pub fn elwmul<
    const CLR: crate::cfg::addr::SrcValid,
    const ADDR_MODE: crate::cfg::addr::AddrSel,
    const BCAST: bool,
>() {
    #[inline]
    pub const fn inner(
        src_clr: crate::cfg::addr::SrcValid,
        bcast: bool,
        src: crate::cfg::addr::AddrSel,
    ) -> u32 {
        asm::ELWMUL_value!(src_clr.0, bcast as u32, src as u32, 0)
    }

    unsafe {
        core::arch::asm!(
            ".word ({value})",
            value = const { inner(CLR, BCAST, ADDR_MODE) }
        );
    }
}
