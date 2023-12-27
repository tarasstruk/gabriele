# Gabriele

100% 🦀 Rust command-line interface for TA Gabriele 9009 Typewriter.

## Description of this project

This package provides a command-line tool for some electronic typewriters 
manufactured by Triumph-Adler in 1980-1990.
It is developed initially for Gabriele 9009 model of typewriter, but may work with other models
which have a similar serial data interface.


These articles were published in one German magazine in July 1988 and served as motivation to research and create this software.
- [Article part 1](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)
- [Article part 2](https://www.stcarchiv.de/stc1988/08/gabriele-9009-2)

Second the authors of these articles, the machines like IBM Action Writer 6715 would also have similar features,
which opens a way to use them as daisy-wheel printer.



## How to use

### Connecting the typewriter to a computer.

The typewriters data interface is based on UART specification. It was designed to communicate with a computer
using a vendor-specific interface box, called "IFD 1" which
is [mentioned in this article](https://www.stcarchiv.de/stc1986/07/schreibmaschine).

This project does not require this super rare interface box. 
The idea of the current project is to implement some of "IFD 1" built-in functions on "our" side, letting the 
application on a host-computer control the typewriters low-level routines via sending commands through the serial port.

To connect the typewriter to your computer a general-purpose USB-UART adapter with TTL level (5 Volts) is needed.
For example, one based on FT232RL chip, which is also used to develop this project.

If the one is available which can be powered from the external source, the Ppp +5 pin from the typewriter can be 
supplying power to the chip's circuits.

The typewriter provides a 8-pin DIN connector. ⚠️ Please be aware of +35 Volts and +10 Volt pins presence! 
A high voltage can destroy the USB-UART adapter or damage the typewriters electronic components if these pins connected. 
 
The wiring diagram is very simple:

| USB-UART adapter pin | typewriter I/O pin |
|----------------------|--------------------|
| GND, signal ground   | GND, central one   |
| RXD                  | TXD                |
| RTS                  | DSR                |
| TXD                  | RXD                |
| CTS                  | DTR                |
| +5V                  | +5V                |

For the farther information please refer to [this guide](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)

### Running the app in console

When the wires are connected correctly and the USB-UART adapter is recognised by the operating system,
it is found as `/dev/tty.usbserial-`. At this point switch on the typewriter and run the command.

It may fail for the first time run, due to the flow-control issues described below in "Known Issues".

```sh
# print a welcome message
cargo run -- /dev/tty.usbserial-A10OFCFV
# print a text file:
cargo run -- /dev/tty.usbserial-A10OFCFV example.txt
# or with debug output:
RUST_LOG=DEBUG cargo run -- /dev/tty.usbserial-A10OFCFV
```

## Known issues

### Timeouts

The timeouts are applied after or before the sending bytes through the serial tty-interface.
Why this? The data flow is controlled via DTR and DSR signals. The host computer must wait for an acknowledge signal
on DTR line from the typewriter. However, this job appears to be hard for the modern non-realtime operating systems:
we can likely miss the mentioned acknowledge signal, as its timespan is just around 1 millisecond.

In [tweetwronger project](https://github.com/binraker/tweetwronger) this problem is described in more details and
a workaround is proposed: using an extra microcontroller would solve the data transmission flow-control issues.

However, the current project proves that the "open loop" ensures a stable communication between the typewriter and
computer, when a timeout is used after each single byte is transmitted. 

Then "open loop" approach (with timeouts as workaround) has these consequences:
- extra delays are applied to ensure the bytes are accepted and no typewriters internal buffer overflow can happen;
- the delays after the execution of each command should be adjustable: for example, the printing of a single symbol
  duration differs from the carriage-return duration;
- we can focus more on the implementing the logic and interesting features instead of spending time on
  the low-level communication interfaces.


## License

This project is licensed under the following license:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])