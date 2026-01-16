ENTRY(_reset)
SECTIONS
{
    . = 0x10000; /* Địa chỉ nạp mặc định của QEMU cho VersatilePB */
    .text : {
        KEEP(*(.vector_table))
        *(.text .text.*)
    }
}
