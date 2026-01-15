ENTRY(_reset)
MEMORY
{
  /* Địa chỉ nạp code của versatilepb */
  FLASH : ORIGIN = 0x10000, LENGTH = 1M
  RAM   : ORIGIN = 0x20000, LENGTH = 1M
}
SECTIONS
{
  /* Bắt đầu chính xác tại 0x10000 */
  . = 0x10000;
  
  .text : {
    /* Ép bảng vector (chứa lệnh nhảy) nằm ở byte 0 của file */
    KEEP(*(.vector_table)) 
    *(.text .text.*)
  } > FLASH
}
