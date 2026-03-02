use captcha_rs::CaptchaBuilder;

#[test]
fn test_large_text() {
    let large_text = "A".repeat(10_000_000);
    let _ = CaptchaBuilder::new().text(large_text).build();
    println!("Done large text");
}
