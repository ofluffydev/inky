use once_cell::sync::OnceCell;
use rp235x_hal::Watchdog;
use spin::Mutex;

pub static WATCHDOG: OnceCell<Mutex<Watchdog>> = OnceCell::new();
