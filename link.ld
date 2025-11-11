ENTRY(_start)

SECTIONS {
  . = 0x80000000;

  .text : {
    KEEP(*(.text._start))
    KEEP(*(.init))
    *(.text .text.*)
  }

  .rodata : { *(.rodata .rodata.*) }
  .data : { *(.data .data.*) }
  .bss  : { *(.bss .bss.*) *(COMMON) }

  /DISCARD/ : { *(.eh_frame) *(.comment) }
}
