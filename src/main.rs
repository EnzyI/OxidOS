#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// Đoạn mã Assembly khởi động quan trọng nhất
global_asm!(
    ".arm",                    // Bắt đầu ở chế độ ARM 32-bit (mặc định của QEMU)
    ".section .vector_table, \"ax\"",
    ".global _reset",
    "_reset:",
    "add r0, pc, #1",          // Thủ thuật: lấy địa chỉ hiện tại + 1 để bật cờ Thumb
    "bx r0",                   // Nhảy đến r0 và chuyển CPU sang chế độ Thumb
    
    ".thumb",                  // Từ đây code sẽ chạy ở chế độ Thumb (như Rust build)
    ".global _start_thumb",
    "_start_thumb:",
    "ldr r0, =_start",         // Nạp địa chỉ hàm _start của Rust
    "blx r0",                  // Nhảy vào hàm Rust
    ".align 4"
);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Địa chỉ cổng Serial (UART0) của máy VersatilePB
    let uart = 0x101f_1000 as *mut u8;

    // Chuỗi thông báo "sống sót"
    let msg = b"ALIVE AND RUNNING!\n";

    for &byte in msg {
        unsafe {
            // Ghi từng byte vào UART để hiện lên Terminal
            core::ptr::write_volatile(uart, byte);
        }
    }

    // Vòng lặp vô tận để CPU không bị hoảng loạn (panic)
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
