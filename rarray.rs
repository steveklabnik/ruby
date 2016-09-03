#![no_std]
#![feature(lang_items)]

#![feature(libc)]
extern crate libc;

pub type Value = libc::uintptr_t;

#[allow(non_upper_case_globals)]
static Qnil: libc::uintptr_t = 0x08 as libc::uintptr_t;

#[no_mangle]
pub unsafe extern fn rb_mem_clear(mut mem: *mut Value, mut size: libc::c_long) {
    while size != 0 {
        size -= 1;
        *mem = Qnil;

        mem = mem.offset(1);
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
