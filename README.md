# Gabriele

100% 🦀 Rust command-line interface for TA Gabriele 9009 Typewriter.

## Context

This package provides a command-line tool for some electronic typewriters 
manufactured by (Triumph-Adler)[https://de.wikipedia.org/wiki/Triumph-Adler]  in 1980-1990.
It is developed initially for Gabriele 9009 model of typewriter, but may work with other models
which have a similar serial data interface.


These articles were published in one German magazine in July 1988 and served as motivation to research and create this software.
- [Article part 1](https://www.stcarchiv.de/stc1988/07/gabriele-9009-1)
- [Article part 2](https://www.stcarchiv.de/stc1988/08/gabriele-9009-2)

Second the authors of these articles, the machines like IBM Action Writer 6715 would also have similar features,
which opens a way to use them as [Daisy-wheel](https://en.m.wikipedia.org/wiki/File:Triumph-Adler_Daisy_wheel_Cubic_PS-92800.jpg) printer.



## How to use

Run in your console:
```sh
cargo run -- /dev/tty.usbserial-A10OFCFV
```


## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])