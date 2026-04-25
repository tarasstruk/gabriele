# Gabriele

100% 🦀 Rust command-line interface for TA Gabriele 9009 Typewriter.

This project provides a command-line tool for electronic typewriters with UART interface
manufactured by Triumph-Adler in 1980–1990. It is tested with Gabriele 9009
and may also work with similar Triumph-Adler devices and IBM Action Writer 6715.

## How It Works

The system consists of two parts:

1. **This repository** — a Rust CLI application (`gabi`) that converts UTF-8 text into the typewriter's native 2-byte UART commands and sends them over TCP.
2. **[gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy)** — bare-metal Rust firmware for the Raspberry Pi Pico W (RP2040) that acts as a WiFi-to-UART bridge for the typewriter.

The Pico W runs a TCP server on **port 1234**. When `gabi` connects:

1. The firmware activates the typewriter via a UART start sequence
2. `gabi` translates text into 2-byte typewriter instructions and streams them over TCP
3. Each byte is forwarded to the typewriter over UART at **4800 baud**
4. The typewriter confirms each byte with a pulse; the firmware echoes it back to the client
5. `gabi` waits for the echo before sending the next byte (flow control)
6. On disconnect, the firmware deactivates the typewriter

## Features

- [x] Abstraction layer for TA Gabriele 9009 serial port connection and low-level 2-byte commands
- [x] Typewriter coordinate system and printing logic, including carriage return
- [x] Printing of text files
- [x] Printing of composite UTF-8 characters (e.g. `é` = base letter + accent)
- [x] Impression compensation (Mild / Normal / Strong / Strongest)
- [x] TCP client communication with the RP2040 bridge
- [ ] Multiple TCP connections for simultaneous printing on several typewriters
- [ ] Bidirectional printing
- [ ] Proportional characters
- [ ] Bold characters
- [ ] Command-line "native" typewriter mode

## Project Structure

The workspace contains three crates:

| Crate | Description |
|-------|-------------|
| **`gabriele`** | Core library (`#![no_std]`). Platform-independent typewriter abstraction: command encoding, daisy wheel mapping, coordinate system, motion/printing logic. Also used by the RP2040 firmware. |
| **`gabi`** | Application binary. CLI entry point, TCP-based HAL, and a simulator for testing without hardware. |
| **`tcp-client`** | Network library. Async TCP client with echo-based flow control, reconnection, and backoff. |

### The `gabriele` core library

The `gabriele` crate is the heart of the project — a `#![no_std]` library with zero heap allocation that encodes all domain knowledge about driving the typewriter. It is shared between this CLI application and the [gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy) RP2040 firmware.

**Key responsibilities:**

- **Binary protocol encoding** (`cmd`) — every typewriter command is exactly 2 bytes (big-endian `u16`). The top 2 bits select the command type: `Motion` (carriage/paper movement), `Jump` (space left/right), `SymbolLow`/`SymbolHigh` (strike a character). Bit-level encoding is handled by the [`deku`](https://crates.io/crates/deku) crate.
- **Daisy wheel mapping** (`symbol`, `sign`, `wheels/standard`) — maps Unicode characters to physical petals on the daisy wheel. Each `Symbol` can produce up to 2 strikes to support composite characters (e.g. `è` = base letter with `HoldOn` + grave accent mark). The standard wheel covers the German layout with Latin accented characters.
- **Coordinate system** (`position`, `resolution`) — tracks the print head position in typewriter units (default: 12 units per character horizontally, 16 per line vertically).
- **Motion generation** (`motion`) — computes movement instructions (relative, absolute, space jumps) as instruction iterators.
- **Printing logic** (`printing`) — `Action` computes the instruction sequence and target position for a given symbol, taking into account current position, printing direction, and repetition (grouped whitespace/line feeds).
- **Machine orchestrator** (`machine`) — `Machine<T: InstructionSender, D: DaisyDatabase>` converts input text to symbols via `DaisyDatabase`, generates actions, and streams instructions through the `InstructionSender` trait. This is the main entry point for both the CLI and the firmware.

Because the crate is `no_std`, all symbol data is defined as static arrays — no runtime allocation is needed. This makes it suitable for bare-metal environments like the RP2040.

## Hardware Setup

### Connecting the typewriter

The typewriter provides an 8-pin DIN connector with a UART interface. A Raspberry Pi Pico W with [gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy) firmware serves as the network bridge.

#### RP2040 wiring

| Pico W Pin | Function |
|------------|----------|
| GP4 | UART1 TX → typewriter RXD |
| GP5 | UART1 RX ← typewriter TXD |
| GP7 | RTS (flow control) → typewriter DSR |
| GP3, GP4 | PIO1 inputs (confirmation pulse detection) |

UART runs at **4800 baud**.

> ⚠️ The typewriter's DIN connector has **+35V** and **+10V** pins — do not connect these to the microcontroller or USB-UART adapter.

For alternative wiring options (USB-UART adapter, RS flip-flop circuits for CTS flow control), see [docs/README.md](docs/README.md).

### Firmware setup

See [gabriele-embassy README](https://github.com/tarasstruk/gabriele-embassy) for flashing the RP2040 firmware, WiFi configuration, and troubleshooting.

## Running the App

```sh
# Print from STDIN (interactive mode):
cargo run --bin gabi -- --ip 192.168.0.5

# Print a text file:
cargo run --bin gabi -- --ip 192.168.0.5 --text gabi/welcome.txt

# With debug output:
RUST_LOG=DEBUG cargo run --bin gabi -- --ip 192.168.0.5
```


Type `exit` and press return to quit interactive mode.

### Simulator (testing without hardware)

```sh
# Start the simulator on localhost:
cargo run --bin sim -- --ip 127.0.0.1 --port 1234

# In another terminal, connect gabi to the simulator:
cargo run --bin gabi -- --ip 127.0.0.1
```

The simulator echoes received bytes (mimicking the RP2040) and writes output to a binary file for verification against `gabi/ref_output.bin`.

## Building & Testing

```sh
# Build everything
cargo build

# Run tests
cargo test
```

## External Links

- [ST Computer article part 1](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)
- [ST Computer article part 2](https://www.stcarchiv.de/stc1988/08/gabriele-9009-2)
- [tweetwronger project](https://github.com/binraker/tweetwronger)
- [ST Computer article about IFD-1 interface box](https://www.stcarchiv.de/stc1986/07/schreibmaschine)

## License

This project is licensed under the MIT license — see [LICENSE](LICENSE) for details.
