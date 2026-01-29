#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, AtomicU32, Ordering};

// --- CẤU HÌNH BỘ NHỚ ---
const HEAP_START: usize = 0x2000_2000;
const HEAP_SIZE: usize = 32 * 1024; 

struct BumpingAllocator { next: AtomicUsize }
unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = self.next.load(Ordering::SeqCst);
        if current + layout.size() > HEAP_START + HEAP_SIZE { return core::ptr::null_mut(); }
        self.next.fetch_add(layout.size(), Ordering::SeqCst) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator { next: AtomicUsize::new(HEAP_START) };

// --- NHỊP TIM & UART ---
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    let count = TICK_COUNT.fetch_add(1, Ordering::SeqCst);
    if count % 100 == 0 {
        let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
        let _ = write!(uart, "\x1b[33m.\x1b[0m"); 
    }
}

struct Uart { base_ptr: *mut u32 }
impl Uart {
    fn getc(&self) -> u8 {
        let fr = unsafe { core::ptr::read_volatile(self.base_ptr.add(6)) };
        if (fr & (1 << 4)) != 0 { return 0; }
        unsafe { core::ptr::read_volatile(self.base_ptr) as u8 }
    }
}
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() { unsafe { core::ptr::write_volatile(self.base_ptr, byte as u32); } }
        Ok(())
    }
}

fn init_systick(ticks: u32) {
    unsafe {
        let s = 0xE000_E010 as *mut u32;
        core::ptr::write_volatile(s.add(1), ticks);
        core::ptr::write_volatile(s.add(2), 0);
        core::ptr::write_volatile(s
#[alloc_error_handler] fn alloc_error(_layout: Layout) -> ! { loop {} }
#[panic_handler] fn panic(_info: &PanicInfo) -> ! { loop {} }
