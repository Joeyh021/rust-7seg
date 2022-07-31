#![no_std]

mod display;
mod serial;

use display::Display;
use embedded_hal::blocking::delay::*;
use riscv::delay::McycleDelay;
use serial::Serial;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut serial = Serial::take().unwrap();
    let mut delay = McycleDelay::new(50_000_000);
    let mut display = Display::take().unwrap();
    display.clear();

    serial.print("\n");
    serial.println("Hello from Rust!");
    delay.delay_ms(100_u8);

    for i in (0..8).rev() {
        display.set(i, i as u32);
    }

    loop {
        for i in (1..8).rev() {
            display.set(i, display.get(i - 1));
        }
        display.set(0, (display.get(0) + 1) % 10);

        for i in 0..8 {
            serial.write_byte(display.get(i) as u8 + 48);
            serial.print(" ");
        }
        serial.println("");

        delay.delay_ms(1000);
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
