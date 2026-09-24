#![allow(unused_attributes)]
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_backtrace as _;
use esp_hal::time::{Duration, Instant};

pub fn repeat() {
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(500) {}
}
