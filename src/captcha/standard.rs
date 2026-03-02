use ab_glyph::FontArc;
use base64::Engine;
use base64::engine::general_purpose;
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_cubic_bezier_curve_mut, draw_hollow_ellipse_mut, draw_text_mut};
use rand::{Rng, rng};
use std::io::Cursor;
use std::sync::OnceLock;

// ==========================================
// CONSTANTS
// ==========================================

/// Define the verification code characters.
/// Remove 0, O, I, L and other easily confusing letters.
pub const BASIC_CHAR: [char; 54] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M',
    'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Define background colors for light and dark modes.
pub const LIGHT: [u8; 3] = [224, 238, 253];
pub const DARK: [u8; 3] = [18, 18, 18];

/// Define random text colors for light mode.
pub const LIGHT_BASIC_COLOR: [[u8; 3]; 5] = [
    [214, 14, 50],
    [240, 181, 41],
    [176, 203, 40],
    [105, 137, 194],
    [242, 140, 71],
];

/// Define random text colors for dark mode.
pub const DARK_BASIC_COLOR: [[u8; 3]; 5] = [
    [251, 188, 5],
    [116, 192, 255],
    [255, 224, 133],
    [198, 215, 97],
    [247, 185, 168],
];

/// Define font sizes.
pub const SCALE_SM: f32 = 35.0;
pub const SCALE_MD: f32 = 42.0;
pub const SCALE_LG: f32 = 50.0;

// ==========================================
// UTILITIES (RNG & MATH)
// ==========================================

/// Generate a random number up to `num` (inclusive).
pub fn get_rnd(num: usize) -> usize {
    let mut rng = rng();
    rng.random_range(0..=num)
}

/// Generate a random float between `min` and `max`.
pub fn get_next(min: f32, max: u32) -> f32 {
    if (max as f32) <= min {
        return min;
    }
    let mut rng = rng();
    rng.random_range(min..=(max as f32))
}

// ==========================================
// CAPTCHA GENERATION & CONFIGURATION
// ==========================================

/// Generate an array of captcha characters from the given character set.
///
/// `num` specifies the number of digits/characters in the verification code.
pub fn get_captcha(num: usize, chars: &[char]) -> Vec<String> {
    let mut res = vec![];
    let chars = if chars.is_empty() { &BASIC_CHAR } else { chars };
    let max_idx = chars.len() - 1;
    for _ in 0..num {
        let rnd = get_rnd(max_idx);
        res.push(chars[rnd].to_string())
    }
    res
}

/// Get random color depending on the dark/light mode.
pub fn get_color(dark_mode: bool) -> Rgb<u8> {
    let rnd = get_rnd(4);
    if dark_mode {
        return Rgb(DARK_BASIC_COLOR[rnd]);
    }
    Rgb(LIGHT_BASIC_COLOR[rnd])
}

static FONT: OnceLock<FontArc> = OnceLock::new();

/// Get the captcha font from the embedded TTF file.
pub fn get_font() -> FontArc {
    FONT.get_or_init(|| {
        let font = Vec::from(include_bytes!("../../fonts/arial.ttf") as &[u8]);
        FontArc::try_from_vec(font).unwrap()
    })
    .clone()
}

/// Get an initialized image buffer with the appropriate background color.
pub fn get_image(width: u32, height: u32, dark_mode: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |_, _| {
        if dark_mode {
            return image::Rgb(DARK);
        }
        image::Rgb(LIGHT)
    })
}

// ==========================================
// DRAWING ROUTINES
// ==========================================

/// Write the captcha characters on the background image in a layout.
pub fn cyclic_write_character(
    res: &[String],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    dark_mode: bool,
    drop_shadow: bool,
) {
    if res.is_empty() {
        return;
    }

    let usable_width = image.width().saturating_sub(10);
    let c = usable_width / res.len() as u32;
    let y = (image.height() / 2).saturating_sub(15);

    let scale = match res.len() {
        1..=3 => SCALE_LG,
        4..=5 => SCALE_MD,
        _ => SCALE_SM,
    };

    let font = get_font();

    for (i, _) in res.iter().enumerate() {
        let text = &res[i];
        let color = get_color(dark_mode);
        let x = 5 + (i as u32 * c) as i32;

        if drop_shadow {
            // Draw shadow slightly offset and dark
            draw_text_mut(
                image,
                Rgb([20, 20, 20]), // Dark shadow color
                x + 2,
                y as i32 + 2,
                scale,
                &font,
                text,
            );
        }

        draw_text_mut(image, color, x, y as i32, scale, &font, text);
    }
}

/// Draw a random interference line (bezier curve) on the background picture.
pub fn draw_interference_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, dark_mode: bool) {
    let width = image.width();
    let height = image.height();
    if width <= 5 || height <= 5 {
        return;
    }

    let x1: f32 = 5.0;
    let y1 = get_next(x1, height / 2);

    let x2 = width.saturating_sub(5) as f32;
    let y2 = get_next((height / 2) as f32, height.saturating_sub(5));

    let ctrl_x = get_next((width / 4) as f32, width / 4 * 3);
    let ctrl_y = get_next(x1, height - 5);

    let ctrl_x2 = get_next((width / 4) as f32, width / 4 * 3);
    let ctrl_y2 = get_next(x1, height - 5);

    // Randomly draw bezier curves
    draw_cubic_bezier_curve_mut(
        image,
        (x1, y1),
        (x2, y2),
        (ctrl_x, ctrl_y),
        (ctrl_x2, ctrl_y2),
        get_color(dark_mode),
    );
}

/// Draw a distraction circle (hollow ellipse) in random positions.
pub fn draw_interference_ellipse(
    num: usize,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    dark_mode: bool,
) {
    if image.width() <= 25 || image.height() <= 15 {
        return;
    }
    for _ in 0..num {
        let w = (10 + get_rnd(5)) as i32;
        let x = get_rnd((image.width() - 25) as usize) as i32;
        let y = get_rnd((image.height() - 15) as usize) as i32;
        draw_hollow_ellipse_mut(image, (x, y), w, w, get_color(dark_mode));
    }
}

// ==========================================
// EFFECTS
// ==========================================

/// Apply wavy pixel-level distortion to the image to deter OCR bots.
pub fn apply_wavy_distortion(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, level: u32) {
    if level == 0 {
        return;
    }

    let width = image.width();
    let height = image.height();
    let mut new_image = image.clone();
    let mut rng = rng();

    // Randomize the wave phase and frequency slightly
    let phase: f32 = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    // Amplitude is related to distortion level, capped for readability
    let amplitude = (level as f32) * 1.5;
    let frequency = 0.05 + (rng.random_range(0.0..0.05) * level as f32);

    for y in 0..height {
        for x in 0..width {
            // Calculate pixel displacement using a sine wave
            let offset_x = (amplitude * ((y as f32 * frequency) + phase).sin()) as i32;
            let offset_y = (amplitude * ((x as f32 * frequency) + phase).cos()) as i32;

            let src_x = (x as i32 + offset_x).clamp(0, width as i32 - 1) as u32;
            let src_y = (y as i32 + offset_y).clamp(0, height as i32 - 1) as u32;

            // Map the source pixel to the destination
            let pixel = image.get_pixel(src_x, src_y);
            new_image.put_pixel(x, y, *pixel);
        }
    }

    *image = new_image;
}

// ==========================================
// EXPORT & CONVERSION
// ==========================================

/// Convert a `DynamicImage` to a JPEG base64 Data URI string.
pub fn to_base64_str(image: &DynamicImage, compression: u8) -> String {
    let mut buf = Cursor::new(Vec::new());
    let mut encoder = JpegEncoder::new_with_quality(&mut buf, compression);
    if encoder.encode_image(image).is_err() {
        return "data:image/jpeg;base64,".to_string();
    }
    let res_base64 = general_purpose::STANDARD.encode(buf.into_inner());
    format!("data:image/jpeg;base64,{}", res_base64)
}
