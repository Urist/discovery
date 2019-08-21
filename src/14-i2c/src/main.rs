#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        i2c1.cr2.write(|w| {
            w.nbytes().bits(1); // write 1 byte
            w.autoend().clear_bit(); // don't end since we want to read afterwards
            w.sadd1().bits(MAGNETOMETER); // slave address
            w.rd_wrn().clear_bit(); // WRITE mode
            w.start().set_bit() // send START
        });

        // Wait for signal that we can send
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| w.txdata().bits(IRA_REG_M));

        // Wait for the byte to send
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Send START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|r, w| {
            w.nbytes().bits(1); // read 1 byte
            w.autoend().set_bit(); // auto-send END once 1 byte is read
            w.rd_wrn().set_bit(); // READ mode
            w.start().set_bit() // send START
        });

        // Wait until there is data to read
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // Receive the contents of the register
        i2c1.rxdr.read().rxdata().bits()

        // Broadcast STOP
        // (we set auto-stop, so no need to send a STOP)
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
