ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    .vector_table ORIGIN(FLASH) : {
        LONG(0x20005000);         /* 1. Stack Pointer */
        LONG(_reset_handler | 1);  /* 2. Reset Vector */
        LONG(0);                   /* 3. NMI */
        LONG(0);                   /* 4. HardFault */
        /* Padding cho các ngắt khác */
        LONG(0); LONG(0); LONG(0); LONG(0); LONG(0); LONG(0); LONG(0);
        LONG(0);                   /* 12. SVCall */
        LONG(0);                   /* 13. Reserved */
        LONG(0);                   /* 14. PendSV */
        LONG(SysTick_Handler | 1); /* 15. SysTick - Nhịp tim ở đây! */
    } > FLASH

    .text : { *(.text .text.*) } > FLASH
}
