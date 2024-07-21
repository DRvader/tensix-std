core::arch::global_asm! {
".section .init",
".global __program_start",
"__program_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    // The program will start here, so
    // initialize global state to 0
    "li a0, 0",
    "la t0, STATE_BRISC",
    "sw a0, 0(t0)",
    "la t0, POSTCODE_BRISC",
    "sw a0, 0(t0)",

    "la t0, STATE_NCRISC",
    "sw a0, 0(t0)",
    "la t0, POSTCODE_NCRISC",
    "sw a0, 0(t0)",

    "la t0, STATE_TRISC0",
    "sw a0, 0(t0)",
    "la t0, POSTCODE_TRISC0",
    "sw a0, 0(t0)",

    "la t0, STATE_TRISC1",
    "sw a0, 0(t0)",
    "la t0, POSTCODE_TRISC1",
    "sw a0, 0(t0)",

    "la t0, STATE_TRISC2",
    "sw a0, 0(t0)",
    "la t0, POSTCODE_TRISC2",
    "sw a0, 0(t0)"
}

#[cfg(feature = "sync-start")]
core::arch::global_asm! {
    // Wait for sync input
    "la t0, START_SYNC",
    // Set it to 1 to indicate that we need to
    // start the sync
    "li t1, 1",
    "sw t1, 0(t0)",
    // Wait for the host to write a 2
    "li t1, 1",
"sync_loop_start:",
    "lw a0, 0(t0)",
    "beq a0, t1, sync_loop_start",
    // Set the sync to 3 to indicate that we are done
    "li a0, 3",
    "sw a0, 0(t0)",
    // Fall through to brisc_start
}

core::arch::global_asm! {
    ".cfi_endproc",

".global __brisc_start",
"__brisc_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    "li a0, 0",
    "la t0, STATE_BRISC",
    "li t1, 1",
    "sw t1, 0(t0)",
    "j __start",

    ".cfi_endproc",

".global __ncrisc_start",
"__ncrisc_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    "li a0, 1",
    "la t0, STATE_NCRISC",
    "li t1, 1",
    "sw t1, 0(t0)",
    "j __start",

    ".cfi_endproc",

".global __trisc0_start",
"__trisc0_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    "li a0, 2",
    "la t0, STATE_TRISC0",
    "li t1, 1",
    "sw t1, 0(t0)",
    "j __start",

    ".cfi_endproc",

".global __trisc1_start",
"__trisc1_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    "li a0, 3",
    "la t0, STATE_TRISC1",
    "li t1, 1",
    "sw t1, 0(t0)",
    "j __start",

    ".cfi_endproc",

".global __trisc2_start",
"__trisc2_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    "li a0, 4",
    "la t0, STATE_TRISC2",
    "li t1, 1",
    "sw t1, 0(t0)",
    "j __start",

    ".cfi_endproc",

"__start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    // ZERO OUT GENERAL-PURPOSE REGISTERS
    "li x1, 0",
    "li x2, 0",
    "li x3, 0",
    "li x4, 0",
    "li x5, 0",
    "li x6, 0",
    "li x7, 0",
    "li x8, 0",
    "li x9, 0",
    // We don't zero x10 because it is how we are passing the core id around
    // "li x10, 0",
    "li x11, 0",
    "li x12, 0",
    "li x13, 0",
    "li x14, 0",
    "li x15, 0",
    "li x16, 0",
    "li x17, 0",
    "li x18, 0",
    "li x19, 0",
    "li x20, 0",
    "li x21, 0",
    "li x22, 0",
    "li x23, 0",
    "li x24, 0",
    "li x25, 0",
    "li x26, 0",
    "li x27, 0",
    "li x28, 0",
    "li x29, 0",
    "li x30, 0",
    "li x31, 0",

    // Init the global pointer
    // All of the options stuff ensures that we actual set gp to the value we want.
    ".option push",
    ".option norelax",
    "la gp, __global_pointer$",
    ".option pop",

    // Init the stack pointer
    "li t2, 0",
    "bne a0, t2, ncrisc_stack_select",
    "la t0, ___brisc_stack_bottom",
    "la t1, ___brisc_stack_top",
    "j stack_init_start",
"ncrisc_stack_select:",
    "li t2, 1",
    "bne a1, t2, trisc_stack_select",
    "la t0, ___ncrisc_stack_bottom",
    "la t1, ___ncrisc_stack_top",
    "j stack_init_start",
"trisc_stack_select:",
    "la t0, ___trisc_stack_bottom",
    "la t1, ___trisc_stack_top",
"stack_init_start:",

    // Align stack top to 16 bytes (down)
    "li sp, 0",
    "andi sp, t1, -16",

    // Set t1 to the aligned sp
    "addi t1, sp, 0",

    // We are going to put the current core id in the bottom of the stack.
    // This will mean that the panic knows which core was the problem.
    // We are also going to play double duty here and put a sentinal value in the
    // bottom 3 bytes of this to try and detect stack overflows.
    // Load a random value into t2
    "li t2, 0xafd9d1",
    // Shift if left in preparation for oring with our core id
    "slli t2, t2, 8",
    "or t2, t2, a0",
    "sw t2, 0(t0)",
    // Increment stack bottom by 4 bytes.
    "addi t0, t0, 4",

    // Align stack bottom to 16 bytes (up)
    "addi t0, t0, 15",
    "andi t0, t0, -16",

    // Zero out .stack
    "bgeu  t0, t1, zero_stack_end",
"zero_stack_start:",
    "sw zero, 0(t0)",
    "addi t0, t0, 4",
    "bltu t0, t1, zero_stack_start",
"zero_stack_end:",

    // Init the frame pointer to be equal to the stack pointer
    "add fp, sp, zero",

    // Only zero .bss when running brisc init
    "bne a0, x0, zero_bss_end",

    // Zero out .bss
    "la t0, _bss",
    "la t1, _ebss",
    "bgeu  t0, t1, zero_bss_end",
"zero_bss_start:",
    "sw  zero, 0(t0)",
    "addi t0, t0, 4",
    "bltu t0, t1, zero_bss_start",
"zero_bss_end:",
    // Zero used registers (probably doesn't matter)
    "li t0, 0",
    "li t1, 0",
    "li t2, 0",

    "jal zero, __rust_start",

    ".cfi_endproc",

// Default core function
/* Make it .weak so PAC/HAL can provide their own if needed. */
".weak brisc_main",
"brisc_main:",
    "la t0, STATE_BRISC",
    "li t1, 7",
    "sw t1, 0(t0)",
    "j fw_end",

// Default core function
".weak ncrisc_main",
"ncrisc_main:",
    "la t0, STATE_NCRISC",
    "li t1, 7",
    "sw t1, 0(t0)",
    "j fw_end",

// Default core function
".weak trisc0_main",
"trisc0_main:",
    "la t0, STATE_TRISC0",
    "li t1, 7",
    "sw t1, 0(t0)",
    "j fw_end",

// Default core function
".weak trisc1_main",
"trisc1_main:",
    "la t0, STATE_TRISC1",
    "li t1, 7",
    "sw t1, 0(t0)",
    "j fw_end",

// Default core function
".weak trisc2_main",
"trisc2_main:",
    "la t0, STATE_TRISC2",
    "li t1, 7",
    "sw t1, 0(t0)",

"fw_end:",
    "j fw_end"
}
