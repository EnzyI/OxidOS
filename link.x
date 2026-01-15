ENTRY(_reset)
MEMORY
{
  /* VersatilePB bắt đầu đọc từ địa chỉ 0 */
  RAM : ORIGIN = 0, LENGTH = 128M
}
SECTIONS
{
  .text : {
    KEEP(*(.vector_table)) 
    *(.text .text.*)
  } > RAM
}
