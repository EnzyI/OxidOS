#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::panic::PanicInfo;
use core::fmt::{Write, Result};
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// --- BẢNG VECTOR TABLE CỐ ĐỊNH ---
#[link_section = ".vector_table"]
#[no_mangle]
pub static VECTOR_TABLE: [u32; 2] = [
    0x2000_5000,               // Stack Pointer
    _reset_handler as u32 | 1, // Reset Handler + Thumb bit
];

// --- ALLOCATOR ---
struct BumpingAllocator { next: AtomicUsize }
unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = self.next.fetch_add(layout.size(), Ordering::SeqCst);
        start as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator { next: AtomicUsize::new(0x2000_2000) };

// --- UART DRIVER ---
struct Uart { base_ptr: *mut u32 }
impl Uart {
    fn putc(&mut self, c: u8) { unsafe { core::ptr::write_volatile(self.base_ptr, c as u32); } }
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

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
    let _ = write!(uart, "\x1b[2J\x1b[H\x1b[32m[OK] OXID OS BOOTED!\x1b[0m\n> ");
    loop {
        let key = uart.getc();
        if key != 0 { let _ = write!(uart, "\x1b[36m{}\x1b[0m", key as char); }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! { loop {} }
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
