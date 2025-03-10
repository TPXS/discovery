#![deny(unsafe_code)]
#![no_main]
#![no_std]


use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) : (Delay, LedArray) = aux5::init();

    let ms = 25_u8;
    
    loop {
        for current in 0..8 {
            let next = (current + 1) % 4;

            leds[next].on().ok();
            leds[(next + 4) % 8].on().ok();
            delay.delay_ms(2 * ms);

            leds[current].off().ok();
            leds[(current + 4) % 8].off().ok();
            delay.delay_ms(ms);
        }
    }

}
