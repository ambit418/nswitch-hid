//! D-pad (hat switch) definitions for the Nintendo Switch controller.

/// D-pad direction sent as part of the HID input report.
///
/// Only one direction can be active at a time. Use [`Hat::Neutral`]
/// when no direction is pressed.
///
/// # Examples
///
/// ```
/// use nswitch_hid::Hat;
///
/// let direction = Hat::Up;
/// assert_eq!(direction as u8, 0x00);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Hat {
    /// Up.
    Up = 0x00,
    /// Up-right.
    UpRight = 0x01,
    /// Right.
    Right = 0x02,
    /// Down-right.
    DownRight = 0x03,
    /// Down.
    Down = 0x04,
    /// Down-left.
    DownLeft = 0x05,
    /// Left.
    Left = 0x06,
    /// Up-left.
    UpLeft = 0x07,
    /// No direction pressed.
    Neutral = 0x0F,
}
