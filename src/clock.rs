use once_cell::sync::OnceCell;
use rp235x_hal::{
    clocks::{ClocksManager, init_clocks_and_plls},
    pac::{CLOCKS, PLL_SYS, PLL_USB, RESETS, XOSC},
    watchdog::Watchdog,
};
use spin::Mutex;

pub static CLOCK: OnceCell<Mutex<ClocksManager>> = OnceCell::new();

pub const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub fn init_clocks(
    xtal_freq_hz: u32,
    xosc: XOSC,
    clocks: CLOCKS,
    pll_sys: PLL_SYS,
    pll_usb: PLL_USB,
    resets: &mut RESETS,
    watchdog: &mut Watchdog,
) {
    let clocks_manager = init_clocks_and_plls(
        xtal_freq_hz,
        xosc,
        clocks,
        pll_sys,
        pll_usb,
        resets,
        watchdog,
    )
    .expect("Failed to initialize clocks and PLLs");
    CLOCK
        .set(Mutex::new(clocks_manager))
        .unwrap_or_else(|_| panic!("Failed to set CLOCK with ClocksManager"));
}
