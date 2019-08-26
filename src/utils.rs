use image::{DynamicImage, FilterType, GenericImageView, Rgba};

pub struct Rgb(pub u8, pub u8, pub u8);
pub struct AnsiValue(pub u8);

pub struct Block {
    pub ch: char,
    pub fg: Rgb,
    pub bg: Rgb,
}

/// Convert full color rgb to 256 color
pub fn rgb_to_ansi(color: Rgb) -> AnsiValue {
    let r = (u16::from(color.0) * 5 / 255) as u8;
    let g = (u16::from(color.1) * 5 / 255) as u8;
    let b = (u16::from(color.2) * 5 / 255) as u8;
    AnsiValue(16 + 36 * r + 6 * g + b)
}

/// Perform alpha premuliplication on a Rgba pixel to remove the alpha
pub fn premultiply(p: Rgba<u8>) -> Rgba<u8> {
    if p[3] == 255 {
        // No transparency
        return p;
    }

    let mut p = p;
    let alpha = f32::from(p[3]) / 255.;
    let bg = 0.;

    for pixel in p.0.iter_mut() {
        *pixel = (((1. - alpha) * bg) + (alpha * f32::from(*pixel))) as u8
    }

    p
}

/// Resizes an image to fit within a max size, then scales an image to fit within a block size
pub fn resize_image(
    img: &DynamicImage,
    cell_size: (u32, u32),
    max_size: (u16, u16),
) -> DynamicImage {
    let img = img.resize(
        (u32::from(max_size.0)) * cell_size.0,
        (u32::from(max_size.1)) * cell_size.1,
        FilterType::Nearest,
    );

    img.resize_exact(
        closest_mult(img.width(), cell_size.0),
        closest_mult(img.height(), cell_size.1),
        FilterType::Nearest,
    )
}

/// Returns the closest multiple of a base
fn closest_mult(x: u32, base: u32) -> u32 {
    base * ((x as f32) / base as f32).round() as u32
}
