#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[derive(Debug)]
enum NextAction {
    LedOn,
    LedOff,
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let interval = 50_u16;

    let mut led_index = 0_usize;

    let mut next = NextAction::LedOn;

    // Initial state has "N" LED on
    leds[0].on();

    // At each step we are either turning the next led on
    // or turning the current one off and advancing "current"
    // to be the next led, which is already on by then.
    loop {
        match next {
            NextAction::LedOn => {
                leds[(led_index + 1) % 8].on();
                next = NextAction::LedOff;
            }
            NextAction::LedOff => {
                leds[led_index].off();
                led_index = (led_index + 1) % 8;
                next = NextAction::LedOn;
            }
        }
        delay.delay_ms(interval);
    }
}
