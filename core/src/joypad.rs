use bitflags::bitflags;

/// Also known as P1 (some relic from the NES supporting multiple controllers)
///
/// It's a virtual register that converts the button states to a byte
pub struct Joypad {
    /// CPU Pin 14: Select direction buttons
    select_dpad: bool,
    /// CPU Pin 15: Select buttons
    select_button: bool,

    dpad: ButtonState,
    buttons: ButtonState,
}

bitflags! {
    struct ButtonState: u8 {
        /// CPU Pin 10: →, A
        const P10 = 1 << 0;
        /// CPU Pin 11: ←, B
        const P11 = 1 << 1;
        /// CPU Pin 12: ↑, Select
        const P12 = 1 << 2;
        /// CPU Pin 13: ↓, Start
        const P13 = 1 << 3;
    }
}

/// NOTE: If a bit is set to `0`, it means it's active
impl Joypad {
    pub const SELECT_DPAD_BIT_IDX: u8 = 4;
    pub const SELECT_BUTTON_BIT_IDX: u8 = 5;

    /// A mask of all writeable bits
    ///
    /// Only the `select_<dpad|buttons>` bits are writable
    pub const WRITEABLE: u8 = 0b0011_0000;
}

impl Joypad {
    /// NOTE: The Game Boy expects bits to be inversed.
    /// If a bit/ is set to `0`, it means it's active
    pub fn read_register(&self) -> u8 {
        #[rustfmt::skip]
        let state_nibble: u8 = {
            let button_nibble = if self.select_button { self.buttons.bits() } else { 0b0000 };
            let dpad_nibble = if self.select_dpad { self.dpad.bits() } else { 0b0000 };

            0b0000_1111 & (button_nibble | dpad_nibble)
        };

        let inverted_register = 0b0000_0000
            | (self.select_button as u8) << Self::SELECT_BUTTON_BIT_IDX
            | (self.select_dpad as u8) << Self::SELECT_DPAD_BIT_IDX
            | state_nibble;

        // Invert the bits for the CPU so that `0` equals `true`
        return !inverted_register;
    }

    pub fn write_register(&mut self, byte: u8) {
        // Invert the given `byte` because the Game Boy considers bits set as `0` to be `true`
        let truncated = !byte & Self::WRITEABLE;

        self.select_dpad = truncated & (1 << Self::SELECT_DPAD_BIT_IDX) != 0;
        self.select_button = truncated & (1 << Self::SELECT_BUTTON_BIT_IDX) != 0;
    }

    pub fn key_down(&mut self, button: JoypadButton) -> &mut Self {
        return self.set_key(button, true);
    }
    pub fn key_up(&mut self, button: JoypadButton) -> &mut Self {
        return self.set_key(button, false);
    }
    #[inline]
    fn set_key(&mut self, button: JoypadButton, pressed: bool) -> &mut Self {
        use JoypadButton::*;

        match button {
            A | B | Select | Start => self
                .buttons
                .set(ButtonState::from_bits_retain(button as u8), pressed),

            Right | Left | Up | Down => self.dpad.set(
                ButtonState::from_bits_retain(button as u8 >> JoypadButton::DPAD_BIT_OFFSET),
                pressed,
            ),
        };

        return self;
    }
}

#[repr(u8)]
/// Enum representing all joypad buttons
///
/// Every button is mapped to a specific bit index in a byte.
///  - `0b0000_1111`: Buttons
///  - `0b1111_0000`: DPAD
pub enum JoypadButton {
    // Buttons
    A = ButtonState::P10.bits(),
    B = ButtonState::P11.bits(),
    Select = ButtonState::P12.bits(),
    Start = ButtonState::P13.bits(),

    // DPAD
    Right = ButtonState::P10.bits() << Self::DPAD_BIT_OFFSET,
    Left = ButtonState::P11.bits() << Self::DPAD_BIT_OFFSET,
    Up = ButtonState::P12.bits() << Self::DPAD_BIT_OFFSET,
    Down = ButtonState::P13.bits() << Self::DPAD_BIT_OFFSET,
}
impl JoypadButton {
    const DPAD_BIT_OFFSET: u8 = 4;
}
