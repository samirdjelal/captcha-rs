use captcha_rs::{CaptchaBuilder, verify};

fn main() {
    // A secret key used to sign the verification token.
    // In a real application, this should be stored securely (e.g., environment variables).
    let secret = "my-super-secret-key";

    println!("--- Captcha Stateless Verification Example ---");

    // 1. Generate a new captcha
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(200)
        .height(70)
        .dark_mode(false)
        .complexity(5)
        .build();

    println!("Generated Captcha Text: {}", captcha.text);

    // 2. Generate a verification token (stateless)
    // We'll set an expiration time of 5 minutes (300 seconds)
    // as_tuple returns (base64_image, token)
    let (image_base64, token) = captcha.as_tuple(secret, 300)
        .expect("Failed to generate stateless token. Make sure 'stateless' feature is enabled.");

    println!("Base64 Image (truncated): {}...", &image_base64[..50]);
    println!("Verification Token: {}", token);

    println!("\n--- Verification Scenario ---");

    // 3. Simulate user providing the correct solution
    let user_solution = captcha.text.to_lowercase(); // Captcha comparison is case-insensitive
    println!("User provides solution: {}", user_solution);

    let is_valid = verify(&token, &user_solution, secret)
        .expect("Token was invalid (could not be decoded or secret mismatch)");

    if is_valid {
        println!("✅ Success: Captcha verified successfully!");
    } else {
        println!("❌ Failure: Captcha verification failed!");
    }

    // 4. Simulate user providing an incorrect solution
    let wrong_solution = "wrong123";
    println!("\nUser provides wrong solution: {}", wrong_solution);

    let is_valid_wrong = verify(&token, wrong_solution, secret)
        .unwrap_or(false);

    if is_valid_wrong {
        println!("✅ Success: Captcha verified successfully!");
    } else {
        println!("❌ Failure: Captcha verification failed (expected)!");
    }

    // 5. Simulate verification with a different secret (should fail to decode)
    println!("\nVerifying with wrong secret...");
    let result_wrong_secret = verify(&token, &user_solution, "wrong-secret");
    
    match result_wrong_secret {
        Some(_) => println!("❌ Error: Should not have successfully decoded with wrong secret!"),
        None => println!("✅ Success: Failed to decode as expected due to secret mismatch."),
    }
}
