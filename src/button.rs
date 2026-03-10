//! Button definitions for the Nintendo Switch controller.

use bitflags::bitflags;

bitflags! {
    /// Button state represented as a 16-bit bitmask.
    ///
    /// Each bit corresponds to a single button. Multiple buttons can be
    /// combined using the `|` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use nswitch_hid::Buttons;
    ///
    /// let pressed = Buttons::A | Buttons::B;
    /// assert!(pressed.contains(Buttons::A));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Buttons: u16 {
        /// Y button.
        const Y       = 0x0001;
        /// B button.
        const B       = 0x0002;
        /// A button.
        const A       = 0x0004;
        /// X button.
        const X       = 0x0008;
        /// Left shoulder button.
        const L       = 0x0010;
        /// Right shoulder button.
        const R       = 0x0020;
        /// Left trigger button.
        const ZL      = 0x0040;
        /// Right trigger button.
        const ZR      = 0x0080;
        /// Minus (-) button.
        const MINUS   = 0x0100;
        /// Plus (+) button.
        const PLUS    = 0x0200;
        /// Left stick press.
        const L_STICK = 0x0400;
        /// Right stick press.
        const R_STICK = 0x0800;
        /// Home button.
        const HOME    = 0x1000;
        /// Capture (screenshot) button.
        const CAPTURE = 0x2000;
    }
}
