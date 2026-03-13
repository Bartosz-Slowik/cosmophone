//! Hello World for 4.0" ESP32-4848S040 touchscreen board
//! Uses Rust Embassy async executor - backlight on, rainbow on display.

#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::Output;
use esp_hal::timer::timg::TimerGroup;

// Minimal ESP-IDF app descriptor so espflash / bootloader accept the image.
// Layout must match esp_app_desc_t; placed so bootloader can find it.
const ESP_APP_DESC_MAGIC: u32 = 0xABCD_5432;

/// Matches esp_app_desc_t (256 bytes). Bootloader reads min/max eFuse rev from here.
#[repr(C)]
pub struct EspAppDesc {
    magic_word: u32,
    secure_version: u32,
    reserv1: [u32; 2],
    version: [u8; 32],
    project_name: [u8; 32],
    time: [u8; 16],
    date: [u8; 16],
    idf_ver: [u8; 32],
    app_elf_sha256: [u8; 32],
    min_efuse_blk_rev_full: u16,
    max_efuse_blk_rev_full: u16,
    mmu_page_size: u8,
    spi_flash_mode: u8,
    reserv3: [u8; 2],
    reserv2: [u32; 18],
}

const fn cstr_array<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let len = if bytes.len() < N { bytes.len() } else { N };
    let mut a = [0u8; N];
    let mut i = 0;
    while i < len {
        a[i] = bytes[i];
        i += 1;
    }
    a
}

#[used]
#[no_mangle]
#[link_section = ".rodata.esp_app_desc"]
pub static esp_app_desc: EspAppDesc = EspAppDesc {
    magic_word: ESP_APP_DESC_MAGIC,
    secure_version: 0,
    reserv1: [0; 2],
    version: cstr_array(env!("CARGO_PKG_VERSION")),
    project_name: cstr_array(env!("CARGO_PKG_NAME")),
    time: cstr_array("00:00:00"),
    date: cstr_array("2025-01-01"),
    idf_ver: cstr_array("5.3"),
    app_elf_sha256: [0; 32],
    // eFuse rev: chip is v1.3 (=103). 0..=199 accepts 0.0 to 1.99.
    min_efuse_blk_rev_full: 0,
    max_efuse_blk_rev_full: 199,
    mmu_page_size: 0,
    spi_flash_mode: 0,
    reserv3: [0; 2],
    reserv2: [0; 18],
};


#[esp_hal::main]
fn main() -> ! {
    // First line: if you see this, we reached main (USB-JTAG).
    esp_println::println!("[main] entered");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    esp_println::println!("[main] HAL init done");

    // Turn display backlight on (GPIO 38 = high). Leak so it stays on.
    let mut backlight = Output::new(peripherals.GPIO38, esp_hal::gpio::Level::High);
    backlight.set_high();
    core::mem::forget(backlight);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let mut executor = esp_hal_embassy::Executor::new();
    let executor: &'static mut esp_hal_embassy::Executor =
        unsafe { core::mem::transmute(&mut executor) };
    executor.run(|spawner: Spawner| {
        spawner.must_spawn(embassy_main(spawner));
    });
}

#[embassy_executor::task]
async fn embassy_main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    log::info!("Hello World from Rust Embassy!");
    log::info!("Board: 4.0\" ESP32-4848S040");
    log::info!("Backlight on – rainbow starting…");

    // Rainbow color names for logging.
    const RAINBOW: [&str; 7] = [
        "red", "orange", "yellow", "green", "blue", "indigo", "violet",
    ];
    let mut step = 0u32;

    loop {
        let color_name = RAINBOW[(step as usize) % RAINBOW.len()];
        log::info!("Rainbow: {}", color_name);
        step = step.wrapping_add(1);
        Timer::after(Duration::from_millis(400)).await;
    }
}
