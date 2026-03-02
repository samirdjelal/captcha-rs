use captcha_rs::CaptchaBuilder;

fn main() {
    // 1. Read custom TTF font file from disk into a byte array
    // Here we use the embedded fallback font as an example, but you can point it to any valid .ttf file.
    let custom_font = include_bytes!("../fonts/arial.ttf");

    // 2. Initialize CaptchaBuilder and pass the custom font slice
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .font(custom_font) // Set the custom TTF font here
        .build();

    println!("text: {}", captcha.text);
    println!("base_img: {}", captcha.to_base64());
}
