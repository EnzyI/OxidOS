ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    . = 0x0;
    .vector_table : { KEEP(*(.vector_table)) } > FLASH
    .text : { *(.text .text.*) } > FLASH

    /* Vùng nhớ Heap sẽ nằm sau vùng code */
    _heap_start = .;
    _heap_end = ORIGIN(RAM) + LENGTH(RAM);
}
