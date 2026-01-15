ENTRY(_reset)
MEMORY
{
  FLASH : ORIGIN = 0x10000, LENGTH = 1M
  RAM   : ORIGIN = 0x20000, LENGTH = 1M
}
SECTIONS
{
  . = 0x10000;
  .text : {
    KEEP(*(.vector_table)) /* Bắt buộc giữ cái này ở đầu file */
    *(.text .text.*)
  } > FLASH
}
