#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    let data = b"The quick brown fox jumps over the lazy dog.";

    for character in data.iter() {

        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {}

        // Send a single character
        usart1.tdr.write(|w| w.tdr().bits(u16::from(*character)));

    }

    loop {}
}
