ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    .vector_table ORIGIN(FLASH) : {
        LONG(0x20005000);          /* Initial Stack Pointer */
        LONG(_reset_handler | 1);   /* Reset Vector */
        FILL(0); . = 0x3C;
        LONG(SysTick_Handler | 1);  /* SysTick tại offset 0x3C */
    } > FLASH

    .text : { *(.text .text.*) } > FLASH
}
