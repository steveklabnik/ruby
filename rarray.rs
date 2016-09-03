#![no_std]
#![feature(lang_items)]

#![feature(libc)]
extern crate libc;

pub type Value = libc::uintptr_t;

#[allow(non_upper_case_globals)]
static Qnil: libc::uintptr_t = 0x08 as libc::uintptr_t;

use core::slice;

#[no_mangle]
pub unsafe extern fn rb_mem_clear(mem: *mut Value, size: libc::c_long) {
    for x in slice::from_raw_parts_mut(mem, size as usize) {
        *x = Qnil;
    }
}

use core::fmt;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_: fmt::Arguments,
                               _: &'static str,
                               _: u32) -> ! {
    loop {}
}
