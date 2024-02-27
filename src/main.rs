#![no_std]
#![no_main]

pub mod uart;

use core::panic::PanicInfo;
use uart::Uart;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub extern "C" fn _start() -> ! {
    let uart = Uart::new();
    uart.puts(b"YOLO!\n");
    loop {}
}
