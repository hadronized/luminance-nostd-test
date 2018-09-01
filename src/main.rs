#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]

extern crate libc;

mod gl_context;
use core::panic::PanicInfo;

// Entry point for this program.
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn rust_begin_panic(_: &PanicInfo) -> ! {
  loop {}
}
