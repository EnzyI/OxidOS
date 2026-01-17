ENTRY(_reset_handler)

SECTIONS
{
    . = 0x0;
    /* Linker tự tính địa chỉ _reset_handler và điền vào 8 byte đầu */
    .vector_table : {
        LONG(0x20005000);         /* 1. Stack Pointer */
        LONG(_reset_handler | 1);  /* 2. Reset Vector */
    } > FLASH

    .text : {
        *(.text .text.*)
    } > FLASH
}

MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
}
