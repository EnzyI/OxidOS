#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{Write, Result};

struct Uart {
    base_ptr: *mut u8,
}

impl Uart {
    // Hàm đọc 1 byte từ bàn phím (UART nhận dữ liệu)
    fn read_byte(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.base_ptr) }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            unsafe { core::ptr::write_volatile(self.base_ptr, byte); }
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u8 };

    // In menu có màu sắc (Màu xanh: \x1b[32m, Reset: \x1b[0m)
    let _ = write!(uart, "\x1b[32m\n--- OXID OS LOADED SUCCESSFULLY ---\x1b[0m\n");
    let _ = write!(uart, "\x1b[33m[SYSTEM]\x1b[0m Type something on your phone:\n");

    loop {
        // Đọc phím từ bro gõ vào
        let key = uart.read_byte();
        if key != 0 {
            // In ngược lại phím đó ra màn hình (Echo)
            let _ = write!(uart, "\x1b[36m{}\x1b[0m", key as char);
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u8 };
    let _ = write!(uart, "\x1b[31m\n!!! KERNEL PANIC !!!\x1b[0m\n");
    loop {}
}
