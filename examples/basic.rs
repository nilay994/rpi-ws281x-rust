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

// LEDs
const BP_LED_0: u8 = 0;
const BP_LED_1: u8 = 1;
const BP_LED_2: u8 = 2;
const BP_LED_3: u8 = 3;
const BP_LED_4: u8 = 4;
const BP_LED_5: u8 = 5;
const BP_LED_6: u8 = 6;
const BP_LED_7: u8 = 7;


/*
 * Generates strobe pattern. Shall be called every 10 ms.
 *
 * A strobe pattern has two on-times and has a time period.
 * __|""""|____|""""|_____________________
 *
 * @param millis_elapsed: Time elapsed since startup
 * @returns 1 if LEDs should be on in the current cycle, 0 elsewise
 */
fn strobe(millis_elapsed: u32) -> u8
{
    let on_time: u32 = 50;  // 50 ms
    let gap: u32 = 200;     // 200 ms
    let period: u32 = 1500; // 1500 ms

    if (millis_elapsed % period) < on_time {
        return 1;
    } else if (millis_elapsed % period) < (on_time + gap) {
        return 0;
    } else if (millis_elapsed % period) < (on_time + gap + on_time) {
        return 1;
    } else {
        return 0;
    }
}

/*
 * Generates pulse pattern. Shall be called every 10 ms.
 *
 * Follows a sinusoidal pattern with a frequency of 3 seconds
 *
 * @param millis_elapsed: Time elapsed since startup
 * @returns The brightness [0..255] of the LEDs in the current cycle
 */
fn pulse(millis_elapsed: u32) -> u8
{
    // sinewave, amplitude of 125, min-max: 0 to 250, time period of 3000
    let brightness = 125.0 + 125.0 * (2.0 * PI / 3000.0 * millis_elapsed as f32).sin();
    return brightness as u8;
}

fn main()
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    println!("\n------Lego Pi------");
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Program started, press Ctrl-C to exit");

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
        controller.set_brightness(0, strobe(millis_elapsed) * 200);
        controller.render().unwrap();

        // cycles from 0 to 12 seconds
        millis_elapsed += 10;
        millis_elapsed = millis_elapsed % 12000;
        thread::sleep(ten_millis);
    }

    println!("\n------Lego Pi------");
    println!("SIGINT/Ctrl+C received, turning off all LEDs and closing program...");

    // turn off
    controller.set_brightness(0, 0);
    controller.render().unwrap();
    controller.wait().unwrap();
    drop(controller);
}
