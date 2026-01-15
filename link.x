MEMORY
{
  /* Máy versatilepb bắt đầu thực thi từ địa chỉ 0x10000 */
  FLASH : ORIGIN = 0x10000, LENGTH = 1M
  RAM   : ORIGIN = 0x20000, LENGTH = 1M
}

SECTIONS
{
  .text : {
    *(.vector_table)
    *(.text .text.*)
  } > FLASH

  .ARM.exidx : {
    *(.ARM.exidx*)
  } > FLASH

  /DISCARD/ : {
    *(.ARM.attributes)
  }
}
