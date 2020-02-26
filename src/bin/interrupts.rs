#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::cell::Cell;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use mkl25z4::{interrupt, Interrupt, PIT};
use mkl25z4_hal::gpio::{self, GpioExt};
use mkl25z4_hal::time::Hertz;
use mkl25z4_hal::timer::{Timer, TimerInterrupt};
use mkl25z4_hal::{clocks, watchdog};

static LED: Mutex<Cell<Option<gpio::gpiob::PB18<gpio::Output<gpio::PushPull>>>>> =
    Mutex::new(Cell::new(None));
static TIMER: Mutex<Cell<Option<Timer<PIT>>>> = Mutex::new(Cell::new(None));

#[entry]
fn main() -> ! {
    let mut device = mkl25z4::Peripherals::take().unwrap();

    watchdog::disable(&mut device.SIM);
    let clocks = clocks::init();
    let mut gpiob = device.GPIOB.split(&mut device.SIM);

    let timer = Timer::pit(device.PIT, Hertz(2), clocks, &mut device.SIM);
    timer.enable_interrupt();

    cortex_m::interrupt::free(|cs| {
        LED.borrow(cs)
            .set(Some(gpiob.pb18.into_push_pull_output(&mut gpiob.pddr)));
        TIMER.borrow(cs).set(Some(timer));
    });

    unsafe { NVIC::unmask(Interrupt::PIT) };
    loop {
        asm::wfi();
    }
}

#[interrupt]
fn PIT() {
    static mut ON: bool = true;
    cortex_m::interrupt::free(|cs| {
        let led_cell = LED.borrow(cs);
        let timer_cell = TIMER.borrow(cs);
        let mut led = led_cell.replace(None).unwrap();
        let mut timer = timer_cell.replace(None).unwrap();

        timer.wait().ok();
        timer.start(Hertz(2));
        timer.enable_interrupt();

        if *ON {
            led.set_low().ok();
        } else {
            led.set_high().ok();
        }

        led_cell.replace(Some(led));
        timer_cell.replace(Some(timer));
    });
    *ON = !*ON;
}
