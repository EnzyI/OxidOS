#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    ".arm",
    ".section .vector_table, \"ax\"",
    ".global _reset",
    "_reset:",
    "ldr pc, =_start", // Lệnh nhảy này sẽ không bắt đầu bằng byte 00
    ".align 4"
);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart = 0x101f_1000 as *mut u8;
    for &byte in b"ALIVE\n" {
        unsafe { core::ptr::write_volatile(uart, byte); }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
