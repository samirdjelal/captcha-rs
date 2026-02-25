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
    length: usize,
    characters: Vec<char>,
    width: u32,
    height: u32,
    dark_mode: bool,
    complexity: u32,
    compression: u8,
    drop_shadow: bool,
    interference_lines: usize,
    interference_ellipses: usize,
    distortion: u32,
}

impl CaptchaBuilder {
    pub fn new() -> Self {
        CaptchaBuilder {
            text: None,
            length: 5,
            characters: captcha::BASIC_CHAR.to_vec(),
            width: 130,
            height: 40,
            dark_mode: false,
            complexity: 1,
            compression: 40,
            drop_shadow: false,
            interference_lines: 2,
            interference_ellipses: 2,
            distortion: 0,
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length.max(1);
        self
    }

    pub fn chars(mut self, chars: Vec<char>) -> Self {
        self.characters = chars;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width.clamp(30, 2000);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height.clamp(20, 2000);
        self
    }

    pub fn dark_mode(mut self, dark_mode: bool) -> Self {
        self.dark_mode = dark_mode;
        self
    }

    pub fn complexity(mut self, complexity: u32) -> Self {
        self.complexity = complexity.clamp(1, 10);
        self
    }

    pub fn compression(mut self, compression: u8) -> Self {
        self.compression = compression.clamp(1, 99);
        self
    }

    pub fn drop_shadow(mut self, drop_shadow: bool) -> Self {
        self.drop_shadow = drop_shadow;
        self
    }

    pub fn interference_lines(mut self, lines: usize) -> Self {
        self.interference_lines = lines;
        self
    }

    pub fn interference_ellipses(mut self, ellipses: usize) -> Self {
        self.interference_ellipses = ellipses;
        self
    }

    pub fn distortion(mut self, distortion: u32) -> Self {
        self.distortion = distortion;
        self
    }

    pub fn build(self) -> Captcha {
        let text = match self.text {
            Some(t) if !t.is_empty() => t,
            _ => captcha::get_captcha(self.length, &self.characters).join(""),
        };

        // Create a background image
        let mut image = get_image(self.width, self.height, self.dark_mode);

        let res: Vec<String> = text.chars().map(|x| x.to_string()).collect();

        // Loop to write the verification code string into the background image
        cyclic_write_character(&res, &mut image, self.dark_mode, self.drop_shadow);

        if self.distortion > 0 {
            captcha::apply_wavy_distortion(&mut image, self.distortion);
        }

        // Draw interference lines
        for _ in 0..self.interference_lines {
            draw_interference_line(&mut image, self.dark_mode);
        }

        // Draw distraction circles
        draw_interference_ellipse(self.interference_ellipses, &mut image, self.dark_mode);

        if self.complexity > 1 {
            let mut rng = rng();

            gaussian_noise_mut(
                &mut image,
                (self.complexity - 1) as f64,
                ((5 * self.complexity) - 5) as f64,
                rng.random::<u64>(),
            );

            salt_and_pepper_noise_mut(
                &mut image,
                (0.002 * self.complexity as f64) - 0.002,
                rng.random::<u64>(),
            );
        }

        Captcha {
            text,
            image: DynamicImage::ImageRgb8(image),
            compression: self.compression,
            dark_mode: self.dark_mode,
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

    #[test]
    fn it_generates_captcha_with_distortion() {
        let captcha = CaptchaBuilder::new()
            .text(String::from("wavy"))
            .width(200)
            .height(70)
            .distortion(5)
            .build();

        assert_eq!(captcha.text, "wavy");
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn it_generates_captcha_with_custom_interference_and_shadow() {
        let captcha = CaptchaBuilder::new()
            .text(String::from("shadow"))
            .width(200)
            .height(70)
            .drop_shadow(true)
            .interference_lines(5)
            .interference_ellipses(5)
            .build();

        assert_eq!(captcha.text, "shadow");
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn it_generates_captcha_with_custom_characters() {
        let captcha = CaptchaBuilder::new()
            .chars(vec!['A', 'B'])
            .length(10)
            .width(200)
            .height(70)
            .build();

        assert_eq!(captcha.text.len(), 10);
        assert!(captcha.text.chars().all(|c| c == 'A' || c == 'B'));
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
    }
}
