#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::string::String;
use alloc::format;
use core::panic::PanicInfo;
use core::fmt::{Write, Result};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// --- BỘ QUẢN LÝ BỘ NHỚ (ALLOCATOR) ---
struct BumpingAllocator {
    next: AtomicUsize,
}

unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = self.next.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |prev| {
            let aligned = (prev + layout.align() - 1) & !(layout.align() - 1);
            Some(aligned + layout.size())
        }).unwrap();
        ((start + layout.align() - 1) & !(layout.align() - 1)) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator {
    next: AtomicUsize::new(0x2000_2000), // Vùng RAM trống trên máy LM3S
};

// --- DRIVER UART (VÁ LỖI E0422) ---
struct Uart {
    base_ptr: *mut u32,
}

impl Uart {
    fn putc(&mut self, c: u8) {
        unsafe { core::ptr::write_volatile(self.base_ptr, c as u32); }
    }

    fn getc(&self) -> u8 {
        let fr = unsafe { core::ptr::read_volatile(self.base_ptr.add(6)) };
        if (fr & (1 << 4)) != 0 { return 0; }
        unsafe { core::ptr::read_volatile(self.base_ptr) as u8 }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() { self.putc(byte); }
        Ok(())
    }
}

// --- KHỞI ĐỘNG HỆ THỐNG ---
#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };

    // Dùng String và format! nhờ có Allocator
    let msg = format!("\x1b[35m[OXID-ALLOC]\x1b[0m Heap initialized at 0x20002000\n");
    let _ = write!(uart, "{}", msg);
    let _ = write!(uart, "\x1b[32m[OK]\x1b[0m CLI is active. Gõ 'h' để xem lệnh.\n> ");

    loop {
        let key = uart.getc();
        if key != 0 {
            match key {
                b'h' => { let _ = write!(uart, "\n\x1b[33mLệnh: h (help), c (clear)\x1b[0m\n> "); }
                b'c' => { let _ = write!(uart, "\x1b[2J\x1b[H> "); }
                _ => { let _ = write!(uart, "\x1b[36m{}\x1b[0m", key as char); }
            }
        }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! { loop {} }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
