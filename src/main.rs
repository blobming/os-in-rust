#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] //In compiler construction, name mangling (also called name decoration) is a technique used to solve various problems caused by the need to resolve unique names for programming entities in many modern programming languages.
//extern "C" -> tell the compiler that it should use the C calling convention for this function (instead of the unspecified Rust calling convention)
pub extern "C" fn _start() -> ! { //default entrypoint
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}