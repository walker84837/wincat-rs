# wincat-rs

A Windows port of the `cat` coreutils program.

wincat is designed to mimic the functionality of the `cat` command found in
Unix-like systems.

## Installation

No installation is required (except for the Rust compiler). Simply compile the
provided Rust code using cargo:

``` console
$ cargo build --release
```

The code compiles only for Windows targets if you're on Unix-like OSes (i.e.
macOS, Linux, FreeBSD)

## Usage

``` bash
.\cat-win.exe <input_files>
```

  - `<input_files>`: The path to the files whose contents will be displayed.

## Support

For help or issues, please [open an
issue](https://github.com/walker84837/wincat-rs/issues).

## Contributing

Contributions are welcome!

## License

This project is dual-licensed under the MIT or Apache 2.0 License - see the
[Apache, Version 2.0](LICENSE_APACHE.md) and [MIT License](LICENSE_MIT.md) files
for details.

## Roadmap

  - Feature: Add option for line numbers
  - Enhancement: Add a similar functionality as the `cat` coreutil program (I'm
    not expecting full similarity as Windows and Unix-based OSes work very
    differently).
