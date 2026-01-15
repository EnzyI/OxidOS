.syntax unified
.section .vector_table, "ax"
.global _reset

_reset:
    b _start
