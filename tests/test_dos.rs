use captcha_rs::CaptchaBuilder;

#[test]
fn test_dos() {
    let _ = CaptchaBuilder::new().length(100_000_000).build();
    println!("Done");
}
