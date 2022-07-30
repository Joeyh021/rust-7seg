#![no_std]

mod display;
mod serial;

use display::Display;
use embedded_hal::blocking::delay::*;
use riscv::delay::McycleDelay;
use serial::Serial;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let serial = Serial::take().unwrap();
    let mut delay = McycleDelay::new(50_000_000);
    let display = Display::take().unwrap();
    display.clear();

    serial.print("\n");
    serial.println("Hello from Rust!");
    delay.delay_ms(100_u8);

    for i in 0x0..=0xf {
        display.set_all(i);
        delay.delay_ms(100_u8);
    }

    delay.delay_ms(100_u8);

    loop {
        delay.delay_ms(100_u8);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial = unsafe { Serial::new() };
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        serial.print("panic occurred:");
        serial.print(s);
        serial.write_byte(b'\n');
    } else {
        serial.println("panic occurred");
    }
    loop {}
}
