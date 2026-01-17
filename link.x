ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    .vector_table ORIGIN(FLASH) : {
        LONG(0x20005000);         /* 01: Stack Pointer */
        LONG(_reset_handler | 1);  /* 02: Reset */
        LONG(0);                   /* 03: NMI */
        LONG(0);                   /* 04: HardFault */
        LONG(0);                   /* 05: MemManage */
        LONG(0);                   /* 06: BusFault */
        LONG(0);                   /* 07: UsageFault */
        LONG(0); LONG(0); LONG(0); LONG(0); /* 08-11: Reserved */
        LONG(0);                   /* 12: SVCall */
        LONG(0);                   /* 13: Reserved */
        LONG(0);                   /* 14: PendSV */
        LONG(SysTick_Handler | 1); /* 15: SysTick */
    } > FLASH

    .text : { *(.text .text.*) } > FLASH
}
