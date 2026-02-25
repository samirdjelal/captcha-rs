use captcha_rs::CaptchaBuilder;
use std::fs;
use std::path::Path;

fn main() {
    let images_dir = Path::new("images");
    if !images_dir.exists() {
        fs::create_dir(images_dir).expect("Failed to create images directory");
    }

    println!("Generating standard captchas...");
    
    // Standard Light Mode
    let light_standard = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .build();
    light_standard.image.save(images_dir.join("img-light-1.png")).unwrap();

    let light_standard2 = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .build();
    light_standard2.image.save(images_dir.join("img-light-2.png")).unwrap();

    let light_standard3 = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .build();
    light_standard3.image.save(images_dir.join("img-light-3.png")).unwrap();

    // Standard Dark Mode
    let dark_standard = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(true)
        .build();
    dark_standard.image.save(images_dir.join("img-dark-1.png")).unwrap();

    let dark_standard2 = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(true)
        .build();
    dark_standard2.image.save(images_dir.join("img-dark-2.png")).unwrap();

    let dark_standard3 = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(true)
        .build();
    dark_standard3.image.save(images_dir.join("img-dark-3.png")).unwrap();

    println!("Generating complexity level captchas...");

    // Complexity 1 to 10 for both Light and Dark modes
    for i in 1..=10 {
        // Light Mode Complexity
        let light_complex = CaptchaBuilder::new()
            .length(5)
            .width(130)
            .height(40)
            .dark_mode(false)
            .complexity(i)
            .build();
        light_complex.image.save(images_dir.join(format!("img-light-complexity-{}.png", i))).unwrap();

        // Dark Mode Complexity
        let dark_complex = CaptchaBuilder::new()
            .length(5)
            .width(130)
            .height(40)
            .dark_mode(true)
            .complexity(i)
            .build();
        dark_complex.image.save(images_dir.join(format!("img-dark-complexity-{}.png", i))).unwrap();
    }

    println!("Generating enhancement captchas (drop shadow & interference patterns)...");

    // Drop Shadow Example
    let shadow = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .drop_shadow(true)
        .build();
    shadow.image.save(images_dir.join("img-light-shadow.png")).unwrap();
    
    // Heavy Interference Example
    let heavy_interference = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .interference_lines(8)
        .interference_ellipses(6)
        .build();
    heavy_interference.image.save(images_dir.join("img-light-heavy-interference.png")).unwrap();

    println!("Generating bot deterrence (distortion) captchas...");

    // Distortion Example 1 (Mild)
    let mild_distortion = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .distortion(2)
        .build();
    mild_distortion.image.save(images_dir.join("img-light-distortion-mild.png")).unwrap();

    // Distortion Example 2 (Heavy)
    let heavy_distortion = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .distortion(5)
        .build();
    heavy_distortion.image.save(images_dir.join("img-light-distortion-heavy.png")).unwrap();

    println!("Done! Custom images generated and saved to the 'images' folder.");
}
