# Gabriele

100% 🦀 Rust command-line interface for TA Gabriele 9009 Typewriter.

This package provides a command-line tool for electronic typewriters with UART interface
manufactured by Triumph-Adler in 1980-1990. Tested with Gabriele 9009.
It may work also with similar Triumph-Adler devices and IBM Action Writer 6715.

## Features
- [x] abstraction layer for TA Gabriele 9009 serial port connection and the mission-critical low level commands;
- [x] typewriter's coordinate system and basic printing logic, including the carriage return;
- [x] printing of text files;
- [x] printing of composite UTF-8 characters which don't exist on the physical daisy wheel (like "é");
- [x] impression compensation;
- [ ] bidirectional printing;
- [ ] proportional characters;
- [ ] bold characters;
- [ ] command-line "native" typewriter mode;


## Connecting the typewriter to a computer

The typewriters data interface is based on UART specification. It was designed to communicate with a host computer
through vendor-specific interface box "IFD 1". 
 
The idea of the project is to to connect our computer "directly" and drive the typewriter with its native 2-bytes 
commands. 

However, there are still two hardware obstacles to overcome.

### 1. Host machine provides only the USB interface

To connect the typewriter to your computer a general-purpose USB-UART adapter with TTL level (5 Volts) is needed.
For example, one based on FT232RL chip, which is also used to develop this project.

If the one is available which can be powered from the external source, the Ppp +5 pin from the typewriter can be 
supplying power to the chip's circuits.

The typewriter provides a 8-pin DIN connector. Be aware of +35 Volts and +10 Volt pins presence:
a high voltage can damage the electronic components if these pins are connected. 
 
The wiring diagram:

| USB-UART adapter pins | purpose, direction | typewriter I/O pins  |
|-----------------------|--------------------|----------------------|
| `GND`                 | ground             | `GND`                |
| `VCCIO`               | `+` power          | `+5V`                |
| `RXD`                 | `<-`               | `TXD`                |
| `TXD`                 | `->`               | `RXD`                |
| `CTS#`                | `<-`               | `DTR`                |
| `RTS#`                | `->`               | `DSR`                |
| not connected         | not connected      | `+10V`  ⚠️           |
| not connected         | not connected      | `+35V`  ⚠️           |


### 2. Typewriter sends a feedback after accepted command in a non-standard way

The data flow is controlled via CTS# and RTS#  signals on the host side. 
Once a single byte is sent, the host computer waits for a confirmation signal. After each successfully received byte 
the typewriter pulls up the DTR pin up for a short time, about `2..8` milliseconds:

![image](/docs/tx_cts.jpg)

However, in non-realtime operating system environment this short high-level signal on CTS# line is likely to be missed.
To give the host computer a chance to read the feedback signal from typewriter we have to latch the state of `CTS` 
line for longer time.
The idea is to pull this line high when the data transmission begins and release it when the confirmation signal comes.

Let's change the signal shape between `DTR` pin of the typewriter and `CTS` pin on the host-computer side:

1. when a start-bit pulls down the `TX` line, the `CTS` pin is pulled up and latched in high state;
2. typewriter confirms the data reception pulling its `DTR` line up for about 5 ms. and then down;
3. on the falling edge on `DTR` line we pull down the `CTS` pin (possibly with hysteresis effect).

The middleware (latch) design considerations:
- the latch can be built with one `RS#` flip-flop and two logical inverters;
- it's recommended to use the inverters with hysteresis, for example SNx4HC14 with Schmitt-trigger inputs;
- `RS#` trigger can be built on a single IC, for example SNx4HC00 by connecting two 2-input NAND gates together;
- `RTS` pin (on typewriter side) is inverted and then routed to `R#` input;
- `TXD` pin is routed directly to `S#` input;
- `Q` output is inverted and routed to NAND gate;
- the second input of NAND gate is connected to `R#`;
- the output of NAND gate is connected to `CTS#` pin of the host computers UART interface (usually UART-to-USB adapter).

![image](/docs/cts_latch.jpg)


### Running the app

When the wires are connected correctly and the USB-UART adapter is recognised by the operating system,
it is found as `/dev/tty.usbserial-`. At this point switch on the typewriter and run the command.

It may fail for the first time run, due to the flow-control issues described below in "Known Issues".

```sh
# print from the STDIN:
cargo run -- --tty /dev/tty.usbserial-A10OFCFV
# print a text file:
cargo run -- --tty /dev/tty.usbserial-A10OFCFV --text welcome.txt
# or with debug output:
RUST_LOG=DEBUG cargo run -- --tty /dev/tty.usbserial-A10OFCFV
```


## External links to learn more

- [ST Computer article part 1](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)
- [ST Computer article part 2](https://www.stcarchiv.de/stc1988/08/gabriele-9009-2)
- [tweetwronger project](https://github.com/binraker/tweetwronger)
- [ST Computer article about IFD-1 interface box](https://www.stcarchiv.de/stc1986/07/schreibmaschine)


## License

This project is licensed under the following license:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
