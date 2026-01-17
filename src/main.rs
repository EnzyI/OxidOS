#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, AtomicU32, Ordering};

// --- QUẢN LÝ NHỊP TIM ---
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    let count = TICK_COUNT.fetch_add(1, Ordering::SeqCst);
    if count % 100 == 0 {
        let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
        let _ = write!(uart, "\x1b[33m.\x1b[0m"); 
    }
}

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

// --- UART DRIVER ---
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

// --- HÀM KHỞI TẠO SYSTICK (FIX LỖI E0425) ---
fn init_systick(ticks: u32) {
    let systick_base = 0xE000_E010 as *mut u32;
    unsafe {
        core::ptr::write_volatile(systick_base.add(1), ticks);
        core::ptr::write_volatile(systick_base.add(2), 0);
        core::ptr::write_volatile(systick_base, 0x07);
    }
}

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
    let _ = write!(uart, "\x1b[2J\x1b[H\x1b[32m[OXID RTOS v1.0]\x1b[0m\n> ");
    
    init_systick(120_000); // Khởi tạo nhịp tim 10ms

    let mut buffer = [0u8; 32]; 
    let mut pos = 0;

    loop {
        let key = uart.getc();
        if key != 0 {
            match key {
                b'\r' | b'\n' => {
                    let _ = write!(uart, "\n");
                    let cmd = core::str::from_utf8(&buffer[..pos]).unwrap_or("");
                    match cmd {
                        "cls" => { let _ = write!(uart, "\x1b[2J\x1b[H"); }
                        "ver" => { let _ = write!(uart, "OXID RTOS Kernel v1.0 (Rust)\n"); }
                        _ => { if pos > 0 { let _ = write!(uart, "Unknown: {}\n", cmd); } }
                    }
                    pos = 0;
                    let _ = write!(uart, "> ");
                }
                b'\x08' | b'\x7f' => {
                    if pos > 0 { pos -= 1; let _ = write!(uart, "\x08 \x08"); }
                }
                _ => {
                    if pos < buffer.len() {
                        buffer[pos] = key;
                        pos += 1;
                        let _ = write!(uart, "{}", key as char);
                    }
                }
            }
        }
        unsafe { core::arch::asm!("wfi"); }
    }
}

#[alloc_error_handler] fn alloc_error(_layout: Layout) -> ! { loop {} }
#[panic_handler] fn panic(_info: &PanicInfo) -> ! { loop {} }
