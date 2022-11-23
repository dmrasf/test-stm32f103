#![allow(non_snake_case)]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use stm32f1xx_hal::{
    pac::{self, interrupt, Interrupt},
    prelude::*,
    rcc::Clocks,
    timer::{CounterMs, Event, TimerExt},
};

pub fn tim2_init(tim2: pac::TIM2, clocks: &Clocks) {
    let mut timer = tim2.counter_ms(clocks);
    timer.start(200.millis()).unwrap();
    timer.listen(Event::Update);
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }
}

static G_TIM: Mutex<RefCell<Option<CounterMs<pac::TIM2>>>> = Mutex::new(RefCell::new(None));
pub static G_TIMER_FLAG: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterMs<pac::TIM2>> = None;

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| G_TIM.borrow(cs).replace(None).unwrap())
    });

    cortex_m::interrupt::free(|cs| {
        *G_TIMER_FLAG.borrow(cs).borrow_mut() = true;
    });

    let _ = tim.wait();
}
