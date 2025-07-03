# Gabriele

100% 🦀 Rust command-line interface for TA Gabriele 9009 Typewriter.

This package provides a command-line tool for electronic typewriters with UART interface
manufactured by Triumph-Adler in 1980-1990. Package is tested with Gabriele 9009.
It may work also work fine with similar Triumph-Adler devices and IBM Action Writer 6715.

## Features

- [x] abstraction layer for TA Gabriele 9009 serial port connection and the mission-critical low level commands;
- [x] typewriter's coordinate system and basic printing logic, including the carriage return;
- [x] printing of text files;
- [x] printing of composite UTF-8 characters which don't exist on the physical daisy wheel (like "é");
- [x] impression compensation;
- [ ] hot-swap daisy wheels;
- [ ] bidirectional printing;
- [ ] proportional characters;
- [ ] bold characters;
- [ ] command-line "native" typewriter mode;

## Connecting the typewriter to a computer

The typewriters data interface is based on UART specification. It was designed to communicate with a host computer
through vendor-specific interface box "IFD 1". 
 
The idea of the project is to connect our computer "directly" and drive the typewriter with its native 2-bytes 
commands. However, there are still some [hardware obstacles to overcome](/docs/README.md).

## Running the app

When the wires are connected correctly and the USB-UART adapter is recognised by the operating system,
it is found as `/dev/tty.usbserial-`. At this point switch on the typewriter and run the command.

It may fail for the first time run, due to the flow-control issues described below in "Known Issues".

```sh
# print from the STDIN (interactive mode):
cargo run -- --tty /dev/tty.usbserial-A10OFCFV
# print a text file:
cargo run -- --tty /dev/tty.usbserial-A10OFCFV --text welcome.txt
# or with debug output:
RUST_LOG=DEBUG cargo run -- --tty /dev/tty.usbserial-A10OFCFV
```

### Using directives in the interactive mode

`@>daisy`: This directive loads the daisy wheel data from a disk file.
The physical wheel at this point can be replaced with a new one.
This feature permits "how swapping" daisy wheels to use different fonts.
It is very useful when the layouts of typefaces are different. The parameter 
specifies a relative or absolute path to the new wheel data file.

```text
@>daisy wheels/German.toml
```

To quit the interactive mode just type `exit` and press return.


## External links to learn more

- [ST Computer article part 1](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)
- [ST Computer article part 2](https://www.stcarchiv.de/stc1988/08/gabriele-9009-2)
- [tweetwronger project](https://github.com/binraker/tweetwronger)
- [ST Computer article about IFD-1 interface box](https://www.stcarchiv.de/stc1986/07/schreibmaschine)

## License

This project is licensed under the following license:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])
