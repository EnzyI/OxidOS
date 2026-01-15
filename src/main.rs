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
    // UART0 của VersatilePB
    let uart = 0x101f_1000 as *mut u8;
    let msg = b"ALIVE AND RUNNING!\n";

    for &byte in msg {
        unsafe {
            core::ptr::write_volatile(uart, byte);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
