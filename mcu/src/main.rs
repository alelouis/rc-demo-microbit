#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::{Saadc, Timer};
use microbit::hal::saadc::SaadcConfig;
use microbit::hal::pac::Peripherals;
use microbit::hal::gpio;
use heapless::Vec;


use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

mod serial_setup;
use serial_setup::UartePort;

use microbit::hal::gpio::{p0, p1, Floating, Input, Output, Pin, PushPull, Level};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Peripherals::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let gpios = gpio::p0::Parts::new(board.P0);
    let gpios_1 = gpio::p1::Parts::new(board.P1);
    let pins = uarte::Pins {
        txd: gpios.p0_06.into_push_pull_output(Level::Low).degrade(),
        rxd: gpios_1.p1_08.into_floating_input().degrade(),
        cts: None,
        rts: None,
    };

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            pins,
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    let mut buffer: Vec<u8, 32> = Vec::new();

    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut saadc_pin = gpios.p0_02; // the pin your analog device is connected to
    let _saadc_result = saadc.read(&mut saadc_pin);
    nb::block!(serial.flush()).unwrap();
    timer.delay_ms(10000_u32);

    loop {
        let read_value = saadc.read(&mut saadc_pin).unwrap();
        rprintln!("read value i16: {}", read_value);
        let bytes = read_value.to_be_bytes();
        for byte in bytes {
            rprintln!("{}", byte);
            nb::block!(serial.write(byte)).unwrap();
        }
        timer.delay_ms(20_u32);
        nb::block!(serial.flush()).unwrap()

    }
}
