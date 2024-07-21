#![no_std]

mod asm;
pub use runtime_macros::entry;

use core::{fmt::Write, panic::PanicInfo};

/// We export this static with an informative name so that if an application attempts to link
/// two copies of riscv-rt together, linking will fail. We also declare a links key in
/// Cargo.toml which is the more modern way to solve the same problem, but we have to keep
/// __ONCE__ around to prevent linking with versions before the links key was added.
#[export_name = "error: riscv-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

#[repr(u32, align(16))]
#[derive(Clone, Copy)]
pub enum CoreState {
    NotStarted = 0,
    Init = 1,
    Main = 2,
    NoMain = 3,
    Completed = 4,
    NoExit = 5,
    Panic = 6,
    NoRun = 7,
}

#[repr(C, align(16))]
pub struct PanicData {
    pub filename_addr: u32,
    pub filename_len: u32,
    pub line: u32,

    pub message_addr: u32,
    pub message_len: u32,

    pub stack_pointer: u32,
    pub program_counter: u32,

    pub panicked: bool,
}

impl PanicData {
    pub const fn new() -> Self {
        PanicData {
            panicked: false,
            line: 0,
            filename_addr: 0,
            filename_len: 0,
            message_addr: 0,
            message_len: 0,
            stack_pointer: 0,
            program_counter: 0,
        }
    }
}

fn get_program_counter() -> u32 {
    let pc: u32;
    unsafe {
        core::arch::asm!("auipc t0, 0\nmv {}, t0", out(reg) pc);
    }
    pc
}

fn get_stack_pointer() -> u32 {
    let sp: u32;
    unsafe {
        core::arch::asm!("mv {}, sp", out(reg) sp);
    }
    sp
}

fn get_stack_size() -> u32 {
    extern "Rust" {
        static mut ___brisc_stack_bottom: u8;
    }

    get_stack_pointer() - core::ptr::addr_of!(___brisc_stack_bottom) as u32
}

macro_rules! per_core_static {
    ($name:ident, $big:ident) => {
        paste::paste! {
            #[no_mangle]
            pub static mut [<STATE_ $big>]: CoreState = CoreState::NotStarted;
            #[no_mangle]
            pub static mut [<POSTCODE_ $big>]: NocAlignWrapper = NocAlignWrapper(0);
            #[no_mangle]
            pub static mut [<PC_ $big>]: NocAlignWrapper = NocAlignWrapper(0);
            #[no_mangle]
            pub static mut [<STACK_SIZE_ $big>]: NocAlignWrapper = NocAlignWrapper(0);
            #[no_mangle]
            pub static mut [<PANIC_DATA_ $big>]: PanicData = PanicData::new();

            #[no_mangle]
            pub unsafe fn [<complete_ $name>]() {
                [<STATE_ $big>] = CoreState::Completed;
            }

            #[no_mangle]
            pub unsafe fn [<no_exit_ $name>]() {
                [<STATE_ $big>] = CoreState::NoExit;
            }

            #[no_mangle]
            pub unsafe fn [<update_program_counter_ $name>]() {
                let pc = get_program_counter();
                (&raw mut[<PC_ $big>].0).write_volatile(pc);
            }

            #[no_mangle]
            pub unsafe fn [<update_stack_size_ $name>]() -> u32 {
                let stack_size = get_stack_size();
                (&raw mut[<STACK_SIZE_ $big>].0).write_volatile(stack_size);
                stack_size
            }

            #[no_mangle]
            pub unsafe fn [<set_stack_size_ $name>](pc: u32) {
                (&raw mut[<STACK_SIZE_ $big>].0).write_volatile(pc);
            }

            #[no_mangle]
            pub unsafe fn [<get_stack_size_ $name>]() -> u32 {
                (&raw mut[<STACK_SIZE_ $big>].0).read_volatile()
            }

            #[no_mangle]
            pub unsafe fn [<set_postcode_ $name>](pc: u32) {
                (&raw mut[<POSTCODE_ $big>].0).write_volatile(pc);
            }

            #[no_mangle]
            pub unsafe fn [<get_postcode_ $name>]() -> u32 {
                (&raw mut[<POSTCODE_ $big>].0).read_volatile()
            }
        }
    };
}

#[repr(align(16))]
pub struct NocAlignWrapper(pub u32);

#[no_mangle]
#[link_section = ".data"] // Don't put this into ebss, it is used prior to program load
pub static mut START_SYNC: NocAlignWrapper = NocAlignWrapper(0);

per_core_static!(brisc, BRISC);
per_core_static!(ncrisc, NCRISC);
per_core_static!(trisc0, TRISC0);
per_core_static!(trisc1, TRISC1);
per_core_static!(trisc2, TRISC2);

#[no_mangle]
pub static mut PANIC_DATA_UNKNOWN: PanicData = PanicData::new();

pub static mut SCRATCH_BASE: u32 = 0;
pub static mut SCRATCH_TOP: u32 = 0;

#[no_mangle]
pub unsafe extern "C" fn __rust_start(core_id: u32) -> ! {
    extern "C" {
        static mut _bss: u8;
        static mut _ebss: u8;

        static mut __firmware_end: u8;
        static mut __L1_END: u8;
    }

    SCRATCH_BASE = core::ptr::addr_of_mut!(__firmware_end) as u32;
    SCRATCH_TOP = core::ptr::addr_of_mut!(__L1_END) as u32;

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn brisc_main() -> !;

        fn ncrisc_main() -> !;

        fn trisc0_main() -> !;
        fn trisc1_main() -> !;
        fn trisc2_main() -> !;
    }

    match core_id {
        0 => {
            STATE_BRISC = CoreState::Main;
            brisc_main();
        }
        1 => {
            STATE_NCRISC = CoreState::Main;
            ncrisc_main();
        }
        2 => {
            STATE_TRISC0 = CoreState::Main;
            trisc0_main();
        }
        3 => {
            STATE_TRISC1 = CoreState::Main;
            trisc1_main();
        }
        4 => {
            STATE_TRISC2 = CoreState::Main;
            trisc2_main();
        }
        core => {
            unreachable!("Recived unknown core_id {core:x}")
        }
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    extern "Rust" {
        // All of our stacks have the same bottom
        static mut ___brisc_stack_bottom: u8;
    }

    // Read core id
    let core_id = unsafe {
        core::ptr::addr_of_mut!(___brisc_stack_bottom)
            .cast::<u32>()
            .read_volatile()
    };

    let data = PanicData {
        panicked: true,

        program_counter: 0,
        stack_pointer: 0,

        filename_addr: 0,
        filename_len: 0,
        line: 0,

        message_addr: 0,
        message_len: 0,
    };

    unsafe {
        if (core_id >> 8) != 0xafd9d1 {
            // our core id is corrupted, use the unknown panic info struct to hold this data
            core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN)
        } else {
            match core_id & 0xFF {
                0 => core::ptr::addr_of_mut!(PANIC_DATA_BRISC),
                1 => core::ptr::addr_of_mut!(PANIC_DATA_NCRISC),
                2 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC0),
                3 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC1),
                4 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC2),
                _ => core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN),
            }
        }
        .write_volatile(data);
    }

    let file = panic.location().map(|v| v.file()).unwrap_or("");

    let data_base = unsafe { ((SCRATCH_BASE + 31) & !31) as *mut u8 };

    unsafe {
        let mut count = 0;
        for c in file.as_bytes() {
            data_base
                .cast::<u8>()
                .offset(count as isize)
                .write_volatile(*c);
            count += 1;
        }
    }

    struct RawWriter {
        len: usize,
        offset: *mut u8,
    }

    impl Write for RawWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.as_bytes() {
                unsafe {
                    self.offset.offset(self.len as isize).write_volatile(*c);
                }
                self.len += 1;
            }

            Ok(())
        }
    }

    let data = PanicData {
        panicked: true,

        program_counter: get_program_counter() as u32,
        stack_pointer: get_stack_pointer() as u32,

        filename_addr: data_base as u32,
        filename_len: file.len() as u32,
        line: panic.location().map(|v| v.line()).unwrap_or(0xDEAD),

        message_addr: (data_base as usize + file.len()) as u32,
        message_len: 0,
    };

    unsafe {
        if (core_id >> 8) != 0xafd9d1 {
            // our core id is corrupted, use the unknown panic info struct to hold this data
            core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN)
        } else {
            match core_id & 0xFF {
                0 => core::ptr::addr_of_mut!(PANIC_DATA_BRISC),
                1 => core::ptr::addr_of_mut!(PANIC_DATA_NCRISC),
                2 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC0),
                3 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC1),
                4 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC2),
                _ => core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN),
            }
        }
        .write_volatile(data);
    }

    let message = panic.message();
    let mut writer = RawWriter {
        len: 0,
        offset: unsafe { data_base.byte_add(file.len()).cast() },
    };
    // We can't actually fail here...
    let _ = write!(writer, "{}", message);

    let data = PanicData {
        panicked: true,

        program_counter: get_program_counter() as u32,
        stack_pointer: get_stack_pointer() as u32,

        filename_addr: data_base as u32,
        filename_len: file.len() as u32,
        line: panic.location().map(|v| v.line()).unwrap_or(0xDEAD),

        message_addr: (data_base as usize + file.len()) as u32,
        message_len: writer.len as u32,
    };

    unsafe {
        if (core_id >> 8) != 0xafd9d1 {
            // our core id is corrupted, use the unknown panic info struct to hold this data
            core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN)
        } else {
            match core_id & 0xFF {
                0 => core::ptr::addr_of_mut!(PANIC_DATA_BRISC),
                1 => core::ptr::addr_of_mut!(PANIC_DATA_NCRISC),
                2 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC0),
                3 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC1),
                4 => core::ptr::addr_of_mut!(PANIC_DATA_TRISC2),
                _ => core::ptr::addr_of_mut!(PANIC_DATA_UNKNOWN),
            }
        }
        .write_volatile(data);

        if let Some(state) = match core_id & 0xFF {
            0 => Some(core::ptr::addr_of_mut!(STATE_BRISC)),
            1 => Some(core::ptr::addr_of_mut!(STATE_NCRISC)),
            2 => Some(core::ptr::addr_of_mut!(STATE_TRISC0)),
            3 => Some(core::ptr::addr_of_mut!(STATE_TRISC1)),
            4 => Some(core::ptr::addr_of_mut!(STATE_TRISC2)),
            _ => None,
        } {
            state.write_volatile(CoreState::Panic);
        }
    }

    loop {}
}
