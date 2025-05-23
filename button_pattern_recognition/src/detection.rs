

use core::sync::atomic::{AtomicBool, Ordering};
use embassy_stm32::exti::ExtiInput;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
pub async fn button_detect(button: ExtiInput<'static>) {
    loop {
        if button.is_high() {
            BUTTON_PRESSED.store(true, Ordering::Relaxed);
        } else if button.is_low() {
            BUTTON_PRESSED.store(false, Ordering::Relaxed);
        }
    }
}

pub fn is_button_pressed() -> bool {
    BUTTON_PRESSED.load(Ordering::Relaxed)
}