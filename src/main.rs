#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Dùng lệnh Thumb ngay từ đầu để khớp với target của bro
global_asm!(
    ".section .vector_table, \"ax\"",
    ".global _reset",
    ".thumb",                  // Ép dùng Thumb mode
    "_reset:",
    "ldr r0, =_start",         // Nạp địa chỉ hàm _start
    "blx r0",                  // Nhảy vào Rust
    ".align 4"
);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Địa chỉ UART0 của VersatilePB
    let uart0 = 0x101f_1000 as *mut u8;
    
    // In chữ "X" liên tục để dễ nhận diện
    loop {
        unsafe {
            core::ptr::write_volatile(uart0, b'X');
        }
        // Thêm một vòng lặp chờ nhỏ để không làm nghẽn QEMU
        for _ in 0..100000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
