# captcha-rs

[![Captcha-rs crate](https://img.shields.io/crates/v/captcha-rs.svg?style=flat&logo=appveyor)](https://crates.io/crates/captcha-rs)
[![test](https://github.com/samirdjelal/captcha-rs/actions/workflows/test.yml/badge.svg?style=flat&logo=appveyor)](https://github.com/samirdjelal/captcha-rs/actions/workflows/test.yml)
[![issues](https://img.shields.io/github/issues/samirdjelal/captcha-rs?color=%23ffc107&style=flat&logo=appveyor)](https://github.com/samirdjelal/captcha-rs/issues)
[![Downloads](https://img.shields.io/crates/d/captcha-rs?style=flat&logo=appveyor)](https://crates.io/crates/captcha-rs)
[![MIT License](https://img.shields.io/crates/l/captcha-rs?style=flat&logo=appveyor)](LICENSE)
[![Captcha-rs documentation](https://img.shields.io/docsrs/captcha-rs?style=flat&logo=appveyor)](https://docs.rs/captcha-rs)
[![dependency status](https://deps.rs/repo/github/samirdjelal/captcha-rs/status.svg?style=flat&logo=appveyor)](https://deps.rs/repo/github/samirdjelal/captcha-rs)

**captcha-rs** is a library that generate verification images dynamically.

Example pictures are as follows:

![img-light-1.png](images/img-light-1.png) | ![img-light-2.png](images/img-light-2.png) | ![img-light-3.png](images/img-light-3.png)
--- | --- | ---
![img-dark-1.png](images/img-dark-1.png) | ![img-dark-2.png](images/img-dark-2.png) | ![img-dark-3.png](images/img-dark-3.png)

### Using complexity method

Complexity | Light Mode / Noise Filter                        | Dark Mode / Noise Filter
--- |--------------------------------------------------| ---
Level 1 | ![img-light](images/img-light-complexity-1.png)  | ![img-dark](images/img-dark-complexity-1.png)
Level 2 | ![img-light](images/img-light-complexity-2.png)  | ![img-dark](images/img-dark-complexity-2.png)
Level 3 | ![img-light](images/img-light-complexity-3.png)  | ![img-dark](images/img-dark-complexity-3.png)
Level 4 | ![img-light](images/img-light-complexity-4.png)  | ![img-dark](images/img-dark-complexity-4.png)
Level 5 | ![img-light](images/img-light-complexity-5.png)  | ![img-dark](images/img-dark-complexity-5.png)
Level 6 | ![img-light](images/img-light-complexity-6.png)  | ![img-dark](images/img-dark-complexity-6.png)
Level 7 | ![img-light](images/img-light-complexity-7.png)  | ![img-dark](images/img-dark-complexity-7.png)
Level 8 | ![img-light](images/img-light-complexity-8.png)  | ![img-dark](images/img-dark-complexity-8.png)
Level 9 | ![img-light](images/img-light-complexity-9.png)  | ![img-dark](images/img-dark-complexity-9.png)
Level 10 | ![img-light](images/img-light-complexity-10.png) | ![img-dark](images/img-dark-complexity-10.png)

### Using Visual Enhancements and Bot Deterrence

Effect | Image example
--- | ---
Drop Shadow | ![img-light-shadow](images/img-light-shadow.png)
Heavy Interference | ![img-light-heavy-interference](images/img-light-heavy-interference.png)
Mild Distortion | ![img-light-distortion-mild](images/img-light-distortion-mild.png)
Heavy Distortion | ![img-light-distortion-heavy](images/img-light-distortion-heavy.png)

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha-rs = "0.3.0"
```

And then get started in your `main.rs`:

```rust
use captcha_rs::CaptchaBuilder;

fn main() {
	
	let captcha = CaptchaBuilder::new()
		.length(5)
		.width(130)
		.height(40)
		.dark_mode(false)
		.complexity(1) // min: 1, max: 10
		.compression(40) // min: 1, max: 99
		.drop_shadow(false) // Adds a drop shadow to the text
		.interference_lines(2) // Number of interference lines (min 0)
		.interference_ellipses(2) // Number of distraction circles (min 0)
		.distortion(0) // Level of wavy distortion grid (min 0)
		.build();
	
	println!("text: {}", captcha.text);
	println!("base_img: {}", captcha.to_base64());
	
}
```

### Run

```bash
# Dev
💲 cargo run

# Build
💲 cargo build

# Test
💲 cargo test -- --nocapture
```

## Performance

The library is highly optimized for fast image generation. The table below represents benchmarks tested with varying configurations (measured per image):

| Configuration | Time per Image | Core Settings |
| --- | --- | --- |
| **Default** | `~53 µs` | length=5, 130x40 |
| **High Complexity** | `~439 µs` | length=5, 200x70, complexity=10 |
| **High Distortion** | `~160 µs` | length=5, 200x70, distortion=15 |
| **Extreme Security** | `~1.16 ms` | length=8, 300x100, complexity=10, distortion=20, drop_shadow, high interference |

*Note: Benchmarks run on a single thread using `cargo bench`. Performance will vary based on hardware, but standard generation should easily exceed thousands of images per second.*

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `captcha-rs` by you, shall be licensed as MIT, without any additional terms or conditions.
