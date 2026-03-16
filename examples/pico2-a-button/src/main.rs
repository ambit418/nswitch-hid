//! Pico 2 example: register as a Nintendo Switch controller and press A once.
//!
//! This firmware emulates a HORI Pokken Controller on a Raspberry Pi Pico 2.
//! After USB enumeration it presses L+R to register with the Switch, then
//! presses A once and idles.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_time::Timer;
use embassy_usb::Builder;
use nswitch_hid::{Buttons, ControllerState};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

/// USB Vendor ID for HORI.
const VID: u16 = 0x0F0D;
/// USB Product ID for Pokken Controller.
const PID: u16 = 0x0092;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Create the USB driver from the USB peripheral.
    let driver = Driver::new(p.USB, Irqs);

    // Configure the USB device identity.
    let mut config = embassy_usb::Config::new(VID, PID);
    config.manufacturer = Some("HORI CO.,LTD.");
    config.product = Some("POKKEN CONTROLLER");
    config.serial_number = None;
    config.max_power = 500;
    config.max_packet_size_0 = 64;

    // Allocate buffers for the USB stack.
    let mut config_descriptor = [0u8; 256];
    let mut bos_descriptor = [0u8; 256];
    let mut control_buf = [0u8; 64];

    // HID state must be declared before builder so it outlives it.
    let mut hid_state = embassy_usb::class::hid::State::new();

    // Build the USB device.
    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no MSOS descriptors
        &mut control_buf,
    );
    let hid = nswitch_hid::create_hid(&mut builder, &mut hid_state);
    let (_, mut writer) = hid.split();

    // Start the USB device.
    let mut usb = builder.build();
    let usb_fut = usb.run();

    let hid_fut = async {
        let mut state = ControllerState::new();

        // Send neutral reports while waiting for USB enumeration.
        state.buttons = Buttons::empty();
        for _ in 0..50 {
            let _ = writer.write(&state.to_report()).await;
            Timer::after_millis(100).await;
        }

        // Press L + R once to register the controller.
        state.buttons = Buttons::L | Buttons::R;
        for _ in 0..5 {
            let _ = writer.write(&state.to_report()).await;
            Timer::after_millis(100).await;
        }

        // Release and send neutral reports while registration completes.
        state.buttons = Buttons::empty();
        for _ in 0..30 {
            let _ = writer.write(&state.to_report()).await;
            Timer::after_millis(100).await;
        }

        // Press A once.
        state.buttons = Buttons::A;
        for _ in 0..5 {
            let _ = writer.write(&state.to_report()).await;
            Timer::after_millis(100).await;
        }
        state.buttons = Buttons::empty();
        let _ = writer.write(&state.to_report()).await;

        // Keep sending neutral reports to stay connected.
        loop {
            let _ = writer.write(&state.to_report()).await;
            Timer::after_millis(100).await;
        }
    };

    // Run USB stack and HID task concurrently.
    embassy_futures::join::join(usb_fut, hid_fut).await;
}
