#![doc(html_root_url = "https://docs.rs/captcha-rs/latest")]

//! Generate a verification image.
//!
//! ```rust
//! use captcha_rs::{Captcha, CaptchaBuilder};
//!
//! let dark_mode = true;
//! let text_length = 5;
//! let width = 130;
//! let height = 40;
//!
//! // generate a captcha with the given parameters
//! let captcha = Captcha::new(text_length, width, height, dark_mode);
//!
//! // generate a captcha using builder pattern
//! let captcha = CaptchaBuilder::new()
//! 	.length(text_length)
//! 	.width(width)
//! 	.height(height)
//! 	.dark_mode(dark_mode)
//! 	.build();
//!
//! println!("text: {}", captcha.text);
//! println!("base_img: {}", captcha.base_img);
//! ```

use crate::captcha::{get_captcha, get_captcha_img};

mod captcha;

pub struct Captcha {
	pub text: String,
	pub base_img: String,
	pub dark_mode: bool,
}

impl Captcha {
	pub fn new(length: usize, width: u32, height: u32, dark_mode: bool) -> Self {
		// Generate an array of captcha characters
		let res = get_captcha(length);
		let text = res.join("");
		// Generate an image based on the verification code character array and convert the image into a base 64 string
		let base_img = get_captcha_img(res, width, height, dark_mode);
		Captcha {
			text,
			base_img,
			dark_mode,
		}
	}
}

pub struct CaptchaBuilder {
	length: Option<usize>,
	width: Option<u32>,
	height: Option<u32>,
	dark_mode: Option<bool>,
}

impl CaptchaBuilder {
	pub fn new() -> Self {
		CaptchaBuilder {
			length: None,
			width: None,
			height: None,
			dark_mode: None,
		}
	}
	
	pub fn length(mut self, length: usize) -> Self {
		self.length = Some(length);
		self
	}
	
	pub fn width(mut self, width: u32) -> Self {
		self.width = Some(width);
		self
	}
	
	pub fn height(mut self, height: u32) -> Self {
		self.height = Some(height);
		self
	}
	
	pub fn dark_mode(mut self, dark_mode: bool) -> Self {
		self.dark_mode = Some(dark_mode);
		self
	}
	
	pub fn build(self) -> Captcha {
		let length = self.length.unwrap_or(5);
		let width = self.width.unwrap_or(130);
		let height = self.height.unwrap_or(40);
		let dark_mode = self.dark_mode.unwrap_or(false);
		Captcha::new(length, width, height, dark_mode)
	}
}

#[cfg(test)]
mod tests {
	use crate::{Captcha, CaptchaBuilder};
	
	#[test]
	fn it_generates_a_captcha() {
		let dark_mode = false;
		let text_length = 5;
		let width = 130;
		let height = 40;
		
		let start = std::time::Instant::now();
		
		let captcha = Captcha::new(text_length, width, height, dark_mode);
		
		let duration = start.elapsed();
		println!("Time elapsed in generating captcha() is: {:?}", duration);
		
		assert_eq!(captcha.text.len(), 5);
		
		let start_with = captcha.base_img.starts_with("data:image/png;base64,");
		assert_eq!(start_with, true);
		
		println!("text: {}", captcha.text);
		println!("base_img: {}", captcha.base_img);
	}
	
	#[test]
	fn it_generate_captcha_using_builder(){
		let start = std::time::Instant::now();
		let captcha = CaptchaBuilder::new()
			.length(5)
			.width(130)
			.height(40)
			.dark_mode(false)
			.build();
		
		let duration = start.elapsed();
		println!("Time elapsed in generating captcha() is: {:?}", duration);
		
		assert_eq!(captcha.text.len(), 5);
		
		let start_with = captcha.base_img.starts_with("data:image/png;base64,");
		assert_eq!(start_with, true);
	}
}
