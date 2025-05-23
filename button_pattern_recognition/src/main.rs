#![no_std]
#![no_main]

mod fmt;
mod button_state;
mod detection;
mod output;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{Pull}};

use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);

    _spawner.spawn(detection::button_detect(button)).unwrap();
    info!("Button detection started");
    _spawner.spawn(button_state::button_event_control()).unwrap();
    info!("Button event control started");
}
