#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    // UART0 cho máy Stellaris LM3S
    let uart0 = 0x4000_c000 as *mut u8;

    loop {
        for &byte in b"ALIVE AT LAST!\n" {
            unsafe { core::ptr::write_volatile(uart0, byte); }
        }
        // Vòng lặp chờ để Terminal không bị lag
        for _ in 0..1000000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
