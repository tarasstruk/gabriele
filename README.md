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
To give our host computer a chance to capture the event we have to latch the confirmation signal.

This latch can be built with an `RS#` flip-flop and one logical inverter:
- `CTS#` pin is inverted and then routed to `R#` pin;
- `TXD` pin is routed directly to `S#` pin;
- `Q` output is routed to Ring Indicator `RI#` pin on the UART adapter.

This latch helps to capture the event programmatically, awaiting low-level signal on the `RI#` pin:

![image](/docs/tx_ri.jpg)


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
