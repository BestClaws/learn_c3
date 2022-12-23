
#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay,
    Rtc,
};
use esp_backtrace as _;
use riscv_rt::entry;

use esp_println::println;
use crate::Status::MeasuringComplete;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // disable watch dogs
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    // end disable watch dogs


    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut trig = io.pins.gpio4.into_push_pull_output();
    let echo = io.pins.gpio5.into_pull_down_input();



    // setup timers and delays.
    let mut timer0 = timer_group0.timer0;
    timer0.set_counter_active(true);

    // end setup timers and delays.


    // set initial state.
    trig.set_low().unwrap();
    let mut status = Status::PulseStart;

    // let mut pulse_start = 0;
    // let mut count = 0;

    const DIV: u64 = 40;

    loop {


        if (status == Status::PulseStart) {
            trig.set_high().unwrap();
            // pulse_start = timer0.now();
            status = Status::Pulsing(timer0.now() + 10 * DIV);
        }

        if let Status::Pulsing(till) = status {
                let now = timer0.now();
            if (now > till) {
                trig.set_low().unwrap();
                if(echo.is_low().unwrap()) {
                    status = Status::ReadyToMeasure;
                } else {
                    status = Status::UnreadyToMeasure;
                }
                // println!("pulse_start: {}, till: {}, count: {}, now: {}, error: {}, tim taken: {}", pulse_start / D, till /D  , count, now / D, (now - till) / D,  (now - pulse_start) / D);
                // status = Status::PulseStart;
                // count = 0;
            } else {
                // count+=1;
            }
        }

        // if (status == Status::UnreadyToMeasure && echo.is_low().unwrap()) {
        //     status = Status::PulseStart;
        // }

        if (status == Status::ReadyToMeasure && echo.is_high().unwrap()) {
            status = Status::Measuring(timer0.now());
        }

        if let Status::Measuring(since) = status {
            if (echo.is_low().unwrap()) {
                let time_taken = timer0.now() - since;
                status = Status::MeasuringComplete(time_taken);
                println!("distance: {}", (time_taken / DIV) as f32 * 0.034 / 2_f32);
                status = Status::PulseStart;
            }
        }

    }
}

#[derive(PartialEq)]
enum Status {
    PulseStart,
    Pulsing(u64), // till
    UnreadyToMeasure,
    ReadyToMeasure,
    Measuring(u64), // since
    MeasuringComplete(u64), // time high.
}