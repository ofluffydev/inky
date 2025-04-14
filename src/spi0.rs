use once_cell::sync::OnceCell;
use spin::Mutex;

use rp235x_hal::pac::SPI0;

pub static SPI0: OnceCell<Mutex<SPI0>> = OnceCell::new();
