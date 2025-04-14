#![no_std] // Linter usually falsely flags "can't find crate for `test`" error. Ignore it.

// Constants for display dimensions
const EPD_2IN13_V4_WIDTH: u16 = 122;
const EPD_2IN13_V4_HEIGHT: u16 = 250;

use embedded_hal::{
    delay::DelayNs,
    digital::{OutputPin, StatefulOutputPin},
    spi::SpiBus,
};
use rp235x_hal::{
    Spi,
    gpio::{
        FunctionSio, FunctionSpi, Pin, PullDown, SioOutput,
        bank0::{Gpio1, Gpio8, Gpio9, Gpio10, Gpio11, Gpio12, Gpio13},
    },
    pac::SPI1,
    spi::Enabled,
};
use spin::Mutex;
use timer::DEFAULT_TIMER;

pub mod clock;
pub mod spi0;
pub mod timer;
pub mod watchdog;

// Struct for EPD
pub struct Epd2in13V4 {
    epd_dc_pin: Mutex<Pin<Gpio8, FunctionSio<SioOutput>, PullDown>>,
    epd_rst_pin: Mutex<Pin<Gpio12, FunctionSio<SioOutput>, PullDown>>,
    epd_busy_pin: Mutex<Pin<Gpio13, FunctionSio<SioOutput>, PullDown>>,
    epd_cs_pin: Mutex<Pin<Gpio9, FunctionSio<SioOutput>, PullDown>>,
    pub heartbeat: Mutex<Pin<Gpio1, FunctionSio<SioOutput>, PullDown>>,
    spi_bus: Mutex<
        Spi<
            Enabled,
            SPI1,
            (
                Pin<Gpio11, FunctionSpi, PullDown>,
                Pin<Gpio10, FunctionSpi, PullDown>,
            ),
        >,
    >,
}

impl<'a> Epd2in13V4 {
    pub fn new(
        epd_dc_pin: Mutex<Pin<Gpio8, FunctionSio<SioOutput>, PullDown>>,
        epd_rst_pin: Mutex<Pin<Gpio12, FunctionSio<SioOutput>, PullDown>>,
        epd_busy_pin: Mutex<Pin<Gpio13, FunctionSio<SioOutput>, PullDown>>,
        epd_cs_pin: Mutex<Pin<Gpio9, FunctionSio<SioOutput>, PullDown>>,
        heartbeat: Mutex<Pin<Gpio1, FunctionSio<SioOutput>, PullDown>>,
        spi_bus: Mutex<
            Spi<
                Enabled,
                SPI1,
                (
                    Pin<Gpio11, FunctionSpi, PullDown>,
                    Pin<Gpio10, FunctionSpi, PullDown>,
                ),
            >,
        >,
    ) -> Self {
        Self {
            epd_dc_pin,
            epd_rst_pin,
            epd_busy_pin,
            epd_cs_pin,
            heartbeat,
            spi_bus,
        }
    }

    pub fn reset(&self) {
        let mut timer = DEFAULT_TIMER.get().unwrap().lock();
        let mut gpio12 = self.epd_rst_pin.lock();
        let epd_rst_pin = &mut *gpio12;
        epd_rst_pin.set_high().unwrap();
        timer.delay_ms(20);
        epd_rst_pin.set_low().unwrap();
        timer.delay_ms(2);
        epd_rst_pin.set_high().unwrap();
        timer.delay_ms(20);
    }

    pub fn clear(&self) {
        let width = if EPD_2IN13_V4_WIDTH % 8 == 0 {
            EPD_2IN13_V4_WIDTH / 8
        } else {
            EPD_2IN13_V4_WIDTH / 8 + 1
        };
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for _ in 0..height {
            for _ in 0..width {
                self.send_data(0xFF); // Fill with white
            }
        }

        self.turn_on_display(); // Trigger display update
    }

    fn send_command(&self, command: u8) {
        self.epd_dc_pin.lock().set_low().unwrap();
        self.epd_cs_pin.lock().set_low().unwrap();
        self.spi_write_byte(command);
        self.epd_cs_pin.lock().set_high().unwrap();
    }

    fn send_data(&self, data: u8) {
        self.epd_dc_pin.lock().set_high().unwrap();
        self.epd_cs_pin.lock().set_low().unwrap();
        self.spi_write_byte(data);
        self.epd_cs_pin.lock().set_high().unwrap();
    }

    fn spi_write_byte(&self, byte: u8) {
        let mut spi_bus = self.spi_bus.lock();
        if spi_bus.write(&[byte]).is_ok() {
            // Successfully sent byte
        } else {
            // Rapidly flash "heartbeat" LED to indicate error
            let mut heartbeat = self.heartbeat.lock();
            let mut timer = DEFAULT_TIMER.get().unwrap().lock();
            loop {
                heartbeat.set_high().unwrap();
                timer.delay_ms(100);
                heartbeat.set_low().unwrap();
                timer.delay_ms(100);
            }
        }
    }

    fn turn_on_display(&self) {
        self.send_command(0x22); // Display Update Control
        self.send_data(0xF7); // Activate display update sequence
        self.send_command(0x20); // Execute the update
        self.wait_until_idle(); // Wait for the display to finish updating
    }

    fn wait_until_idle(&self) {
        let mut busy_pin = self.epd_busy_pin.lock();
        let mut timer = DEFAULT_TIMER.get().unwrap().lock();
        while busy_pin.is_set_high().unwrap() {
            timer.delay_ms(10);
        }
    }
}
