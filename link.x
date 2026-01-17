ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    /* Ép bảng Vector Table vào địa chỉ 0x0 tuyệt đối */
    .vector_table 0x00000000 : {
        LONG(0x20005000);          /* Initial Stack Pointer */
        LONG(_reset_handler | 1);   /* Reset Vector */
        LONG(0);                    /* NMI */
        LONG(0);                    /* HardFault */
        /* Giữ khoảng trống 11 vị trí đầu tiên */
        FILL(0); . = 0x3C;
        LONG(SysTick_Handler | 1);  /* SysTick nằm ở offset 0x3C */
    } > FLASH

    .text : { *(.text .text.*) } > FLASH
}
