ENTRY(_reset_handler)
SECTIONS
{
    . = 0x00000000;
    .text : {
        KEEP(*(.vector_table)) /* Bắt buộc bảng vector ở 0x0 */
        *(.text .text.*)
    }
}
