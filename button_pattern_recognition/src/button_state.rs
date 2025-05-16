enum ButtonState {
    Init,
    IdleOne,
    PressedOne {pressed_at: embassy_time::Instant},
    IdleTwo {released_at: embassy_time::Instant},
    PressedTwo {pressed_at: embassy_time::Instant},
    End,
}

#[derive(Copy, Clone, Default)]
pub struct ButtonEvent {
    pub pressed_duration_one: embassy_time::Duration,
    pub wait_duration: embassy_time::Duration,
    pub pressed_duration_two: embassy_time::Duration,
}

fn button_event_control() -> ButtonEvent {
    let mut event: ButtonEvent = ButtonEvent::default();
    let mut state: ButtonState = ButtonState::Init;
    loop {
        state = tick(state, &mut event);
        if matches!(state, ButtonState::End) {
            // If the state is End, break the loop
            break;
        }
    }
    event
}

fn tick(state: ButtonState, event: &mut ButtonEvent) -> ButtonState {
    let wait_duration = embassy_time::Duration::from_millis(2000);
    // transition to the next state
    match state {
        ButtonState::Init => {
            // Initialize the button event
            event.pressed_duration_one = embassy_time::Duration::from_millis(0);
            event.wait_duration = embassy_time::Duration::from_millis(0);
            event.pressed_duration_two = embassy_time::Duration::from_millis(0);
            ButtonState::IdleOne
        }
        ButtonState::IdleOne => {
            // Wait for the first button press
            if button_pressed() {
                ButtonState::PressedOne { pressed_at: embassy_time::Instant::now() }
            } else {
                ButtonState::IdleOne
            }
        }
        ButtonState::PressedOne { pressed_at } => {
            // Wait for the button to be released
            if !button_pressed() {
                event.pressed_duration_one = embassy_time::Instant::now() - pressed_at;
                ButtonState::IdleTwo { released_at: embassy_time::Instant::now() }
            } else {
                ButtonState::PressedOne { pressed_at }
            }
        }
        ButtonState::IdleTwo { released_at } => {
            // Wait for the second button press
            if button_pressed() {
                event.wait_duration = embassy_time::Instant::now() - released_at;
                ButtonState::PressedTwo { pressed_at: embassy_time::Instant::now() }
            } else if (embassy_time::Instant::now() - released_at) > wait_duration {
                ButtonState::End
            } else {
                ButtonState::IdleTwo { released_at }
            }
        }
        ButtonState::PressedTwo { pressed_at } => {
            // Wait for the button to be released
            if !button_pressed() {
                event.pressed_duration_two = embassy_time::Instant::now() - pressed_at;
                ButtonState::End
            } else {
                ButtonState::PressedTwo { pressed_at }
            }
        }
        ButtonState::End => {
            // End state, do nothing
            ButtonState::End
        }
    }
}

