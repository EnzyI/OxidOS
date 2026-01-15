#![no_std]
#![no_main]
#![feature(compiler_builtins)] // Mở khóa tính năng đặc biệt của Nightly

use core::panic::PanicInfo;

// Hàm xử lý lỗi nghiêm trọng
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Điểm khởi đầu của nhân hệ điều hành
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // OxidOS bắt đầu ở đây
    loop {}
}
