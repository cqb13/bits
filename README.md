# Bits

A CLI tool for displaying the size of files and directories.

## Installation

### Build from Source

Clone the repository and build the project using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```sh
git clone https://github.com/cqb13/bits.git
cd bits
cargo build --release
# The binary will be located at target/release/bits
```

To add the binary to your PATH, run:

```sh
cargo install --path .
```

### Pre-built Binaries

Pre-built binaries are available for Windows, macOS, and Linux on the [releases page](https://github.com/cqb13/ti-tools/releases).

## Usage

```sh
    bits [COMMAND] [OPTIONS]
```

## Usage

### File Size

```
bits ./file-path.txt
```

### Directory Size

```
bits ./directory
```

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
