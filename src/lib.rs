use crate::captcha::{get_captcha, get_captcha_img};

mod captcha;

pub struct Captcha {
	pub text: String,
	pub base_img: String,
	pub dark_mode: bool,
}

impl Captcha {
	pub fn new(num: usize, width: u32, height: u32, dark_mode: bool) -> Self {
		// Generate an array of captcha characters
		let res = get_captcha(num);
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

#[cfg(test)]
mod tests {
	use crate::Captcha;
	
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
}

