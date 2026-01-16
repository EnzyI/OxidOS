ENTRY(_start)
SECTIONS
{
    . = 0x0;
    .text : {
        KEEP(*(.vector_table)) 
        *(.text .text.*)
    }
}
