ENTRY(_reset_handler)
SECTIONS
{
    . = 0x0;
    .text : {
        KEEP(*(.vector_table)) 
        *(.text .text.*)
    }
}
