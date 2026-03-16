//! Controller state and HID report serialization.

use crate::{Buttons, Hat, Stick};

/// Current state of a Nintendo Switch controller.
///
/// Combines button presses, D-pad direction, and analog stick positions
/// into a single state that can be serialized into an 8-byte HID input report.
///
/// # Examples
///
/// ```
/// use nswitch_hid::{ControllerState, Buttons, Hat};
///
/// let mut state = ControllerState::new();
/// state.buttons = Buttons::A | Buttons::B;
/// state.hat = Hat::Up;
/// let report = state.to_report();
/// assert_eq!(report.len(), 8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerState {
    /// Button state.
    pub buttons: Buttons,
    /// D-pad direction.
    pub hat: Hat,
    /// Left analog stick.
    pub left_stick: Stick,
    /// Right analog stick.
    pub right_stick: Stick,
}

impl ControllerState {
    /// Creates a new state with all inputs at neutral position.
    pub const fn new() -> Self {
        Self {
            buttons: Buttons::empty(),
            hat: Hat::Neutral,
            left_stick: Stick::CENTER,
            right_stick: Stick::CENTER,
        }
    }

    /// Serializes the controller state into an 8-byte HID input report.
    ///
    /// # Report layout
    ///
    /// | Byte | Content                |
    /// |------|------------------------|
    /// | 0-1  | Buttons (u16 LE)       |
    /// | 2    | Hat switch             |
    /// | 3    | Left stick X           |
    /// | 4    | Left stick Y           |
    /// | 5    | Right stick X          |
    /// | 6    | Right stick Y          |
    /// | 7    | Vendor specific (0x00) |
    pub fn to_report(&self) -> [u8; 8] {
        let buttons = self.buttons.bits().to_le_bytes();
        [
            buttons[0],
            buttons[1],
            self.hat as u8,
            self.left_stick.x,
            self.left_stick.y,
            self.right_stick.x,
            self.right_stick.y,
            0x00,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // All inputs at neutral: no buttons, hat=0x0F, sticks centered at 128,
    // vendor byte = 0x00.
    #[test]
    fn neutral_report() {
        let state = ControllerState::new();
        assert_eq!(
            state.to_report(),
            [0x00, 0x00, 0x0F, 128, 128, 128, 128, 0x00]
        );
    }

    // A = bit 2 (0x0004) lands in the low byte of the LE u16.
    #[test]
    fn single_button() {
        let mut state = ControllerState::new();
        state.buttons = Buttons::A;
        let report = state.to_report();
        assert_eq!(report[0], 0x04);
        assert_eq!(report[1], 0x00);
    }

    // L (0x0010) | R (0x0020) = 0x0030, all in the low byte.
    #[test]
    fn combined_buttons() {
        let mut state = ControllerState::new();
        state.buttons = Buttons::L | Buttons::R;
        let report = state.to_report();
        assert_eq!(report[0], 0x30);
        assert_eq!(report[1], 0x00);
    }

    // HOME = 0x1000 — the high byte must carry the value in LE order.
    #[test]
    fn high_byte_button() {
        let mut state = ControllerState::new();
        state.buttons = Buttons::HOME;
        let report = state.to_report();
        assert_eq!(report[0], 0x00);
        assert_eq!(report[1], 0x10);
    }

    // Hat::Up = 0x00, placed at byte index 2.
    #[test]
    fn hat_direction() {
        let mut state = ControllerState::new();
        state.hat = Hat::Up;
        assert_eq!(state.to_report()[2], 0x00);
    }

    // Stick axes map to bytes 3-6: LX, LY, RX, RY.
    #[test]
    fn stick_axes() {
        let mut state = ControllerState::new();
        state.left_stick = Stick { x: 0, y: 255 };
        state.right_stick = Stick { x: 255, y: 0 };
        let report = state.to_report();
        assert_eq!(report[3], 0);
        assert_eq!(report[4], 255);
        assert_eq!(report[5], 255);
        assert_eq!(report[6], 0);
    }

    // Byte 7 is always 0x00 regardless of other fields.
    #[test]
    fn vendor_byte_always_zero() {
        let mut state = ControllerState::new();
        state.buttons = Buttons::all();
        state.hat = Hat::Down;
        state.left_stick = Stick { x: 255, y: 255 };
        assert_eq!(state.to_report()[7], 0x00);
    }
}
