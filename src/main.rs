#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Đưa assembly "mồi" trực tiếp vào đây
global_asm!(
    ".section .vector_table, \"ax\"",
    ".global _reset",
    "_reset:",
    "b _start"
);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Ghi trực tiếp vào UART của máy VersatilePB
    let uart = 0x101f_1000 as *mut u8;
    for &byte in b"ALIVE" {
        unsafe { core::ptr::write_volatile(uart, byte); }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
