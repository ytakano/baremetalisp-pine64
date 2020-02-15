#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![no_std]

mod parser;
mod aarch64;

use core::panic::PanicInfo;

#[no_mangle]
fn func() -> () {
    ()
}

#[no_mangle]
pub fn entry() -> ! {
    unsafe {
        asm!(
            "bl func;"
            : : : :
        );
    }

    aarch64::init();

    parser::test(10);

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}