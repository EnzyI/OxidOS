ENTRY(_reset_handler)

SECTIONS
{
    . = 0x0;
    .vector_table : {
        LONG(0x20005000);        /* 1. Initial Stack Pointer */
        LONG(_reset_handler | 1); /* 2. Reset Vector (Dùng Thumb mode với bit | 1) */
    } > FLASH

    .text : {
        *(.text .text.*)
    } > FLASH
}

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}
