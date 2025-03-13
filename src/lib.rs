#![doc(html_root_url = "https://docs.rs/captcha-rs/latest")]

//! Generate a verification image.
//!
//! ```rust
//! use captcha_rs::{CaptchaBuilder};
//!
//! let captcha = CaptchaBuilder::new()
//!     .length(5) // min: 1, max: 16
//!     .width(130)
//!     .height(40)
//!     .dark_mode(false)
//!     .complexity(1) // min: 1, max: 10
//!     .compression(40) // min: 1, max: 99
//!     .build();
//!
//! println!("text: {}", captcha.text);
//! println!("base_img: {}", captcha.to_base64());
//! ```
use image::DynamicImage;
use imageproc::noise::{gaussian_noise_mut, salt_and_pepper_noise_mut};
use rand::Rng;

use crate::captcha::{
    cyclic_write_character, draw_interference_ellipse, draw_interference_line, get_image,
    to_base64_str, to_bytes, BASIC_CHAR,
};

mod captcha;

pub struct Captcha {
    pub text: String,
    // The image identifier should be set to be invisible externally
    image: DynamicImage,
    pub compression: u8,
    pub dark_mode: bool,
}

impl Captcha {
    /**
     * Convert image to JPEG bytes
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        to_bytes(&self.image, self.compression)
    }

    /**
     * Convert image to JPEG base64 string
     */
    pub fn to_base64(&self) -> String {
        to_base64_str(&self.image, self.compression)
    }
}

#[derive(Default, Clone, Copy)]
pub struct CaptchaBuilder<'a> {
    // The specified verification code string
    text: Option<&'a [u8]>,
    // The specified verification code character set
    base: Option<&'a [u8]>,
    // The length of the verification code
    length: usize,
    // The width of the verification code image
    width: u32,
    // The height of the verification code image
    height: u32,
    // Whether to use dark mode
    dark_mode: bool,
    // The complexity of the verification code image
    complexity: u32,
    // The compression rate of the verification code image
    compression: u8,
}

impl<'a> CaptchaBuilder<'a> {
    pub fn new() -> Self {
        CaptchaBuilder {
            text: None,
            base: Some(&BASIC_CHAR[..]),
            length: 5,
            width: 130,
            height: 40,
            dark_mode: false,
            complexity: 1,
            compression: 40,
        }
    }

    /// Specify the text of the verification code image
    ///
    /// params text - It must satisfy the text as ASCII code
    pub fn text(mut self, text: &'a str) -> Self {
        // Define the verification code characters.
        self.text = text.is_ascii().then(|| text.as_bytes());
        self
    }

    /// Specifies the character set that generates the verification code image
    ///
    /// params base - The character set needs to be satisfied that it is within the ASCII range
    /// otherwise the default character set will be used
    pub fn base(mut self, base: &'a str) -> Self {
        // Define verification code character set
        self.base = base.is_ascii().then(|| base.as_bytes());
        self
    }

    /// Set the verification code length
    ///
    /// params length - The length of the verification code is generated,
    /// and the recommended value is between 4 and 8.
    pub fn length(mut self, mut length: usize) -> Self {
        if length > 16 {
            length = 16;
        }
        if length == 0 {
            length = 1;
        }
        self.length = length;
        self
    }

    /// Set the width of the verification code image
    ///
    /// params width - The width of the verification code image,
    /// and the recommended value is between 100 and 500.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the height of the verification code image
    ///
    /// params height - The height of the verification code image,
    /// and the recommended value is between 30 and 500.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set the dark mode of the verification code image
    ///
    /// params dark_mode - Whether to use dark mode.
    pub fn dark_mode(mut self, dark_mode: bool) -> Self {
        self.dark_mode = dark_mode;
        self
    }

    /// Set the complexity of the verification code image
    ///
    /// params complexity - The complexity of the verification code image,
    /// and range is between 1-10
    pub fn complexity(mut self, mut complexity: u32) -> Self {
        if complexity > 10 {
            complexity = 10;
        }

        if complexity < 1 {
            complexity = 1;
        }

        self.complexity = complexity;
        self
    }

    /// Set the verification code image compression rate
    ///
    /// params compression - The compression rate of the verification code image,
    /// and range is between 1-100
    pub fn compression(mut self, mut compression: u8) -> Self {
        if compression > 100 {
            compression = 100;
        }
        if compression == 0 {
            compression = 1;
        }
        self.compression = compression;
        self
    }

    /// Build verification code image
    pub fn build(self) -> Captcha {
        // Change the build plan
        // If the text is specified, the generated verification code is the specified text
        // Otherwise, the verification code will be generated randomly
        let res = if let Some(text) = self.text {
            text.to_vec()
        } else {
            captcha::get_captcha(self.base, self.length)
        };

        // Create a white background image
        let mut image = get_image(self.width, self.height, self.dark_mode);

        // let res: Vec<String> = text.chars().map(|x| x.to_string()).collect();

        // Loop to write the verification code string into the background image
        cyclic_write_character(&res, &mut image, self.dark_mode);

        // Draw interference lines
        draw_interference_line(&mut image, self.dark_mode);
        draw_interference_line(&mut image, self.dark_mode);

        // Draw a distraction circle
        draw_interference_ellipse(2, &mut image, self.dark_mode);
        draw_interference_ellipse(2, &mut image, self.dark_mode);

        if self.complexity > 1 {
            let mut rng = rand::rng();

            gaussian_noise_mut(
                &mut image,
                (self.complexity - 1) as f64,
                ((5 * self.complexity) - 5) as f64,
                rng.random(),
            );

            salt_and_pepper_noise_mut(
                &mut image,
                (0.002 * self.complexity as f64) - 0.002,
                rng.random(),
            );
        }

        Captcha {
            text: unsafe { String::from_utf8_unchecked(res) },
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
            .text("based")
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
    fn it_generates_captcha_in_character_set() {
        let start = std::time::Instant::now();
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(200)
            .height(70)
            .dark_mode(false)
            .complexity(5)
            .compression(40)
            .base("0123456789")
            .build();
        let duration = start.elapsed();
        println!("Time elapsed in generating captcha() is: {:?}", duration);

        assert_eq!(captcha.text.len(), 5);
        let base_img = captcha.to_base64();
        assert!(base_img.starts_with("data:image/jpeg;base64,"));
        println!("text: {}", captcha.text);
        println!("base_img: {}", base_img);
    }
}
