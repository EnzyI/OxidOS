#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Địa chỉ thanh ghi truyền dữ liệu UART trên nRF51 (Micro:bit)
    let uart_txd = 0x40002508 as *mut u32;
    // Thanh ghi bắt đầu truyền
    let uart_starttx = 0x40002008 as *mut u32;

    unsafe {
        // Bật UART truyền
        core::ptr::write_volatile(uart_starttx, 1);
        
        loop {
            for &byte in b"ALIVE\n" {
                core::ptr::write_volatile(uart_txd, byte as u32);
                // Đợi một chút để UART kịp gửi
                for _ in 0..10000 { core::hint::spin_loop(); }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
