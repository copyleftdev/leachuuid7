# leachuuid7

A UUIDv7 generator written in Rust, conforming 100% to the proposed UUID-7 specification.  
This crate generates UUIDs using the current Unix timestamp in milliseconds, a version number of 7, the binary variant `10`, and 62 bits of randomness.

## Overview

UUIDv7 is designed with high concurrency in mind and provides an effective means of generating unique identifiers in distributed systems and asynchronous environments.

**UUID layout (128 bits total):**
- **60 bits:** Unix timestamp in milliseconds (since the Unix epoch)
- **4 bits:** Version (always 7)
- **2 bits:** Variant (always binary `10`)
- **62 bits:** Random

The resulting UUID is formatted in the canonical form: `8-4-4-4-12` hexadecimal digits.

## Features

- **Standards-Compliant:** Fully conforms to the UUIDv7 specification.
- **High Concurrency:** Uses thread-local RNG for safe and efficient generation in multi-threaded environments.
- **Custom RNG Support:** Option to inject a custom RNG for greater control over randomness.
- **Robust Parsing:** Validates UUID format, version, and variant.
- **Comprehensive Testing:** Includes unit tests for formatting, parsing, and uniqueness.

## Installation

Add `leachuuid7` to your `Cargo.toml`:

```toml
[dependencies]
leachuuid7 = "0.1.0"
```

## Usage

Generate a new UUIDv7:

```rust
use leachuuid7::Uuid7;

fn main() {
    let uuid = Uuid7::new();
    println!("Generated UUIDv7: {}", uuid);
}
```

Parse a UUIDv7 string:

```rust
use leachuuid7::Uuid7;
use std::str::FromStr;

fn main() {
    let uuid_str = "0184e1a0-7e2a-7d40-8f3b-5c1a2b3c4d5e";
    let uuid = Uuid7::from_str(uuid_str).expect("Invalid UUIDv7 string");
    println!("Parsed UUIDv7: {}", uuid);
}
```

## Testing

Run the tests with:

```bash
cargo test
```

## Continuous Integration

This project uses GitHub Actions for continuous integration. The CI workflow performs:

- Code formatting checks with `cargo fmt`
- Linting with `cargo clippy`
- Running tests with `cargo test`
- Building in release mode with `cargo build --release`
- Packaging the crate with `cargo package`

See [`.github/workflows/ci.yml`](.github/workflows/ci.yml) for more details.

## Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check the [issues page](https://github.com/copyleftdev/leachuuid7/issues) if you have any questions or suggestions.

## Repository

The repository for this project is hosted on GitHub:  
[https://github.com/copyleftdev/leachuuid7](https://github.com/copyleftdev/leachuuid7)

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

A special thanks to P. Leach and R. Salz, pioneers in UUID development, for inspiring the naming of this crate.
