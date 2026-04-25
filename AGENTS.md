# AGENTS.md

## Project Overview

**Gabriele** is a Rust CLI application for driving a Triumph-Adler Gabriele 9009 electronic typewriter via its UART interface over TCP. The project converts UTF-8 text into low-level 2-byte typewriter commands (carriage motion, paper feed, daisy wheel symbol strikes) and transmits them to the machine through an RP2040 microcontroller acting as a network bridge.

## Architecture

The workspace contains three crates:

### `gabriele` (core library, `#![no_std]`)
Platform-independent typewriter abstraction. All domain logic lives here.

- **`cmd`** — Binary command encoding using `deku`. Defines `Cmd` enum variants: `Motion` (carriage/paper movement), `Jump` (space left/right), `SymbolLow`/`SymbolHigh` (print a character). Also defines `Impression` (hammer force: Mild/Normal/Strong/Strongest).
- **`symbol`** — `Symbol` struct: maps a Unicode `char` to up to 2 `Sign` strikes (supports composite characters like `é` = base letter + accent). `ActionMapping` classifies symbols as Print, Whitespace, or LineFeed.
- **`sign`** — `Sign` struct: a single petal on the daisy wheel with index, impression, and after-print carriage direction. Builds the final 2-byte `Instruction`.
- **`database`** — `DaisyDatabase` trait for looking up `Symbol` by `char`.
- **`wheels/standard`** — Static array of 123 symbols mapping the German daisy wheel layout (letters, digits, punctuation, accented characters).
- **`position`** — `Position` (x, y coordinates in typewriter units) tracking the print head.
- **`resolution`** — Default X=12, Y=16 units per character/line.
- **`motion`** — Functions to generate movement `Instruction` iterators (relative, absolute, space jumps).
- **`printing`** — `Instruction` enum (`SendBytes(u16)`, `Halt`). `Action` struct computes instructions and target position for a given symbol in context.
- **`machine`** — `Machine<T: InstructionSender, D: DaisyDatabase>`: the main orchestrator. Converts input text to symbols, generates actions, streams instructions through `InstructionSender`.
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

- The core `gabriele` crate is `#![no_std]` — no standard library, no heap allocation. All symbol data is static.
- Binary protocol: every command is exactly 2 bytes (big-endian `u16`). The top 2 bits encode command type. Bit-level encoding is handled by the `deku` crate.
- Communication uses the echo principle: each transmitted byte must be echoed back identically before the next byte is sent.
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

The typewriter communicates via UART (8-pin DIN connector). A USB-UART adapter or RP2040 microcontroller with the [gabriele-embassy](https://github.com/tarasstruk/gabriele-embassy) firmware provides the TCP bridge. See `docs/README.md` for wiring diagrams and signal flow control details.

