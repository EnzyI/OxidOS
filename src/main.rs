#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // UART0 trên máy lm3s6965evb nằm ở địa chỉ này
    let uart0 = 0x4000_c000 as *mut u8;

    loop {
        for &byte in b"ALIVE ON LM3S!\n" {
            unsafe {
                core::ptr::write_volatile(uart0, byte);
            }
        }
        // Delay một chút
        for _ in 0..500000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
