# wincat-rs

A Windows port of the `cat` coreutils program.

wincat is designed to mimic the functionality of the `cat` command found in
Unix-like systems.

## Installation

The only dependencies are the the Rust compiler and cargo. The binary
is compiled only when using Windows targets:

``` console
$ cargo build --release
```

## Usage

``` console
.\wincat.exe <FILEs>
```

  - `<FILEs>`: The files whose contents will be concatenated and written to
    stdout.
  - `-v, --verbose`: Logs file operations and error handling where applicable.

## Contributing

Contributions are welcome! Here are the contributing guidelines:

  - Try to use the least amount of `unsafe` blocks. If that's needed make some
    safe wrapper function around it.
  - Since this repo is based around Windows, use functions from `winapi` when
    applicable. However, try keeping a tradeoff between safety and using the
    Windows API.
  - Prefer using the standard library over reinventing the wheel.
  - Format code with
    ``` console
    rustfmt --edition 2021
    ```
  - If you would like to make big changes (eg. changing libraries for
    checksums), open an issue, explaining what you'd like to change, the main
    reasons as to why.

### Roadmap

  - Feature: Add option for line numbers
  - Enhancement: Add a similar functionality as the `cat` Coreutils program (I'm
    not expecting full similarity as Windows and Unix-based OSes work very
    differently).

### Support

For help or issues, please [open an
issue](https://github.com/walker84837/wincat-rs/issues).

## License

This project is dual-licensed under the MIT or Apache 2.0 License - see the
[Apache, Version 2.0](LICENSE_APACHE.md) and [MIT License](LICENSE_MIT.md) files
for details.
