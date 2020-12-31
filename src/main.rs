#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal;

use crate::stm32f4xx_hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    let peripherals = stm32::Peripherals::take().unwrap();
    
    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    
    // use pc13 for stm32f411
    let gpioc = peripherals.GPIOC.split();


    let mut led = gpioc.pc13.into_push_pull_output();
 
    // Reset and clock control
    let rcc = peripherals.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();

    // System Timer
    let mut delay = stm32f4xx_hal::delay::Delay::new(core_peripherals.SYST, clocks);

    // let freq = 1_000_000;
    let mut pwm: u32 = 1;
    let mut direction = true;

    loop {
        if pwm == 0 || pwm == 100 {
            direction = !direction
        };
        
        if direction {
            pwm = pwm + 1;
        } else {
            pwm = pwm - 1;
        }
        
        for i in 1..100_u64 {
            led.set_high().unwrap();
            delay.delay_us(100 - pwm);
            led.set_low().unwrap();
            delay.delay_us(pwm);
        }
    }
    
    

}
