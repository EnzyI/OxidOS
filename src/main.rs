#![no_std]
#![no_main]
#![feature(compiler_builtins)]

use core::panic::PanicInfo;

// 1. XỬ LÝ LỖI HỆ THỐNG (PANIC)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Nếu có lỗi, in thông báo hoặc đứng im
    loop {}
}

// 2. BẢN THIẾT KẾ DRIVER CHUNG
pub trait SerialDevice {
    fn write_byte(&self, byte: u8);
    
    fn write_str(&self, s: &str) {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
    }
}

// 3. CÀI ĐẶT UART CHO MÁY ẢO QEMU 'VIRT'
struct UartDriver {
    address: u32,
}

impl SerialDevice for UartDriver {
    fn write_byte(&self, byte: u8) {
        let ptr = self.address as *mut u8;
        unsafe {
            // Dùng write_volatile để ghi trực tiếp vào cổng UART của QEMU
            core::ptr::write_volatile(ptr, byte);
        }
    }
}

// 4. ĐIỂM KHỞI CHẠY OXIDOS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Trong hàm _start của src/main.rs
let console = UartDriver { address: 0x0900_0000 };


    
    // Gửi dữ liệu vào file log
    console.write_str("\n==============================\n");
    console.write_str("   OxidOS v0.1.0 IS ALIVE!    \n");
    console.write_str("==============================\n");
    console.write_str("Status: Kernel running on QEMU Virt\n");
    console.write_str("Security: Rust memory safety active\n");

    loop {
        // CPU sẽ nghỉ ngơi ở đây sau khi in xong
    }
}
