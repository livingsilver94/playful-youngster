enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

pub struct Keypad {
    value: u8,
    interrupt_raised: bool,
}

impl Keypad {
    pub fn set_pressed(&mut self, button: Button, pressed: bool) {
        if pressed {
            self.interrupt_raised = true;
        }
    }
}
