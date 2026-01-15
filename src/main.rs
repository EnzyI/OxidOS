#![no_std]
#![no_main]
#![feature(compiler_builtins)]

use core::panic::PanicInfo;

// -------------------------------------------------------------------
// 1. CẤU HÌNH PANIC (Khi hệ điều hành gặp lỗi nghiêm trọng)
// -------------------------------------------------------------------
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Trong một OS thực thụ, đây là nơi ta in mã lỗi lên màn hình xanh
    loop {}
}

// -------------------------------------------------------------------
// 2. BẢN THIẾT KẾ DRIVER (Trait SerialDevice)
// -------------------------------------------------------------------
pub trait SerialDevice {
    fn write_byte(&self, byte: u8);
    
    fn write_str(&self, s: &str) {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
    }
}

// -------------------------------------------------------------------
// 3. TRIỂN KHAI UART DRIVER (Cho chip STM32/ARM giả lập)
// -------------------------------------------------------------------
struct UartDriver {
    address: u32,
}

impl SerialDevice for UartDriver {
    fn write_byte(&self, byte: u8) {
        let ptr = self.address as *mut u8;
        unsafe {
            // Dùng write_volatile để đảm bảo lệnh không bị trình biên dịch xóa
            core::ptr::write_volatile(ptr, byte);
        }
    }
}

// -------------------------------------------------------------------
// 4. ĐIỂM KHỞI ĐẦU CỦA OXIDOS (Entry Point)
// -------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Địa chỉ UART1 thường gặp trên dòng Cortex-M (STM32)
    let console = UartDriver { address: 0x4001_1004 };
    
    // Xuất lời chào "thương hiệu" của OxidOS
    console.write_str("\n--- OxidOS v0.1.0 ---\n");
    console.write_str("Status: Hardened & Secure Kernel booting...\n");
    console.write_str("Architecture: ARM Cortex-M4\n");
    console.write_str("System Ready.\n");

    // Vòng lặp chính của nhân hệ điều hành
    loop {
        // Sau này ta sẽ xử lý các tác vụ (Tasks) ở đây
    }
}
