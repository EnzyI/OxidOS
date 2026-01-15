MEMORY
{
  /* Giả lập một con chip đơn giản với 256KB Flash và 64KB RAM */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
  .vector_table :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM)); /* Điểm đầu của Stack pointer */
    KEEP(*(.vector_table.reset_vector)); /* Reset vector */
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}
