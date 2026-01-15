ENTRY(_reset)
SECTIONS
{
    /* Ép mọi thứ bắt đầu từ 0x10000 của VersatilePB */
    . = 0x10000;
    .text : {
        KEEP(*(.vector_table))
        *(.text .text.*)
    }
}
