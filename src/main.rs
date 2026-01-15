#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Dùng lệnh Thumb ngay từ đầu để khớp với target của bro
global_asm!(
    ".section .vector_table, \"ax\"",
    ".global _reset",
    ".thumb",                  // Ép dùng Thumb mode
    "_reset:",
    "ldr r0, =_start",         // Nạp địa chỉ hàm _start
    "blx r0",                  // Nhảy vào Rust
    ".align 4"
);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = 0x101f_1000 as *mut u8;
    // Thanh ghi trạng thái (Flag Register) để kiểm tra UART có bận không
    let uart_fr = 0x101f_1018 as *const u32;

    let msg = b"ALIVE AND RUNNING!\n";

    for &byte in msg {
        unsafe {
            // Đợi cho đến khi hàng đợi truyền (TX) không còn đầy
            while (core::ptr::read_volatile(uart_fr) & 0x20) != 0 {}
            // Ghi dữ liệu
            core::ptr::write_volatile(uart0, byte);
        }
    }

    loop {}
}
