# Captcha-rs

[![Current Crates.io Version](https://img.shields.io/crates/v/captcha-rs.svg)](https://crates.io/crates/captcha-rs)

**captcha-rs** is a library that generate verification images dynamically.

Example pictures are as follows:


 ![img-light-1.png](images/img-light-1.png) | ![img-light-1.png](images/img-light-2.png) | ![img-light-1.png](images/img-light-2.png) 
   ---- | ----- | ------  
 ![img-dark-1.png](images/img-dark-1.png) | ![img-dark-1.png](images/img-dark-2.png) | ![img-dark-1.png](images/img-dark-2.png) 

## Example

Add the following dependency to the Cargo.toml file:

```toml
[dependencies]
captcha-rs = "0.1.1"
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