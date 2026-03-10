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
