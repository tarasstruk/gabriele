# AGENTS.md

## Project Overview

**Gabriele** is a Rust CLI application for driving a Triumph-Adler Gabriele 9009 electronic typewriter via its UART interface over TCP. The project converts UTF-8 text into low-level 2-byte typewriter commands (carriage motion, paper feed, daisy wheel symbol strikes) and transmits them to the machine through an RP2040 microcontroller acting as a network bridge.

The system consists of two repositories:
1. **This repository** — the CLI application (`gabi`) and the core `gabriele` library.
2. **[gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy)** — bare-metal Rust firmware for the Raspberry Pi Pico W (RP2040) that acts as a WiFi-to-UART bridge, running a TCP server on port 1234.

The `gabriele` core library is shared between both projects — it is the central piece of the architecture.

## Architecture

The workspace contains three crates:

### `gabriele` (core library, `#![no_std]`)

The heart of the project — a platform-independent, zero-allocation library that encodes all domain knowledge about driving the typewriter. It is used by both the CLI (`gabi`) and the RP2040 firmware (`gabriele-embassy`).

Because the crate is `#![no_std]` with no heap allocation, all symbol data is defined as static arrays. This makes it suitable for bare-metal environments like the RP2040.

**Modules:**

- **`cmd`** — Binary command encoding using `deku`. Defines `Cmd` enum variants: `Motion` (carriage/paper movement), `Jump` (space left/right), `SymbolLow`/`SymbolHigh` (strike a character). Every command is exactly 2 bytes (big-endian `u16`); the top 2 bits select the command type. Also defines `Impression` (hammer force: Mild/Normal/Strong/Strongest).
- **`symbol`** — `Symbol` struct: maps a Unicode `char` to up to 2 `Sign` strikes (supports composite characters like `è` = base letter with `HoldOn` + grave accent mark). `ActionMapping` classifies symbols as Print, Whitespace, or LineFeed. Builder methods: `petal()`, `grave()`, `acute()`, `imp()`, `mild()`, `strong()`.
- **`sign`** — `Sign` struct: a single petal on the daisy wheel with index, impression, and after-print carriage direction (`MoveRight`, `MoveLeft`, `HoldOn`). Builds the final 2-byte `Instruction`.
- **`database`** — `DaisyDatabase` trait for looking up `Symbol` by `char`.
- **`wheels/standard`** — Static array of symbols mapping the German daisy wheel layout (letters, digits, punctuation, accented characters with grave/acute marks).
- **`position`** — `Position` (x, y coordinates in typewriter units) tracking the print head.
- **`resolution`** — Default X=12, Y=16 units per character/line.
- **`motion`** — Functions to generate movement `Instruction` iterators (relative, absolute, space jumps).
- **`printing`** — `Instruction` enum (`SendBytes(u16)`, `Halt`). `Action` struct computes the instruction sequence and target position for a given symbol, taking into account current position, printing direction, and repetition (grouped whitespace/line feeds).
- **`machine`** — `Machine<T: InstructionSender, D: DaisyDatabase>`: the main orchestrator. Converts input text to symbols via `DaisyDatabase`, generates actions, streams instructions through `InstructionSender`. This is the main entry point for both the CLI and the firmware.
- **`to_symbols`** — `ToSymbols` trait converting `&str` to an iterator of `&Symbol` via `DaisyDatabase`.

### `gabi` (application binary)
Two binaries:

- **`main`** (`gabi`) — CLI entry point. Parses `--ip` and optional `--text` args. Creates a `Machine`, connects via TCP to the RP2040, and prints from stdin or a file.
- **`sim`** — TCP server simulator. Listens on a port, echoes received bytes (for testing without hardware), and writes output to a binary file.
- **`hal`** — Hardware Abstraction Layer. Bridges `Machine` instructions to the TCP client; receives `Instruction` from an mpsc channel, serializes to bytes, and sends over the network. Waits for echo confirmation from the RP2040.
- **`lib`** — `SenderWrapper`: adapts `tokio::mpsc::UnboundedSender<Instruction>` to the `InstructionSender` trait.

### `tcp-client` (network library)
- **`client`** — `run_tcp_client()`: spawns a tokio task that connects to the typewriter's RP2040 over TCP, writes bytes one at a time, and validates echo replies. Supports cancellation and reconnection with backoff.

## Key Conventions

- The core `gabriele` crate is `#![no_std]` — no standard library, no heap allocation. All symbol data is static. This is essential because the crate is also compiled for the RP2040 (`thumbv6m-none-eabi` target) in the `gabriele-embassy` firmware.
- Binary protocol: every command is exactly 2 bytes (big-endian `u16`). The top 2 bits encode command type. Bit-level encoding is handled by the `deku` crate.
- Communication uses the echo principle: each transmitted byte must be echoed back identically before the next byte is sent. The RP2040 firmware manages this — it forwards each byte to the typewriter via UART, waits for a physical confirmation pulse, then echoes it back to the TCP client.
- Composite UTF-8 characters (e.g., `è`, `é`) are printed as two strikes on the same position: base letter (with `HoldOn` direction) followed by the accent mark.
- Daisy wheel layouts are defined as static `Symbol` arrays. The `standard` wheel covers German layout with Latin accented characters.
- Async runtime: `tokio` with full features. Inter-task communication via `mpsc` (Machine→HAL) and `broadcast` (HAL→TCP client) channels.

## Building & Running

```sh
# Build everything
cargo build

# Run the CLI (connect to typewriter at given IP)
cargo run --bin gabi -- --ip 192.168.0.5

# Print a file
cargo run --bin gabi -- --ip 192.168.0.5 --text gabi/welcome.txt

# Run the simulator (for testing without hardware)
cargo run --bin sim -- --ip 127.0.0.1 --port 1234

# Run tests
cargo test
```

## Testing

- Unit tests are co-located in modules (`printing`, `motion`, `symbol`).
- Integration tests are in `gabi/tests/test_printing.rs` with helpers in `gabi/tests/helpers/`.
- The `sim` binary + `ref_output.bin` can be used to validate output against a reference.

## Hardware Context

The typewriter communicates via UART (8-pin DIN connector, 4800 baud). A Raspberry Pi Pico W running the [gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy) firmware provides the TCP-to-UART bridge. The firmware runs a TCP server on port 1234, manages typewriter activation/deactivation sequences, and implements byte-level flow control via the echo principle.

See `docs/README.md` for wiring diagrams, signal flow control details, and alternative connection methods (USB-UART adapter with RS flip-flop circuits).
