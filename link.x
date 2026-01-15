ENTRY(_reset) /* Điểm vào là Assembly */

MEMORY
{
  FLASH : ORIGIN = 0x10000, LENGTH = 1M
  RAM   : ORIGIN = 0x20000, LENGTH = 1M
}

SECTIONS
{
  .text : {
    KEEP(*(.vector_table)) /* Đặt lệnh nhảy của boot.s ở byte đầu tiên */
    *(.text .text.*)
  } > FLASH
}
