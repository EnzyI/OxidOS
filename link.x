MEMORY
{
  /* Địa chỉ bắt đầu của máy ảo QEMU Virt là 0x40000000 */
  FLASH : ORIGIN = 0x40000000, LENGTH = 1M
  RAM   : ORIGIN = 0x80000000, LENGTH = 1M
}

SECTIONS
{
  /* Điểm bắt đầu của chương trình */
  .text : {
    *(.vector_table)
    *(.text .text.*)  /* Thêm .text.* để gom cả hàm _start */
  } > FLASH

  /* Các bảng thông tin ARM mà trình biên dịch tự tạo ra */
  .ARM.exidx : {
    *(.ARM.exidx*)
  } > FLASH

  /DISCARD/ : {
    *(.ARM.attributes)
  }
}
