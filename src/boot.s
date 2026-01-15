.section .vector_table, "ax"
.global _reset
_reset:
    b _start  /* CPU nhảy vào đây và bị ép phải chạy _start của Rust */
