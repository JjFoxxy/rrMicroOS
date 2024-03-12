#![no_std]
#![no_main]

pub mod timer;
pub mod uart;

use core::arch::asm;
use core::panic::PanicInfo;
use timer::Timer;
use uart::Uart;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub extern "C" fn _start() -> ! {
    // Setup the exceptions handler address and enable interrupts
    unsafe {
        asm!("la a0, trap_entry", "csrw mtvec, a0");
        asm!("li a0, 0x8", "csrw mie, a0");
    }

    let uart = Uart::new();
    let timer = Timer::new(0xf0002800);
    timer.set_periodic(10);

    uart.puts(b"Starting!\n");

    unsafe {
        // Test that exceptions work and control is transfered to the exception handler
        asm!("j 2");
    }
    loop {}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub extern "C" fn trap_entry() -> ! {
    let uart = Uart::new();
    uart.puts(b"IT S A Trap!\n");
    loop {}
}
