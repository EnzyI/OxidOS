#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Địa chỉ UART0 của máy Stellaris LM3S6965
    let uart0 = 0x4000_c000 as *mut u8;

    loop {
        for &byte in b"ALIVE ON LM3S!\n" {
            unsafe {
                core::ptr::write_volatile(uart0, byte);
            }
        }
        // Vòng lặp chờ để không làm treo terminal điện thoại
        for _ in 0..1000000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
