#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Định nghĩa bảng Vector Table ở đầu file
#[link_section = ".vector_table"]
#[no_mangle]
pub static MS_VECTOR_TABLE: [u32; 2] = [
    0x2000_5000,    // 1. Initial Stack Pointer (Địa chỉ cuối RAM)
    _start as u32,  // 2. Reset Handler (Địa chỉ hàm bắt đầu)
];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = 0x4000_c000 as *mut u8; // UART cho máy lm3s

    loop {
        for &byte in b"ALIVE AT LAST!\n" {
            unsafe {
                core::ptr::write_volatile(uart0, byte);
            }
        }
        for _ in 0..1000000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
