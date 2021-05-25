#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// The main function doesn't make sense without
// an underlying runtime that calls it.
// #[no_mangle] attribute disables the name mangling
// to ensure that the Rust compiler outputs
// a function with the name _start, instead of generating
// some cryptic symbol to give every function an unique name.
// This is required because we need to tell the name of the
// entry point function to the linker.
// ! return type means that the function is diverging, i.e. not
// allowed to ever return.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
