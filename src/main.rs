#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::panic::PanicInfo;
use core::fmt::Write;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, AtomicU32, Ordering};

// --- QUẢN LÝ BỘ NHỚ ---
const HEAP_START: usize = 0x2000_2000;
const HEAP_SIZE: usize = 32 * 1024; 

struct BumpingAllocator { next: AtomicUsize }
unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = self.next.load(Ordering::SeqCst);
        if current + layout.size() > HEAP_START + HEAP_SIZE { return core::ptr::null_mut(); }
        // Thêm lệnh black_box ngầm để đảm bảo việc tăng next luôn xảy ra
        self.next.fetch_add(layout.size(), Ordering::SeqCst) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator { next: AtomicUsize::new(HEAP_START) };

// --- UART & NHỊP TIM ---
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
        core::ptr::write_volatile(s, 0x07);
    }
}

// --- COMMAND LOOP ---
#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };
    let _ = write!(uart, "\x1b[2J\x1b[H\x1b[32m[OXID RTOS v1.0]\x1b[0m Ready.\n> ");
    init_systick(120_000);
    
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
                        "ver" => { let _ = write!(uart, "OXID RTOS v1.0.0 (Rust)\n"); }
                        "free" => {
                            let used = ALLOCATOR.next.load(Ordering::SeqCst) - HEAP_START;
                            let _ = write!(uart, "Heap Used: {}/{} bytes\n", used, HEAP_SIZE);
                        }
                        "test" => {
                            let layout = Layout::from_size_align(1024, 4).unwrap();
                            let ptr = unsafe { ALLOCATOR.alloc(layout) };
                            if !ptr.is_null() {
                                // Ghi giá trị ảo để tránh bị compiler tối ưu xóa bỏ
                                unsafe { core::ptr::write_volatile(ptr as *mut u32, 0xDEADBEEF); }
                                let _ = write!(uart, "Successfully allocated and wrote to 1KB!\n");
                            } else { let _ = write!(uart, "Error: OOM!\n"); }
                        }
                        "peek" => {
                            let addr = 0x2000_2000 as *const u32; // Đọc ngay đầu Heap
                            let val = unsafe { core::ptr::read_volatile(addr) };
                            let _ = write!(uart, "Data at 0x20002000: 0x{:08X}\n", val);
                        }
                        "poke" => {
                            let addr = 0x2000_2000 as *mut u32;
                            unsafe { core::ptr::write_volatile(addr, 0x12345678); }
                            let _ = write!(uart, "Wrote 0x12345678 to 0x20002000\n");
                        }
                        _ => { if pos > 0 { let _ = write!(uart, "Unknown: {}\n", cmd); } }
                    }
                    pos = 0; let _ = write!(uart, "> ");
                }
                b'\x08' | b'\x7f' => { if pos > 0 { pos -= 1; let _ = write!(uart, "\x08 \x08"); } }
                _ => { if pos < buffer.len() { buffer[pos] = key; pos += 1; let _ = write!(uart, "{}", key as char); } }
            }
        }
        unsafe { core::arch::asm!("wfi"); }
    }
}

#[alloc_error_handler] fn alloc_error(_l: Layout) -> ! { loop {} }
#[panic_handler] fn panic(_i: &PanicInfo) -> ! { loop {} }

