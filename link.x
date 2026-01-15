ENTRY(_reset) /* Bảo Linker bắt đầu từ file Assembly */

MEMORY
{
  FLASH : ORIGIN = 0x10000, LENGTH = 1M
  RAM   : ORIGIN = 0x20000, LENGTH = 1M
}

SECTIONS
{
  .text : {
    KEEP(*(.vector_table)) /* Giữ lệnh nhảy ở vị trí đầu tiên */
    *(.text .text.*)
  } > FLASH

  .ARM.exidx : { *(.ARM.exidx*) } > FLASH
}
