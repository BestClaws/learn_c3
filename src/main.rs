//! Turns on LED with the option to change LED intensity depending on `duty`
//! value. Possible values (`u32`) are in range 0..100.
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO4)

#![no_std]
#![no_main]

use esp32c3_hal::{clock::ClockControl, gpio::IO, ledc::{
    channel::{self, ChannelIFace},
    timer::{self, TimerIFace},
    LSGlobalClkSource,
    LowSpeed,
    LEDC,
}, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, Delay};
use esp_backtrace as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio5.into_push_pull_output();

    let mut ledc = LEDC::new(
        peripherals.LEDC,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer2);

    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 50u32.Hz(),
        })
        .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel1, led);
    let mut direction:i8 = 1;
    let mut count:u32 = 0;

    let mut delay = Delay::new(&clocks);

    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
        })
        .unwrap();
    use esp_println::println;

    loop {

        println!("{}",count);

        channel0.set_duty_hw(count);
        // delay.delay_ms(1u32);

        if(direction > 0) {
            count += 2;
        } else {
            count -= 2;
        }

        if(count >= 2200) {
            direction = -1;
        } else if (count < 200) {
            direction = 1;
        }
    }
}