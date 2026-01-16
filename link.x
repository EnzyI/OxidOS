ENTRY(_reset_handler)

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
    /* Đặt bảng Vector cố định tại địa chỉ 0x0 */
    .vector_table ORIGIN(FLASH) : {
        LONG(0x20005000);        /* Top of Stack */
        LONG(_reset_handler | 1); /* Reset Vector */
        KEEP(*(.vector_table))
    } > FLASH

    .text : {
        *(.text .text.*)
    } > FLASH

    /* Vùng RAM dành cho Allocator bắt đầu từ 0x20002000 */
    .data : {
        *(.data .data.*)
    } > RAM AT > FLASH
}
