#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::string::String;
use alloc::format;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

// Lấy địa chỉ từ file Linker
extern "C" {
    static _heap_start: u8;
}

struct BumpingAllocator {
    next: AtomicUsize,
}

unsafe impl GlobalAlloc for BumpingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Lấy vị trí hiện tại và cộng thêm kích thước cần cấp phát
        let start = self.next.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |prev| {
            let aligned = (prev + layout.align() - 1) & !(layout.align() - 1);
            Some(aligned + layout.size())
        }).unwrap();
        
        let ptr = ((start + layout.align() - 1) & !(layout.align() - 1)) as *mut u8;
        ptr
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bumping allocator không giải phóng bộ nhớ từng mẩu nhỏ
    }
}

#[global_allocator]
static ALLOCATOR: BumpingAllocator = BumpingAllocator {
    next: AtomicUsize::new(0x2000_2000), // Bắt đầu từ vùng RAM trống
};

// --- Phần Driver UART cũ của bro ---
// (Giữ nguyên cấu trúc Uart và Write đã chạy tốt)

#[no_mangle]
pub extern "C" fn _reset_handler() -> ! {
    let mut uart = Uart { base_ptr: 0x4000_c000 as *mut u32 };

    // TEST THỬ SỨC MẠNH CỦA ALLOC
    let mut welcome_msg = String::from("\x1b[35m[OXID-ALLOC]\x1b[0m ");
    welcome_msg.push_str("Memory Manager is LIVE!\n");
    let _ = write!(uart, "{}", welcome_msg);

    let system_info = format!("\x1b[33m[INFO]\x1b[0m RAM Start: 0x20000000, Heap Start: 0x{:x}\n", 0x20002000);
    let _ = write!(uart, "{}", system_info);

    let _ = write!(uart, "> ");

    loop {
        let key = uart.getc();
        if key != 0 {
            // Echo lại bằng String cho "sang"
            let msg = format!("\x1b[36m{}\x1b[0m", key as char);
            let _ = write!(uart, "{}", msg);
        }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! { loop {} }
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
