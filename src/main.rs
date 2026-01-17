#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// --- BỘ CẤP PHÁT BỘ NHỚ ---
struct BumpingAllocator { next: AtomicUsize }
unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.next.fetch_add(layout.size(), Ordering::SeqCst) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator { next: AtomicUsize::new(0x2000_2000) };

// --- DRIVER UART ---
struct Uart { base_ptr: *mut u32 }
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() { unsafe { core::ptr::write_volatile(self.base_ptr, byte as u32); } }
        Ok(())
    }
}

// --- KHỞI TẠO SYSTICK (NHỊP TIM) ---
fn init_systick(ticks: u32) {
    let systick_base = 0xE000_E010 as *mut u32;
    unsafe {
        core::ptr::write_volatile(systick_base.add(1), ticks); // Reload
        core::ptr::write_volatile(systick_base.add(2), 0);     // Clear current
        core::ptr::write_volatile(systick_base, 0x07);        // Enable + Int + Source
    }
}

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
    let _ = writeln!(uart, "\x1b[2J\x1b[H\x1b[32m[OXID RTOS]\x1b[0m SysTick starting...");
    
    init_systick(120_000); // 10ms heartbeat
    
    let _ = writeln!(uart, "[SYSTEM] Heartbeat is pulsing. OS is ALIVE.");
    loop { unsafe { core::arch::asm!("wfi"); } }
}

#[alloc_error_handler] fn alloc_error(_layout: Layout) -> ! { loop {} }
#[panic_handler] fn panic(_info: &PanicInfo) -> ! { loop {} }
