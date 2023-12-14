// @file: basic.rs: LED control for LegoPi

// for main loop
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// WS2812 related
use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;

// math
use core::f32::consts::PI;

fn strobe()
{

}

fn pulse()
{

}

fn main()
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Program started, press Ctrl-C for exit");

    // thread/sleep related
    let ten_millis = time::Duration::from_millis(10);
    let mut millis_elapsed: u32 = 0;

    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(18) // GPIO 10 = SPI0 MOSI
                .count(8) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(0) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = [0x14, 0xFF, 0x39, 100];
    }

    while running.load(Ordering::SeqCst) {
        // sinewave, amplitude of 100, min-max: 0 to 200, time period of 3000
        let brightness = 125.0 + 125.0 * (2.0 * PI/ 3000.0 * millis_elapsed as f32).sin();
        println!("brightness: {}", brightness as u8);
        controller.set_brightness(0, brightness as u8);
        controller.render().unwrap();

        //
        millis_elapsed += 10;
        thread::sleep(ten_millis);
    }

    println!("\nSIGINT/Ctrl+C received, exiting and turning off...");

    // turn off
    controller.set_brightness(0, 0);
    controller.render().unwrap();
    controller.wait().unwrap();
    drop(controller);
}
