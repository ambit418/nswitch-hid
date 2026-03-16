//! HID Report Descriptor for the HORI Pokken Controller.
//!
//! This descriptor tells the USB host (Nintendo Switch) how to interpret
//! the 8-byte input reports sent by the controller.

/// HID Report Descriptor (86 bytes).
///
/// Describes the report layout:
///
/// **Input (8 bytes, device → host):**
/// - 16 buttons (2 bytes)
/// - 1 hat switch (4 bits + 4 bits padding)
/// - 4 analog axes: LX, LY, RX, RY (1 byte each)
/// - 1 vendor-specific byte
///
/// **Output (8 bytes, host → device):**
/// - 8 vendor-specific bytes
///
/// Based on the HORI Pokken Controller descriptor, compatible with
/// Nintendo Switch v3.0.0+.
pub const HID_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x05,       // Usage (Game Pad)
    0xA1, 0x01,       // Collection (Application)
    // --- Buttons (16 bits) ---
    0x15, 0x00,       //   Logical Minimum (0)
    0x25, 0x01,       //   Logical Maximum (1)
    0x35, 0x00,       //   Physical Minimum (0)
    0x45, 0x01,       //   Physical Maximum (1)
    0x75, 0x01,       //   Report Size (1 bit)
    0x95, 0x10,       //   Report Count (16)
    0x05, 0x09,       //   Usage Page (Button)
    0x19, 0x01,       //   Usage Minimum (Button 1)
    0x29, 0x10,       //   Usage Maximum (Button 16)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)
    // --- Hat Switch (4 bits + 4 bits padding) ---
    0x05, 0x01,       //   Usage Page (Generic Desktop)
    0x25, 0x07,       //   Logical Maximum (7)
    0x46, 0x3B, 0x01, //   Physical Maximum (315)
    0x75, 0x04,       //   Report Size (4 bits)
    0x95, 0x01,       //   Report Count (1)
    0x65, 0x14,       //   Unit (Degrees)
    0x09, 0x39,       //   Usage (Hat Switch)
    0x81, 0x42,       //   Input (Data, Variable, Absolute, Null State)
    0x65, 0x00,       //   Unit (None)
    0x95, 0x01,       //   Report Count (1)
    0x81, 0x01,       //   Input (Constant) - 4-bit padding
    // --- Analog Sticks (4 axes, 1 byte each) ---
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x46, 0xFF, 0x00, //   Physical Maximum (255)
    0x09, 0x30,       //   Usage (X) - Left Stick X
    0x09, 0x31,       //   Usage (Y) - Left Stick Y
    0x09, 0x32,       //   Usage (Z) - Right Stick X
    0x09, 0x35,       //   Usage (Rz) - Right Stick Y
    0x75, 0x08,       //   Report Size (8 bits)
    0x95, 0x04,       //   Report Count (4)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)
    // --- Vendor Specific Input (1 byte) ---
    0x06, 0x00, 0xFF, //   Usage Page (Vendor Defined)
    0x09, 0x20,       //   Usage (Vendor Usage 0x20)
    0x95, 0x01,       //   Report Count (1)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)
    // --- Vendor Specific Output (8 bytes) ---
    0x0A, 0x21, 0x26, //   Usage (Vendor Usage 0x2621)
    0x95, 0x08,       //   Report Count (8)
    0x91, 0x02,       //   Output (Data, Variable, Absolute)
    0xC0,             // End Collection
];
