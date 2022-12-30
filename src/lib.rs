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
//! let base_img = captcha.to_base64();
//! println!("base_img: {}", base_img);
//! ```
use image::{DynamicImage};
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};
use crate::captcha::{cyclic_write_character, draw_interference_ellipse, draw_interference_line, get_image, to_base64_str};

mod captcha;

pub struct Captcha {
	pub text: String,
	pub image: DynamicImage,
	pub dark_mode: bool,
}

impl Captcha {
	pub fn to_base64(&self) -> String {
		to_base64_str(&self.image)
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
		
		gaussian_noise_mut(&mut image, (complexity.clone() - 1) as f64, ((10 * complexity.clone()) - 10) as f64, ((5 * complexity.clone()) - 5) as u64);
		salt_and_pepper_noise_mut(&mut image, ((0.001 * complexity.clone() as f64) - 0.001) as f64, (0.5 * complexity.clone() as f64) as u64);
		
		Captcha {
			text,
			image: DynamicImage::ImageRgb8(image),
			dark_mode,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{CaptchaBuilder};
	
	#[test]
	fn it_generate_captcha_using_builder() {
		let start = std::time::Instant::now();
		let captcha = CaptchaBuilder::new()
			.length(5)
			.width(200)
			.height(70)
			.dark_mode(false)
			.complexity(5)
			.build();
		
		let duration = start.elapsed();
		println!("Time elapsed in generating captcha() is: {:?}", duration);
		
		assert_eq!(captcha.text.len(), 5);
		
		let base_img = captcha.to_base64();
		let start_with = base_img.starts_with("data:image/jpeg;base64,");
		assert_eq!(start_with, true);
		
		println!("text: {}", captcha.text);
		println!("base_img: {}", base_img);
	}
}
