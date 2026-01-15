#![no_std]
#![no_main]
#![feature(compiler_builtins)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Hàm gửi một ký tự ra cổng Serial
fn print_char(c: u8) {
    // Địa chỉ thanh ghi dữ liệu của UART (thay đổi tùy chip, đây là ví dụ STM32)
    let uart_dr = 0x4001_1004 as *mut u8;
    unsafe {
        // Ghi trực tiếp vào bộ nhớ phần cứng
        *uart_dr = c;
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let message = b"OxidOS online!";
    
    for &byte in message {
        print_char(byte);
    }

    loop {}
}
