#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use mkl25z4_hal::gpio::{self, GpioExt};
use mkl25z4_hal::time::Hertz;
use mkl25z4_hal::timer::Timer;
use mkl25z4_hal::{clocks, watchdog};
use nb::block;

pub struct Led {
    pub red: gpio::gpiob::PB18<gpio::Output<gpio::PushPull>>,
    pub green: gpio::gpiob::PB19<gpio::Output<gpio::PushPull>>,
    pub blue: gpio::gpiod::PD1<gpio::Output<gpio::PushPull>>,
}

impl Led {
    /// Sets the LED's color
    pub fn set_color(&mut self, red: bool, green: bool, blue: bool) {
        if red {
            self.red.set_low().ok();
        } else {
            self.red.set_high().ok();
        }
        if green {
            self.green.set_low().ok();
        } else {
            self.green.set_high().ok();
        }
        if blue {
            self.blue.set_low().ok();
        } else {
            self.blue.set_high().ok();
        }
    }
}

#[entry]
fn main() -> ! {
    let mut device = mkl25z4::Peripherals::take().unwrap();

    watchdog::disable(&mut device.SIM);
    let clocks = clocks::init();
    let mut gpiob = device.GPIOB.split(&mut device.SIM);
    let mut gpiod = device.GPIOD.split(&mut device.SIM);
    let mut led = Led {
        red: gpiob.pb18.into_push_pull_output(&mut gpiob.pddr),
        green: gpiob.pb19.into_push_pull_output(&mut gpiob.pddr),
        blue: gpiod.pd1.into_push_pull_output(&mut gpiod.pddr),
    };
    led.set_color(true, true, true);

    let mut timer = Timer::pit(device.PIT, Hertz(2), clocks, &mut device.SIM);
    loop {
        timer.start(Hertz(2));
        block!(timer.wait()).ok();
        led.set_color(false, false, false);
        timer.start(Hertz(2));
        block!(timer.wait()).ok();
        led.set_color(true, true, true);
    }
}
