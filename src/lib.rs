//! Nintendo Switch Pro Controller (HORI Pokken Controller) USB HID emulation.
//!
//! A `no_std` library for creating USB HID reports compatible with
//! the Nintendo Switch.

#![no_std]
#![warn(missing_docs)]

mod button;

pub use button::Buttons;
