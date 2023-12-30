//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use cortex_m_rt::entry;
use ra4m1 as pac;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    // let mut pac = unsafe { pac::Peripherals::steal() };
    let core = pac::CorePeripherals::take().unwrap();
    let led_pin_bit = 1 << 11;
    //1000_0000_0000
    let led_port_dir = pac.PORT1.pdr();
    let led_port_out = pac.PORT1.podr();
    // LED pin to Output.
    led_port_dir.write(|w| unsafe { w.pdr().bits(led_pin_bit) });
    let mut delay = cortex_m::delay::Delay::new(core.SYST, 480000);
    loop {
        info!("on!");
        led_port_out.modify(|_, w| unsafe { w.podr().bits(led_pin_bit) });
        delay.delay_ms(1000);
        info!("off!");
        led_port_out.modify(|r, w| unsafe { w.podr().bits(r.bits() & !led_pin_bit) });
        // led_pin.set_low().unwrap();
        delay.delay_ms(1000);
    }
}

// End of file
