ENTRY(_reset_handler)
MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}
SECTIONS {
    .vector_table 0x00000000 : {
        LONG(0x20005000);
        LONG(_reset_handler | 1);
        FILL(0); . = 0x3C;
        LONG(SysTick_Handler | 1);
    } > FLASH
    .text : { *(.text .text.*) } > FLASH
}
