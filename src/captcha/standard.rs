use ab_glyph::{FontVec, PxScale};
use base64::{engine::general_purpose, Engine};
use image::{codecs::jpeg::JpegEncoder, DynamicImage, ImageBuffer, Rgb};
use imageproc::drawing::{draw_cubic_bezier_curve_mut, draw_hollow_ellipse_mut, draw_text_mut};
use rand::{rng, Rng};
use std::{io::Cursor, mem::transmute};

// Define the verification code characters.
// Remove 0, O, I, L and other easily confusing letters
pub const BASIC_CHAR: [u8; 54] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
    b'J', b'K', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a',
    b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'j', b'k', b'm', b'n', b'p', b'q', b'r', b's', b't',
    b'u', b'v', b'w', b'x', b'y', b'z',
];

// Define a random color for a string
pub const LIGHT_BASIC_COLOR: [[u8; 3]; 5] = [
    [214, 14, 50],
    [240, 181, 41],
    [176, 203, 40],
    [105, 137, 194],
    [242, 140, 71],
];

pub const DARK_BASIC_COLOR: [[u8; 3]; 5] = [
    [251, 188, 5],
    [116, 192, 255],
    [255, 224, 133],
    [198, 215, 97],
    [247, 185, 168],
];

// Define background color
pub const LIGHT: [u8; 3] = [224, 238, 253];
pub const DARK: [u8; 3] = [18, 18, 18];

// Define font size
pub const SCALE_SM: PxScale = PxScale { x: 38.0, y: 35.0 };
pub const SCALE_MD: PxScale = PxScale { x: 45.0, y: 42.0 };
pub const SCALE_LG: PxScale = PxScale { x: 53.0, y: 50.0 };

/***
 * Generate random numbers
 * params num - maximum random number
 */
pub fn get_rnd(num: usize) -> usize {
    let mut rng = rng();
    rng.random_range(0..=num)
}

/**
 * Generate an array of captcha characters
 * params base   - Verify the code character set and the character set is within the ASCII range
 * params length - length between 1-16
 */
pub fn get_captcha(base: Option<&[u8]>, length: usize) -> Vec<u8> {
    let base = if let Some(base) = base {
        base
    } else {
        &BASIC_CHAR[..]
    };
    let mut res = vec![];
    let mut rng = rng();
    for _ in 0..length {
        let rng = rng.random_range(0..base.len());
        res.push(*unsafe { base.get_unchecked(rng) });
    }
    res
}

/**
 * Get color
 */
pub fn get_color(dark_mode: bool) -> Rgb<u8> {
    let rnd = get_rnd(4);
    if dark_mode {
        return Rgb(DARK_BASIC_COLOR[rnd]);
    }
    Rgb(LIGHT_BASIC_COLOR[rnd])
}

/**
 * Generate random numbers between two numbers
 * params min – minimum
 *        max – maximum value
 * return: random number
 */
pub fn get_next(min: f32, max: u32) -> f32 {
    min + get_rnd(max as usize - min as usize) as f32
}

/**
 * Get font
 */
pub fn get_font() -> FontVec {
    let font = Vec::from(include_bytes!("../../fonts/arial.ttf") as &[u8]);
    FontVec::try_from_vec(font).unwrap()
}

/**
 * Get an image with a white background
 */
pub fn get_image(width: u32, height: u32, dark_mode: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |_, _| {
        if dark_mode {
            return image::Rgb(DARK);
        }
        image::Rgb(LIGHT)
    })
}

/**
 * Loop to write captcha characters on background image
 * params res    - Array of verification code characters to be written
 *        image  - Background picture
 */
pub fn cyclic_write_character(
    res: &[u8],
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    dark_mode: bool,
) {
    let c = (image.width() - 10) / res.len() as u32;
    let y = image.height() / 2 - 15;

    let scale = match res.len() {
        1..=3 => SCALE_LG,
        4..=5 => SCALE_MD,
        _ => SCALE_SM,
    };

    for (i, _) in res.iter().enumerate() {
        let text = &[res[i]][..];

        draw_text_mut(
            image,
            get_color(dark_mode),
            5 + (i as u32 * c) as i32,
            y as i32,
            scale,
            &get_font(),
            unsafe { transmute(text) },
        );
    }
}

/**
 * Draw interference lines
 * params image  - Background picture
 */
pub fn draw_interference_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, dark_mode: bool) {
    let width = image.width();
    let height = image.height();
    let x1: f32 = 5.0;
    let y1 = get_next(x1, height / 2);

    let x2 = (width - 5) as f32;
    let y2 = get_next((height / 2) as f32, height - 5);

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

/**
 * Draw a distraction circle
 * params num    - Number of circles drawn
 *        image  - Background picture
 */
pub fn draw_interference_ellipse(
    num: usize,
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    dark_mode: bool,
) {
    for _ in 0..num {
        let w = (10 + get_rnd(5)) as i32;
        let x = get_rnd((image.width() - 25) as usize) as i32;
        let y = get_rnd((image.height() - 15) as usize) as i32;
        draw_hollow_ellipse_mut(image, (x, y), w, w, get_color(dark_mode));
    }
}

/**
 * Convert image to JPEG bytes
 */
#[inline]
pub fn to_bytes(image: &DynamicImage, compression: u8) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    _ = JpegEncoder::new_with_quality(&mut buf, compression).encode_image(image);
    buf.into_inner()
}

/**
 * Convert image to JPEG base64 string
 */
pub fn to_base64_str(image: &DynamicImage, compression: u8) -> String {
    let res_base64 = general_purpose::STANDARD.encode(to_bytes(image, compression));
    format!("data:image/jpeg;base64,{}", res_base64)
}
