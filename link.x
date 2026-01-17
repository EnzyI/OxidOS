ENTRY(_reset_handler)
SECTIONS
{
    . = 0x00000000;
    .text : {
        /* Điền trực tiếp 8 byte đầu tiên cho CPU */
        LONG(0x20005000);          /* 1. Stack Pointer */
        LONG(_reset_handler | 1);  /* 2. Reset Vector */
        
        *(.text .text.*)
    }
}
