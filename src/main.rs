#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Bảng Vector Table dùng u32 để trình biên dịch không báo lỗi pointer
#[link_section = ".vector_table"]
#[no_mangle]
pub static MS_VECTOR_TABLE: [u32; 2] = [
    0x2000_5000,               // 1. Initial Stack Pointer
    _reset_handler as u32,     // 2. Reset Handler (Địa chỉ hàm khởi động)
];

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let uart0 = 0x4000_c000 as *mut u8; // UART cho máy LM3S

    loop {
        for &byte in b"ALIVE AT LAST!\n" {
            unsafe { core::ptr::write_volatile(uart0, byte); }
        }
        // Delay để terminal điện thoại kịp hiển thị
        for _ in 0..1000000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
