#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{Write, Result};

// 1. Tạo cấu trúc UART
struct Uart {
    base_ptr: *mut u8,
}

// 2. Triển khai Write cho Uart để dùng được macro write!
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            unsafe {
                core::ptr::write_volatile(self.base_ptr, byte);
            }
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    // Khởi tạo Driver với địa chỉ UART0 của máy LM3S
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u8 };

    // Bây giờ bro có thể in bất cứ thứ gì cực dễ dàng
    let _ = write!(uart, "\n--- OXID OS BOOTING ---\n");
    let _ = write!(uart, "Status: {}\n", "READY");
    let _ = write!(uart, "System Time: {} ms\n", 2026);
    let _ = write!(uart, "-----------------------\n");

    loop {
        let _ = write!(uart, "ALIVE ");
        for _ in 0..2000000 { core::hint::spin_loop(); }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u8 };
    let _ = write!(uart, "\n!!! KERNEL PANIC !!!\n");
    if let Some(location) = info.location() {
        let _ = write!(uart, "At: {}:{}\n", location.file(), location.line());
    }
    loop {}
}
