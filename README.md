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

As far as I have seen, the code only compiles if you're building for Windows
targets, hence why I set up the GitHub CI to use Windows.

## Usage

``` console
.\wincat.exe <FILEs>
```

  - `<FILEs>`: The files whose contents will be concatenated and written to
    stdout.

## Contributing

Contributions are welcome! Here are the contributing guidelines (I will try to
keep them as simple as possible):

  - Try to use the least amount of `unsafe` blocks. If that's needed make some
    safe wrapper function around it.
  - I recommend you to use Windows functions (like `winapi` or
    `std::os::windows`) if performance will be better. However, try to keep a
    tradeoff between safety and using the Windows API.
  - Prefer using the standard library over reinventing the wheel.
  - Format code with
    ``` console
    rustfmt --edition 2021
    ```
  - If you would like to make big changes (eg. changing libraries for
    checksums), open an issue, explaining what you'd like to change, the main
    reasons as to why, and what is the difference between using it or not.

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
