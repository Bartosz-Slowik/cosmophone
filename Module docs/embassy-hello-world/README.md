# Embassy Hello World — 4.0" ESP32-4848S040

Minimal **Rust Embassy** “Hello World” for the 4.0" ESP32-4848S040 touchscreen board. It logs to the serial console over USB.

## Prerequisites

1. **ESP Rust toolchain (Xtensa)**  
   ESP32-S3 uses Xtensa, so you need the `esp` toolchain, not standard Rust:

   ```bash
   cargo install espup --locked
   espup install
   ```

   Then load the environment (each new terminal or add to your shell profile):

   - **Windows (PowerShell):** `.\export-esp.ps1`
   - **Linux/macOS:** `source export-esp.sh`

2. **Flash tool**

   ```bash
   cargo install espflash
   ```

## Build

From this directory:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

## Flash and monitor

Connect the board via USB, then:

```bash
cargo run
```

Or flash and open the serial monitor manually:

```bash
espflash flash --monitor target/xtensa-esp32s3-none-elf/debug/embassy-hello-world
```

You should see `Hello World from Rust Embassy!` and then `Hello World!` every second on the serial console.

## Board note

This project targets **ESP32-S3** to match the Arduino demos in this repo. Display/touch (e.g. ST7701, GT911) are not used here; this example only demonstrates Embassy + logging over serial.
