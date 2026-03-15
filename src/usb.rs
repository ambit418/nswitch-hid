//! USB HID device integration with `embassy-usb`.
//!
//! This module provides a helper to configure a USB HID device
//! that is recognized by the Nintendo Switch as a Pro Controller
//! (HORI Pokken Controller).
//!
//! Requires the `embassy` feature.

use embassy_usb::{
    class::hid::{Config as HidConfig, HidReaderWriter, State as HidState},
    driver::Driver,
    Builder,
};

use crate::descriptor::HID_REPORT_DESCRIPTOR;

/// Size of the HID input report in bytes.
const INPUT_REPORT_SIZE: usize = 8;

/// Size of the HID output report in bytes (not used, but required by the API).
const OUTPUT_REPORT_SIZE: usize = 0;

/// Creates a [`HidReaderWriter`] configured as a Nintendo Switch controller.
///
/// This wires up the HID Report Descriptor so the Switch recognizes
/// the device as a HORI Pokken Controller.
///
/// # Arguments
///
/// * `builder` - The embassy-usb device builder.
/// * `state` - HID class state storage. Must outlive the returned `HidReaderWriter`.
///
/// # Examples
///
/// ```ignore
/// let mut state = embassy_usb::class::hid::State::new();
/// let hid = nswitch_hid::create_hid(&mut builder, &mut state);
/// ```
pub fn create_hid<'d, D: Driver<'d>>(
    builder: &mut Builder<'d, D>,
    state: &'d mut HidState<'d>,
) -> HidReaderWriter<'d, D, INPUT_REPORT_SIZE, OUTPUT_REPORT_SIZE> {
    let hid_config = HidConfig {
        report_descriptor: HID_REPORT_DESCRIPTOR,
        request_handler: None,
        poll_ms: 5,
        max_packet_size: 64,
    };

    HidReaderWriter::new(builder, state, hid_config)
}
