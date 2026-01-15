#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Ép Linker giữ lại đoạn này ở ngay đầu file
#[link_section = ".vector_table"]
#[no_mangle]
pub static BOOT_VECTOR: [u32; 2] = [
    0xe59ff000, // Lệnh ARM: ldr pc, [pc, #0]
    _start as u32,
];

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
