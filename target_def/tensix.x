MEMORY {
/*
	L1 : ORIGIN = 0x0, LENGTH = 1536K
*/
	L1 : ORIGIN = 0x0, LENGTH = 1024K
	LDM : ORIGIN = 0xFFB00000, LENGTH = 2K
}

__firmware_stack_size = 2K;

ENTRY(__brisc_start)

SECTIONS {
	.text : ALIGN(4) {
		__firmware_start = .;

		FILL(0xff)

		*(.init*)

		*(.text*)

		KEEP(*(.eh_frame))
		*(.eh_frame_hdr)
	} > L1

	.rodata : ALIGN(4) {
		_rodata = .;

		PROVIDE(__global_pointer$ = . + 0x800);

		*(.srodata .srodata.*);
		*(.rodata .rodata.*);

		. = ALIGN(4);

		_erodata = .;
	} > L1

	.data : ALIGN(4) {
		_data = .;

		*(.sdata .sdata.*);
		*(.data .data.*);

		. = ALIGN(4);

		_edata = .;
	} > L1

	.bss : ALIGN(4) {
		_bss = .;

		*(.sbss .sbss.* .bss .bss.*);

		. = ALIGN(4);

		_ebss = .;

		. = ALIGN(16);

		__firmware_end = .;
	} > L1

	.stack : ALIGN(16) {
		__stack_bottom = .;
		. += __firmware_stack_size;
		__stack_top = .;
	} > LDM

	/* fake output .got section */
	/* Dynamic relocations are unsupported. This section is only used to detect
	relocatable code in the input files and raise an error if relocatable code
	is found */
	/*
	.got (INFO) : {
		KEEP(*(.got .got.*));
	}
	*/
}

PROVIDE(__L1_END = ORIGIN(L1) + LENGTH(L1));
PROVIDE(__LDM_END = ORIGIN(LDM) + LENGTH(LDM));

/*
ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
*/
