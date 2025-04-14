#![no_std] // Linter falsely flags "can't find crate for `test`" error. Ignore it.
#![feature(unchecked_shifts)]
// Constants for display dimensions
const EPD_2IN13_V4_WIDTH: usize = 122;
const EPD_2IN13_V4_HEIGHT: usize = 250;

const BYTES_PER_LINE: usize = (EPD_2IN13_V4_WIDTH + 7) / 8;
const FRAMEBUFFER_SIZE: usize = BYTES_PER_LINE * EPD_2IN13_V4_HEIGHT;

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
    spi::SpiBus,
};
use paint::{
    Paint,
    Rotation::{Rotate0, Rotate90},
};
use rp235x_hal::{
    Spi,
    gpio::{
        FunctionSio, FunctionSpi, Pin, PullDown, PullNone, SioInput, SioOutput,
        bank0::{Gpio1, Gpio8, Gpio9, Gpio10, Gpio11, Gpio12, Gpio13},
    },
    pac::SPI1,
    spi::Enabled,
};
use spin::Mutex;
use timer::DEFAULT_TIMER;

pub mod clock;
pub mod paint;
pub mod spi0;
pub mod timer;
pub mod watchdog;

extern crate alloc;

use portable_dlmalloc::DLMalloc;

#[global_allocator]
static ALLOCATOR: DLMalloc = DLMalloc;

lazy_static::lazy_static! {
    static ref BLACK_IMAGE: Mutex<[u8; FRAMEBUFFER_SIZE]> = Mutex::new([0xFF; FRAMEBUFFER_SIZE]); // Initialize with white (0xFF)
}

// Struct for EPD
pub struct Epd2in13V4 {
    epd_dc_pin: Mutex<Pin<Gpio8, FunctionSio<SioOutput>, PullDown>>,
    epd_rst_pin: Mutex<Pin<Gpio12, FunctionSio<SioOutput>, PullDown>>,
    epd_busy_pin: Mutex<Pin<Gpio13, FunctionSio<SioInput>, PullNone>>,
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
        epd_busy_pin: Mutex<Pin<Gpio13, FunctionSio<SioInput>, PullNone>>,
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

    pub fn display_bw_image(&self) {
        let width = (EPD_2IN13_V4_WIDTH + 7) / 8;
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for _ in 0..height {
            for _ in 0..width {
                self.send_data(0xFF); // Fill with white
            }
        }

        self.turn_on_display(); // Trigger display update
    }

    pub fn display_white_image(&self) {
        let width = (EPD_2IN13_V4_WIDTH + 7) / 8;
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for _ in 0..height {
            for _ in 0..width {
                self.send_data(0x0); // Fill with 0
            }
        }

        self.turn_on_display(); // Trigger display update
    }

    pub fn clear(&self) {
        self.display_bw_image();
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
        while busy_pin.is_high().unwrap() {
            timer.delay_ms(10);
        }
    }

    pub fn init(&self) {
        self.reset();

        self.wait_until_idle();
        self.send_command(0x12); // SWRESET
        self.wait_until_idle();

        self.send_command(0x01); // Driver output control
        self.send_data(0xF9);
        self.send_data(0x00);
        self.send_data(0x00);

        self.send_command(0x11); // Data entry mode
        self.send_data(0x03);

        self.set_windows(
            0,
            0,
            EPD_2IN13_V4_WIDTH as u8 - 1,
            EPD_2IN13_V4_HEIGHT as u8 - 1,
        );
        self.set_cursor(0, 0);

        self.send_command(0x3C); // BorderWaveform
        self.send_data(0x05);

        self.send_command(0x21); // Display update control
        self.send_data(0x00);
        self.send_data(0x80);

        self.send_command(0x18); // Read built-in temperature sensor
        self.send_data(0x80);
        self.wait_until_idle();

        let image = Paint::new_image(
            Paint::get_image(),
            EPD_2IN13_V4_WIDTH,
            EPD_2IN13_V4_HEIGHT,
            Rotate0,
            0xFF,
        );

        self.display_image(image.image);
    }

    pub fn init_fast(&self) {
        self.reset();

        self.send_command(0x12); // SWRESET
        self.wait_until_idle();

        self.send_command(0x18); // Read built-in temperature sensor
        self.send_data(0x80);

        self.send_command(0x11); // Data entry mode
        self.send_data(0x03);

        self.set_windows(
            0,
            0,
            EPD_2IN13_V4_WIDTH as u8 - 1,
            EPD_2IN13_V4_HEIGHT as u8 - 1,
        );
        self.set_cursor(0, 0);

        self.send_command(0x22); // Load temperature value
        self.send_data(0xB1);
        self.send_command(0x20);
        self.wait_until_idle();

        self.send_command(0x1A); // Write to temperature register
        self.send_data(0x64);
        self.send_data(0x00);

        self.send_command(0x22); // Load temperature value
        self.send_data(0x91);
        self.send_command(0x20);
        self.wait_until_idle();
    }

    fn set_windows(&self, x_start: u8, y_start: u8, x_end: u8, y_end: u8) {
        self.send_command(0x44); // Set RAM X-address Start/End position
        self.send_data(x_start >> 3);
        self.send_data(x_end >> 3);

        self.send_command(0x45); // Set RAM Y-address Start/End position
        self.send_data((y_start as u16 >> 8) as u8 & 0xFF);
        self.send_data((y_start as u16 & 0xFF) as u8);
        self.send_data((y_end as u16 >> 8) as u8 & 0xFF);
        self.send_data((y_end as u16 & 0xFF) as u8);
    }

    fn set_cursor(&self, x: u8, y: u8) {
        self.send_command(0x4E); // SET_RAM_X_ADDRESS_COUNTER
        self.send_data(x);

        self.send_command(0x4F); // SET_RAM_Y_ADDRESS_COUNTER
        self.send_data((y & 0xFF) as u8);
        self.send_data(((y.wrapping_shr(8)) & 0xFF) as u8);
    }

    pub fn create_black_image_cache(&self) -> Result<&'static mut [u8], &'static str> {
        let mut black_image = BLACK_IMAGE.lock();
        if black_image.is_empty() {
            Err("Failed to allocate memory for image cache")
        } else {
            Ok(unsafe { &mut *(black_image.as_mut_ptr() as *mut [u8; FRAMEBUFFER_SIZE]) })
        }
    }

    pub fn display_image(&self, image: &[u8]) {
        let width = (EPD_2IN13_V4_WIDTH + 7) / 8;
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for j in 0..height {
            for i in 0..width {
                let data = image[i + j * width];
                self.send_data(data);
            }
        }

        self.turn_on_display(); // Trigger display update
    }

    pub fn display_fast(&self, image: &[u8]) {
        let width = (EPD_2IN13_V4_WIDTH + 7) / 8;
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for j in 0..height {
            for i in 0..width {
                let data = image[i + j * width];
                self.send_data(data);
            }
        }

        self.turn_on_display_fast();
    }

    fn turn_on_display_fast(&self) {
        self.send_command(0x22); // Display Update Control
        self.send_data(0xC7); // Fast update sequence
        self.send_command(0x20); // Execute the update
        self.wait_until_idle(); // Wait for the display to finish updating
    }

    pub fn display_predefined_image(&self) {
        let width = (EPD_2IN13_V4_WIDTH + 7) / 8;
        let height = EPD_2IN13_V4_HEIGHT;

        self.send_command(0x24); // Write Black and White image to RAM
        for j in 0..height {
            for i in 0..width {
                self.send_data(paint::G_IMAGE_2IN13_2[i + j * width]);
            }
        }

        self.turn_on_display(); // Trigger display update
    }

    pub fn test(&self) {
        let predefined_image = paint::G_IMAGE_2IN13_2; // Retrieve the predefined image
        self.display_fast(&predefined_image); // Use fast refresh to display the image
    }
}
