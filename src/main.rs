#![no_std]
#![no_main]
#![feature(alloc_error_handler)] // Cho phép xử lý khi hết bộ nhớ

extern crate alloc;
use alloc::string::String;
use alloc::format;

use core::alloc::{GlobalAlloc, Layout};

struct SimpleAllocator;

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        // Đây là nơi bro sẽ thực hiện logic cấp phát
        // Tạm thời ta trỏ vào một vùng RAM trống cố định
        0x2000_1000 as *mut u8 
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! { loop {} }
