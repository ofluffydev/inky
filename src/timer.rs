use once_cell::sync::OnceCell;
use rp235x_hal::{Timer, timer::CopyableTimer0};
use spin::Mutex;

pub static DEFAULT_TIMER: OnceCell<Mutex<Timer<CopyableTimer0>>> = OnceCell::new();
