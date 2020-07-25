#![no_std]
#![feature(alloc_prelude)]

#[macro_use]
extern crate alloc;
use core::panic::PanicInfo;
use win_kmd_alloc::KernelAlloc;
use winapi::km::wdm;
use winapi::shared::ntdef;
use core::ptr;
use obfstr::wide;

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
pub extern "system" fn DriverEntry(
    driver_object: wdm::PDRIVER_OBJECT ,
    _registry_path_ptr: ntdef::PUNICODE_STRING
) -> u32 {
    log!("Driver Entry called.");

    unsafe {
        let mut driver_name : ntdef::UNICODE_STRING = core::mem::zeroed();
        let mut dos_driver_name : ntdef::UNICODE_STRING = core::mem::zeroed();
        let driver_name_str = wide!("\\Device\\MyHypervisor");
        let dos_driver_name_str=  wide!("\\Device\\MyHypervisor");

        wdm::RtlInitUnicodeString(&mut driver_name, driver_name_str.as_ptr());
	    wdm::RtlInitUnicodeString(&mut dos_driver_name, dos_driver_name_str.as_ptr());

        let mut device_object = ptr::null_mut();
        let status = wdm::IoCreateDevice(
            driver_object,
            0,
            &driver_name,
            wdm::DEVICE_TYPE::FILE_DEVICE_UNKNOWN,
            wdm::DEVICE_CHARACTERISTICS::FILE_DEVICE_SECURE_OPEN as u32,
            ntdef::FALSE,
            &mut device_object);
        
        log!("IoCreateDevice status:{}", status);

        (*driver_object).DriverUnload = Some(DriverUnload);

    }
    log!("DriverEntry Finish");
    0 /* STATUS_SUCCESS */
}

#[no_mangle]
pub extern "system" fn DriverUnload(driver_object: &mut wdm::DRIVER_OBJECT) {
    log!("Driver Unload start!");
    unsafe {
        wdm::IoDeleteDevice(driver_object.DeviceObject);
    }
    log!("Driver Unload end!");
}