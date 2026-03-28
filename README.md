# facilguide

Multilingual tech guide utilities for Rust. Access guides in EN, ES, FR, PT, and IT.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
facilguide = "0.1.0"
```

## Usage

```rust
use facilguide::{VERSION, BASE_URL};

fn main() {
    println!("FacilGuide v{}", VERSION);
    println!("Homepage: {}", BASE_URL);
}
```

## Links

- **Homepage**: [https://facil.guide](https://facil.guide)
- **English Guides**: [https://facil.guide/en/](https://facil.guide/en/)
- **Repository**: [https://github.com/arnaudleroy-studio/facilguide-rust](https://github.com/arnaudleroy-studio/facilguide-rust)

## License

MIT - See [LICENSE](LICENSE) for details.
