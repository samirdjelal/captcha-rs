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


## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha-rs = "0.2.8"
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

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `captcha-rs` by you, shall be licensed as MIT, without any additional terms or conditions.
