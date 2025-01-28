use bitflags::bitflags;

/// Also known as P1 (some relic from the NES supporting multiple controllers)
///
/// It's a virtual register that converts the button states to a byte
///
/// NOTE: Joypad register has active as low (0)
pub struct Joypad {
    /// CPU Pin 14: Select direction buttons
    select_dpad: bool,
    /// CPU Pin 15: Select buttons
    select_button: bool,

    dpad: ButtonState,
    buttons: ButtonState,
}
impl Default for Joypad {
    fn default() -> Self {
        Self {
            select_dpad: false,
            select_button: false,

            dpad: ButtonState::empty(),
            buttons: ButtonState::empty(),
        }
    }
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

impl Joypad {
    const SELECT_DPAD_BIT_IDX: u8 = 4;
    const SELECT_BUTTON_BIT_IDX: u8 = 5;

    /// NOTE: The Game Boy expects bits to be inversed.
    /// If a bit is set to `0`, it means it's active
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

    /// Write selection mode to the register
    ///
    /// NOTE: this register is active low: `0` means `true`
    pub fn write_register(&mut self, byte: u8) {
        /// A mask of all writeable bits
        ///
        /// Only the `select_<dpad|buttons>` bits are writable
        const WRITEABLE_MASK: u8 = 0b0011_0000;

        // Invert the given `byte` because the Game Boy considers bits set as `0` to be `true`
        let truncated = !byte & WRITEABLE_MASK;

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

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::*;

    macro_rules! assert_u8_eq {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!(
                    "assertion `left == right` failed\n  left: 0b{:04b}_{:04b}\n right: 0b{:04b}_{:04b}",
                    ($left & 0b1111_0000) >> 4,
                    ($left & 0b0000_1111),
                    ($right & 0b1111_0000) >> 4,
                    ($right & 0b0000_1111),
                );
            }
        };
    }

    const WRITE_DPAD_SELECT: u8 = 0b1110_1111;
    const WRITE_BUTTON_SELECT: u8 = 0b1101_1111;

    #[test]
    fn initial_state() {
        let mut joyp = Joypad::default();

        // READ (initial selection modes OFF)
        {
            assert_eq!(joyp.select_dpad, false);
            assert_eq!(joyp.select_button, false);
            assert_u8_eq!(joyp.read_register(), 0b1111_1111);
        }

        // READ with selection modes ON
        {
            joyp.write_register(0b1100_0000);
            assert_eq!(joyp.select_dpad, true);
            assert_eq!(joyp.select_button, true);
            assert_u8_eq!(joyp.read_register(), 0b1100_1111);
        }

        // READ with selection modes OFF
        {
            joyp.write_register(0b1111_0000);
            assert_eq!(joyp.select_dpad, false);
            assert_eq!(joyp.select_button, false);
            assert_u8_eq!(joyp.read_register(), 0b1111_1111);
        }
    }

    #[test]
    fn readonly_bits() {
        let mut joyp = Joypad::default();

        joyp.write_register(0b0000_0000); // Means "everything active"
        assert_u8_eq!(joyp.read_register(), 0b1100_1111);

        joyp.write_register(0b0000_1111); // Means "everything active"
        assert_u8_eq!(joyp.read_register(), 0b1100_1111);
    }

    #[test]
    fn selection_modes() {
        // Select DPAD
        {
            let mut joyp = Joypad::default();
            joyp.write_register(0b1110_1111);
            assert_eq!(joyp.select_dpad, true);
            assert_eq!(joyp.select_button, false);
        }

        // Select Buttons
        {
            let mut joyp = Joypad::default();
            joyp.write_register(0b1101_1111);
            assert_eq!(joyp.select_dpad, false);
            assert_eq!(joyp.select_button, true);
        }

        // Select both
        {
            let mut joyp = Joypad::default();
            joyp.write_register(0b1100_1111);
            assert_eq!(joyp.select_dpad, true);
            assert_eq!(joyp.select_button, true);
        }
    }

    #[test]
    fn button_A() {
        const KEY: JoypadButton = JoypadButton::A;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0001;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_B() {
        const KEY: JoypadButton = JoypadButton::B;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0010;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_SELECT() {
        const KEY: JoypadButton = JoypadButton::Select;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0100;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_START() {
        const KEY: JoypadButton = JoypadButton::Start;
        const KEY_ACTIVE_MASK: u8 = !0b0000_1000;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }

    #[test]
    fn button_RIGHT() {
        const KEY: JoypadButton = JoypadButton::Right;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0001;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_LEFT() {
        const KEY: JoypadButton = JoypadButton::Left;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0010;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_UP() {
        const KEY: JoypadButton = JoypadButton::Up;
        const KEY_ACTIVE_MASK: u8 = !0b0000_0100;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
    #[test]
    fn button_DOWN() {
        const KEY: JoypadButton = JoypadButton::Down;
        const KEY_ACTIVE_MASK: u8 = !0b0000_1000;

        let mut joyp = Joypad::default();
        joyp.key_down(KEY);

        assert_u8_eq!(joyp.read_register(), 0b1111_1111);

        joyp.write_register(WRITE_DPAD_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_DPAD_SELECT & KEY_ACTIVE_MASK);

        joyp.write_register(WRITE_BUTTON_SELECT);
        assert_u8_eq!(joyp.read_register(), WRITE_BUTTON_SELECT);

        joyp.write_register(WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT);
        assert_u8_eq!(
            joyp.read_register(),
            WRITE_BUTTON_SELECT & WRITE_DPAD_SELECT & KEY_ACTIVE_MASK
        );
    }
}
