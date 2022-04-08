#![doc(html_root_url = "https://docs.rs/captcha-rs/latest")]

//! Generate a verification image.
//!
//! ```rust
//! use captcha_rs::{CaptchaBuilder};
//!
//! let captcha = CaptchaBuilder::new()
//! 	.length(5)
//! 	.width(130)
//! 	.height(40)
//! 	.dark_mode(false)
//! 	.complexity(1) // min: 1, max: 10
//! 	.build();
//!
//! println!("text: {}", captcha.text);
//! println!("base_img: {}", captcha.base_img);
//! ```
use imageproc::noise::{gaussian_noise_mut};
use crate::captcha::{cyclic_write_character, draw_interference_ellipse, draw_interference_line, get_image, to_base64_str};

mod captcha;

pub struct Captcha {
	pub text: String,
	pub base_img: String,
	pub dark_mode: bool,
}

impl Captcha {
	pub fn new(length: usize, width: u32, height: u32, dark_mode: bool) -> Self {
		// Generate an array of captcha characters
		let res = captcha::get_captcha(length);
		let text = res.join("");
		// Generate an image based on the verification code character array and convert the image into a base 64 string
		let base_img = captcha::get_captcha_img(res, width, height, dark_mode);
		
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
	complexity: Option<u32>,
}

impl CaptchaBuilder {
	pub fn new() -> Self {
		CaptchaBuilder {
			length: None,
			width: None,
			height: None,
			dark_mode: None,
			complexity: None,
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
	
	pub fn complexity(mut self, complexity: u32) -> Self {
		let mut complexity = complexity;
		if complexity > 10 { complexity = 10; }
		if complexity < 1 { complexity = 1; }
		self.complexity = Some(complexity);
		self
	}
	
	pub fn build(self) -> Captcha {
		let length = self.length.unwrap_or(5);
		let width = self.width.unwrap_or(130);
		let height = self.height.unwrap_or(40);
		let dark_mode = self.dark_mode.unwrap_or(false);
		let complexity = self.complexity.unwrap_or(1);
		
		// Generate an array of captcha characters
		let res = captcha::get_captcha(length);
		
		let text = res.join("");
		
		// Create a white background image
		let mut image = get_image(width, height, dark_mode);
		
		// Loop to write the verification code string into the background image
		cyclic_write_character(&res, &mut image, dark_mode);
		
		// Draw interference lines
		draw_interference_line(&mut image, dark_mode);
		draw_interference_line(&mut image, dark_mode);
		
		// Draw a distraction circle
		draw_interference_ellipse(2, &mut image, dark_mode);
		draw_interference_ellipse(2, &mut image, dark_mode);
		
		gaussian_noise_mut(&mut image, (complexity.clone() - 1) as f64, ((10 * complexity.clone()) - 10) as f64, ((10 * complexity.clone()) - 10) as u64);
		
		// Convert to base 64 string
		let base_img = to_base64_str(image);
		
		Captcha {
			text,
			base_img,
			dark_mode,
		}
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
	fn it_generate_captcha_using_builder() {
		let start = std::time::Instant::now();
		let captcha = CaptchaBuilder::new()
			.length(5)
			.width(200)
			.height(70)
			.dark_mode(false)
			.complexity(1)
			.build();
		
		let duration = start.elapsed();
		println!("Time elapsed in generating captcha() is: {:?}", duration);
		
		assert_eq!(captcha.text.len(), 5);
		
		let start_with = captcha.base_img.starts_with("data:image/png;base64,");
		assert_eq!(start_with, true);
		
		println!("text: {}", captcha.text);
		println!("base_img: {}", captcha.base_img);
	}
}
