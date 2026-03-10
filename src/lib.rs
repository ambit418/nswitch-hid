//! Nintendo Switch Pro Controller (HORI Pokken Controller) USB HID emulation.
//!
//! A `no_std` library for creating USB HID reports compatible with
//! the Nintendo Switch.

#![no_std]
#![warn(missing_docs)]

mod button;
mod controller;
mod hat;
mod stick;

pub use button::Buttons;
pub use controller::ControllerState;
pub use hat::Hat;
pub use stick::Stick;
