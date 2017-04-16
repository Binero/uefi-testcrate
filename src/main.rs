#![feature(lang_items)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(start)]

extern crate rlibc;

use core::intrinsics;

#[no_mangle]
pub extern fn efi_main() -> isize {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}


#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

