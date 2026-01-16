#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Định nghĩa kiểu hàm để làm Vector Table
type Handler = unsafe extern "C" fn() -> !;

#[link_section = ".vector_table"]
#[no_mangle]
pub static MS_VECTOR_TABLE: [Option<Handler>; 2] = [
    unsafe { core::mem::transmute(0x2000_5000usize) }, // Stack Pointer ban đầu
    Some(_start),                                     // Reset Handler
];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart0 = 0x4000_c000 as *mut u8;

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
