#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use ra4m1 as pac;

#[entry]
fn main() -> ! {
    info!("Program start");
    let pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let led_pin_bit = 1 << 11;
    let led_port_dir = pac.PORT1.pdr();
    let led_port_out = pac.PORT1.podr();
    // LED pin to Output.
    led_port_dir.write(|w| unsafe { w.pdr().bits(led_pin_bit) });
    // Use systick as our delay source
    let mut delay = cortex_m::delay::Delay::new(core.SYST, 480000);
    loop {
        info!("on!");
        led_port_out.modify(|r, w| unsafe { w.podr().bits(r.bits() | led_pin_bit) });
        delay.delay_ms(1000);
        info!("off!");
        led_port_out.modify(|r, w| unsafe { w.podr().bits(r.bits() & !led_pin_bit) });
        delay.delay_ms(1000);
    }
}

// End of file
