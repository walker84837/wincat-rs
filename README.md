# cat-win

A Windows port of the `cat` coreutils program.

`cat-win` is a command-line utility for Windows that is designed to mimic the functionality of the `cat` command found in Unix-like systems. It allows users to concatenate and display the contents of one or more files to the standard output.

## Installation

No installation is required (except for the Rust compiler). Simply compile the provided Rust code using cargo: `cargo build --release`. If you get an error while compiling on non-Windows OSes, it's because the program uses the `winapi` crate, so it compiles only for Windows, so specify `--target x86_64-pc-windows-gnu` if you're on Unix-like OSes (like macOS, Linux, FreeBSD, ...), like `cargo build --release --target x86_64-pc-windows-gnu`

## Usage

```bash
.\cat-win.exe <input_files>
```

- `<input_files>`: The path to the files whose contents will be displayed. Must be at least one, or else the help page will be printed.

## Examples

```bash
.\cat-win.exe index.html style.css app.js
```

```bash
.\cat-win.exe error.txt
```

## Support

For help or issues, please [open an issue](https://github.com/walker84837/cat-win-rs/issues).

## Contributing

Contributions are welcome! Please follow the guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).

## Authors and Acknowledgment

- Sole mantainer - [walker84837](https://github.com/walker84837)

## License

This project is dual-licensed under the MIT or Apache 2.0 License - see the [Apache, Version 2.0](LICENSE_APACHE.md) and [MIT License](LICENSE_MIT.md) files for details.

## Project Status

Development is not very active, so contributions are encouraged.

## Roadmap

- Feature: Add option for line numbers
- Enhancement: Add a similar functionality as the `cat` coreutil program (I'm not expecting full similarity as Windows and Unix-based OSes work very differently).
