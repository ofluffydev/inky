#![no_std]
#![no_main]

/*
    e-Paper     Pico/Pico2 	    Description
    VCC 	    VSYS 	        Power input
    GND 	    GND 	        Ground
    DIN 	    GP11 	        MOSI pin of SPI interface, data transmitted from the Host to Slave.
    CLK 	    GP10 	        SCK pin of SPI interface, clock input of the Slave
    CS 	        GP9 	        Chip select pin of SPI interface, Low Active
    DC 	        GP8 	        Data/Command control pin (High: Data; Low: Command)
    RST 	    GP12 	        Reset pin, low active
    BUSY 	    GP13 	        Busy output pin
*/


use hal::{
    Timer,
    binary_info::{
        rp_cargo_bin_name, rp_cargo_homepage_url, rp_cargo_version, rp_program_build_attribute,
        rp_program_description,
    },
    block::ImageDef,
};
use inky::{
    Epd2in13V4,
    clock::{self, CLOCK, XTAL_FREQ_HZ},
    spi0::SPI0,
    timer::DEFAULT_TIMER,
    watchdog::WATCHDOG,
};
use panic_halt as _;

use embedded_hal::{delay::DelayNs, digital::OutputPin};
use rp235x_hal::{self as hal, Clock, fugit::RateExtU32};

use defmt_rtt as _;
use spin::mutex::Mutex;

/// Tell the Boot ROM about our application
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    defmt::info!("I got here!");
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let _ = WATCHDOG.set(Mutex::new(hal::Watchdog::new(pac.WATCHDOG)));
    let _ = SPI0.set(Mutex::new(pac.SPI0));

    // Initialize the global clock
    clock::init_clocks(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut WATCHDOG.get().unwrap().lock(),
    );
    let clocks = CLOCK.get().unwrap().lock();

    let mut timer = Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &*clocks);
    let _ = DEFAULT_TIMER.set(Mutex::new(timer));
    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let epd_dc_pin = Mutex::new(pins.gpio8.into_push_pull_output());
    let epd_rst_pin = Mutex::new(pins.gpio12.into_push_pull_output());
    let epd_busy_pin = Mutex::new(pins.gpio13.into_floating_input());
    let epd_cs_pin = Mutex::new(pins.gpio9.into_push_pull_output());
    // let heartbeat = Mutex::new(pins.gpio1.into_push_pull_output());

    // The e-ink does not use MISO
    let spi_mosi = pins.gpio11.into_function::<hal::gpio::FunctionSpi>();
    let spi_sclk = pins.gpio10.into_function::<hal::gpio::FunctionSpi>();
    let spi_bus = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI1, (spi_mosi, spi_sclk));

    let spi_bus = spi_bus.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        4u32.MHz(),
        embedded_hal::spi::MODE_0,
    );

    let spi_bus_mutex = Mutex::new(spi_bus);

    epd_cs_pin.lock().set_high().unwrap();

    let display = Epd2in13V4::new(
        epd_dc_pin,
        epd_rst_pin,
        epd_busy_pin,
        epd_cs_pin,
        None,
        spi_bus_mutex,
    );
    // display.heartbeat.lock().set_high().unwrap();
    display.init();
    display.init_fast(); // Reset and init the display (Required on startups)
    display.clear(); // Clear the display
    display.test(); // Call the test function
    timer.delay_ms(1000); // Wait for 1 second

    // let mut heartbeat = display.heartbeat.lock();
    // loop {
    //     display.show_bk1();
    //     timer.delay_ms(500);
    //     display.show_bk2();
    //     timer.delay_ms(500);
    //     display.show_bk3();
    //     timer.delay_ms(500);
    //     display.show_bk4();
    //     timer.delay_ms(500);
    // }

    display.show_toaster();

    loop {}
}

/// Program metadata for `picotool info`
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    rp_cargo_bin_name!(),
    rp_cargo_version!(),
    rp_program_description!(c"Inky"),
    rp_cargo_homepage_url!(),
    rp_program_build_attribute!(),
];
