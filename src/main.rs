#![allow(dead_code)]
#![no_std]
#![no_main]

mod hardware;
use hardware::tim2::{tim2_init, G_TIMER_FLAG};

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{
    flash, pac,
    prelude::*,
    rcc::{Clocks, Rcc, RccExt},
};

#[panic_handler]
fn my_panic_handler(info: &PanicInfo) -> ! {
    hprintln!("my_panic_handler: {}", info);
    loop {}
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let clocks = systick_init(dp.FLASH.constrain(), dp.RCC.constrain());

    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut _delay = cp.SYST.delay(&clocks);

    tim2_init(dp.TIM2, &clocks);

    loop {
        let flag = cortex_m::interrupt::free(|cs| G_TIMER_FLAG.borrow(cs).replace(false));
        if flag {
            led.toggle();
        }
    }
}

fn systick_init(mut flash: flash::Parts, rcc: Rcc) -> Clocks {
    rcc.cfgr.freeze(&mut flash.acr)
}
