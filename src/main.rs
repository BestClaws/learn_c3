#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay, Rtc,
};
use esp_backtrace as _;
use esp_println::println;
use riscv_rt::entry;

use core::fmt::Write;
use arrayvec::ArrayString;

use embedded_graphics::image::{Image, ImageRaw, ImageRawLE};
use embedded_graphics::mono_font::ascii::{FONT_4X6, FONT_6X10};
use embedded_graphics::mono_font::iso_8859_16::FONT_5X7;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Text};
use fugit::Duration;

use st7735_lcd;
use st7735_lcd::Orientation;

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

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    let mut timer0 = timer_group0.timer0;
    timer0.start(1u64.secs());

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio6;
    let miso = io.pins.gpio2;
    let mosi = io.pins.gpio7;
    let cs = io.pins.gpio10;

    let mut lcd_led = io.pins.gpio4.into_push_pull_output();
    let dc = io.pins.gpio5.into_push_pull_output();
    let rst = io.pins.gpio8.into_push_pull_output();

    let mut spi = Spi::new(
        peripherals.SPI2,
        sclk,
        mosi,
        miso,
        cs,
        100u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );


    let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);




    disp.init(&mut delay).unwrap();


    disp.set_orientation(&Orientation::Landscape).unwrap();
    disp.clear(Rgb565::BLACK).unwrap();
    // disp.set_offset(0, 25);

    // use tinybmp::Bmp;
    // let bmp = Bmp::<Rgb565>::from_slice(include_bytes!("tao.bmp")).unwrap();
    // let image = Image::new(&bmp, Point::new(0, 0)).draw(&mut disp);

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_5X7)
    //     .text_color(Rgb565::WHITE)
    //     .build();
    //
    // Text::with_alignment(
    //     "Chip: ESP32-C3",
    //     disp.bounding_box().center() + Point::new(0, 14),
    //     text_style,
    //     Alignment::Center,
    // )
    // .draw(&mut disp)
    // .unwrap();


    // Wait until the background and image have been rendered otherwise
    // the screen will show random pixels for a brief moment
    lcd_led.set_high().unwrap();




    let mut count = 0;

    let mut  x = 0;
    let mut y = 0;

    loop {
        x+=1;
        y+=1;
        let col = Rgb565::new(255, 0, 0);
        let y: RawU16 = col.into();
        let y = y.into_inner();
        disp.set_pixel(x ,y, y);
    }

}
