MEMORY
{
  /* Máy ảo Virt bắt đầu đọc bộ nhớ tại đây */
  FLASH : ORIGIN = 0x40000000, LENGTH = 1M
  RAM   : ORIGIN = 0x80000000, LENGTH = 1M
}

SECTIONS
{
  .vector_table : { *(.vector_table) } > FLASH
  .text : { *(.text) } > FLASH
}
