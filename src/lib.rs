#![doc(html_root_url = "https://docs.rs/captcha-rs/latest")]

//! Generate a verification image.
//!
//! ```rust
//! use captcha_rs::{CaptchaBuilder};
//!
//! let captcha = CaptchaBuilder::new()
//!     .length(5)
//!     .width(130)
//!     .height(40)
//!     .dark_mode(false)
//!     .complexity(1) // min: 1, max: 10
//!     .compression(40) // min: 1, max: 99
//!     .build();
//!
//! println!("text: {}", captcha.text);
//! let base_img = captcha.to_base64();
//! println!("base_img: {}", base_img);
//! ```
use image::DynamicImage;
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};
use rand::{rng, Rng};

use crate::captcha::{
    cyclic_write_character, draw_interference_ellipse, draw_interference_line, get_image,
    to_base64_str,
};

mod captcha;

pub struct Captcha {
    pub text: String,
    pub image: DynamicImage,
    pub compression: u8,
    pub dark_mode: bool,
}

impl Captcha {
    pub fn to_base64(&self) -> String {
        to_base64_str(&self.image, self.compression)
    }
}

#[derive(Default)]
pub struct CaptchaBuilder {
    text: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    dark_mode: Option<bool>,
    complexity: Option<u32>,
    compression: Option<u8>,
}

impl CaptchaBuilder {
    pub fn new() -> Self {
        CaptchaBuilder {
            text: None,
            width: None,
            height: None,
            dark_mode: None,
            complexity: None,
            compression: Some(40),
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        // Generate an array of captcha characters
        let length = length.max(1);
        let res = captcha::get_captcha(length);
        self.text = Some(res.join(""));
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

        if complexity > 10 {
            complexity = 10;
        }

        if complexity < 1 {
            complexity = 1;
        }

        self.complexity = Some(complexity);
        self
    }

    pub fn compression(mut self, compression: u8) -> Self {
        let compression = compression.clamp(1, 99);
        self.compression = Some(compression);
        self
    }

    pub fn build(self) -> Captcha {
        let text = match self.text {
            Some(text) if !text.is_empty() => text,
            _ => captcha::get_captcha(5).join(""),
        };

        let width = self.width.unwrap_or(130).clamp(30, 2000);
        let height = self.height.unwrap_or(40).clamp(20, 2000);
        let dark_mode = self.dark_mode.unwrap_or(false);
        let complexity = self.complexity.unwrap_or(1);
        let compression = self.compression.unwrap_or(40).clamp(1, 99);

        // Create a white background image
        let mut image = get_image(width, height, dark_mode);

        let res: Vec<String> = text.chars().map(|x| x.to_string()).collect();

        // Loop to write the verification code string into the background image
        cyclic_write_character(&res, &mut image, dark_mode);

        // Draw interference lines
        draw_interference_line(&mut image, dark_mode);
        draw_interference_line(&mut image, dark_mode);

        // Draw a distraction circle
        draw_interference_ellipse(2, &mut image, dark_mode);
        draw_interference_ellipse(2, &mut image, dark_mode);

        if complexity > 1 {
            let mut rng = rng();

            gaussian_noise_mut(
                &mut image,
                (complexity - 1) as f64,
                ((5 * complexity) - 5) as f64,
                rng.random::<u64>(),
            );

            salt_and_pepper_noise_mut(
                &mut image,
                (0.002 * complexity as f64) - 0.002,
                rng.random::<u64>(),
            );
        }

        Captcha {
            text,
            image: DynamicImage::ImageRgb8(image),
            compression,
            dark_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CaptchaBuilder;

    #[test]
    fn it_generates_a_captcha() {
        let _dark_mode = false;
        let _text_length = 5;
        let _width = 130;
        let _height = 40;

        let start = std::time::Instant::now();

        let captcha = CaptchaBuilder::new()
            .text(String::from("based"))
            .width(200)
            .height(70)
            .dark_mode(false)
            .build();

        let duration = start.elapsed();
        println!("Time elapsed in generating captcha() is: {:?}", duration);

        assert_eq!(captcha.text.len(), 5);
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
        println!("text: {}", captcha.text);
        println!("base_img: {}", base_img);
    }

    #[test]
    fn it_generates_captcha_using_builder() {
        let start = std::time::Instant::now();
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(200)
            .height(70)
            .dark_mode(false)
            .complexity(5)
            .compression(40)
            .build();

        let duration = start.elapsed();
        println!("Time elapsed in generating captcha() is: {:?}", duration);

        assert_eq!(captcha.text.len(), 5);
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
        println!("text: {}", captcha.text);
        println!("base_img: {}", base_img);
    }

    #[test]
    fn it_handles_empty_text_and_small_dimensions() {
        let captcha = CaptchaBuilder::new()
            .text(String::new())
            .width(10)
            .height(10)
            .compression(0)
            .build();

        assert!(!captcha.text.is_empty());
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
    }
}
