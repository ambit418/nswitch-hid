//! Analog stick axis definitions for the Nintendo Switch controller.

/// Analog stick with X and Y axes.
///
/// Each axis ranges from 0 to 255, where 128 is the center (neutral) position.
///
/// - X axis: 0 = left, 128 = center, 255 = right
/// - Y axis: 0 = up, 128 = center, 255 = down
///
/// # Examples
///
/// ```
/// use nswitch_hid::Stick;
///
/// let stick = Stick::CENTER;
/// assert_eq!(stick.x, 128);
/// assert_eq!(stick.y, 128);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stick {
    /// Horizontal axis. 0 = left, 128 = center, 255 = right.
    pub x: u8,
    /// Vertical axis. 0 = up, 128 = center, 255 = down.
    pub y: u8,
}

impl Stick {
    /// Center (neutral) position.
    pub const CENTER: Self = Self { x: 128, y: 128 };
}
