# captcha-rs

[![Captcha-rs crate](https://img.shields.io/crates/v/captcha-rs.svg)](https://crates.io/crates/captcha-rs)
[![CI](https://github.com/samirdjelal/captcha-rs/actions/workflows/CI.yml/badge.svg)](https://github.com/samirdjelal/captcha-rs/actions/workflows/CI.yml)
[![issues](https://img.shields.io/github/issues/samirdjelal/captcha-rs?color=%23ffc107)](https://github.com/samirdjelal/captcha-rs/issues)
[![Downloads](https://img.shields.io/crates/d/captcha-rs)](https://crates.io/crates/captcha-rs)
[![MIT License](https://img.shields.io/crates/l/captcha-rs)](LICENSE)
[![Captcha-rs documentation](https://img.shields.io/docsrs/captcha-rs)](https://docs.rs/captcha-rs)
[![dependency status](https://deps.rs/repo/github/samirdjelal/captcha-rs/status.svg)](https://deps.rs/repo/github/samirdjelal/captcha-rs)

**captcha-rs** is a library that generate verification images dynamically.

Example pictures are as follows:

![img-light-1.png](images/img-light-1.png) | ![img-light-2.png](images/img-light-2.png) | ![img-light-3.png](images/img-light-3.png) 
--- | --- | ---
![img-dark-1.png](images/img-dark-1.png) | ![img-dark-2.png](images/img-dark-2.png) | ![img-dark-3.png](images/img-dark-3.png) 

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha-rs = "0.1.9"
```

And then get started in your `main.rs`:

```rust
use captcha_rs::Captcha;

fn main() {
   
   let dark_mode = true;
   let text_length = 5;
   let width = 130;
   let height = 40;
   
   let captcha = Captcha::new(text_length, width, height, dark_mode);
   
   println!("text: {}", captcha.text);
   println!("base_img: {}", captcha.base_img);
   
}
```

### Run

```bash
# Dev
$ cargo run

# Build
$ cargo build

# Test
$ cargo test -- --nocapture
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `captcha-rs` by you, shall be licensed as MIT, without any additional
terms or conditions.
