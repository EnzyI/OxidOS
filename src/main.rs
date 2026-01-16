#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::{Write, Result};

struct Uart {
    base_ptr: *mut u32, // Đổi sang u32 để khớp với thanh ghi chip
}

impl Uart {
    fn putc(&mut self, c: u8) {
        unsafe { core::ptr::write_volatile(self.base_ptr, c as u32); }
    }

    fn getc(&self) -> u8 {
        // Thanh ghi Flag (FR) nằm cách gốc 0x18 byte
        let fr = unsafe { core::ptr::read_volatile(self.base_ptr.add(6)) };
        // Kiểm tra bit RXFE (Receive FIFO Empty) - bit số 4
        // Nếu bit này bằng 1 nghĩa là KHÔNG có dữ liệu, ta trả về 0
        if (fr & (1 << 4)) != 0 {
            return 0;
        }
        unsafe { core::ptr::read_volatile(self.base_ptr) as u8 }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() { self.putc(byte); }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };

    let _ = write!(uart, "\x1b[32m\n[OK] OXID OS CLI READY\x1b[0m\n");
    let _ = write!(uart, "Type 'h' for help: ");

    loop {
        let key = uart.getc();
        if key != 0 {
            match key {
                b'h' => { let _ = write!(uart, "\n\x1b[33mCommands: h (help), c (clear)\x1b[0m\n> "); }
                b'c' => { let _ = write!(uart, "\x1b[2J\x1b[H> "); } // Xóa màn hình
                _ => { let _ = write!(uart, "\x1b[36m{}\x1b[0m", key as char); }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
