//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for data_byte in s.as_bytes().iter() {

            // wait until it's safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}

            // Send a single character
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(*data_byte)));

        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // I'm getting garbage output most of the time, I think the M[1:0] bits that control
    // data length are not getting intialiazed properly (they are set for 9 bit data length)
    // which my FTDI board does not seem to support.
    // This block is me attempting to set the number of data bits to 7 (or 8)
    unsafe {
        // Reset TE before disabling usart
        usart1.cr1.write(|w| w.te().clear_bit());
        // Wait for TC bit in ISR to be set, indicating the transmitter is disabled
        while usart1.isr.read().tc().bit_is_clear() {}

        // Disable usart so that the configuration can be changed
        usart1.cr1.write(|w| w.ue().clear_bit());

        // 7 data bits => M1 (bit 28) = 1, M0 (bit 12) = 0
        usart1.cr1.modify(|r, w| w.bits( (r.bits() & (1 << 28)) & !(1 << 12)) );

        // 8 data bits => M1 (bit 28) = 0, M0 (bit 12) = 0
        // usart1.cr1.modify(|r, w| w.bits(r.bits() & !((1 << 28) & (1 << 12)) ));

        // Re-enable usart
        usart1.cr1.write(|w| w.ue().set_bit());
    }

    let mut serial = SerialPort { usart1 };

    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}
