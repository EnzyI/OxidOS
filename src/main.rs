#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Khi OxidOS gặp lỗi không thể hồi phục
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Điểm khởi đầu thực sự của OxidOS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Mã nguồn điều khiển CPU sẽ nằm ở đây
    loop {}
}
