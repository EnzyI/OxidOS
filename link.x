ENTRY(_start)
SECTIONS
{
    . = 0x0;
    .text : {
        KEEP(*(.vector_table)) /* Giữ bảng vector ở vị trí 0x0 */
        *(.text .text.*)
    }
}
