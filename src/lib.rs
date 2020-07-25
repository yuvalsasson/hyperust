#![no_std]
#![feature(alloc_prelude)]

#[macro_use]
extern crate alloc;
use core::panic::PanicInfo;
use win_kmd_alloc::KernelAlloc;

mod logger;

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[used]
#[no_mangle]
static _fltused: i32 = 0;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn DriverEntry() -> u32 {
    log!("Hello world3{}", 12); 
    0 /* STATUS_SUCCESS */
}