mod blocks;
mod utils;

use image::{GenericImage, GenericImageView, Rgba};

pub use utils::{rgb_to_ansi, Block, Rgb};

fn process_block(
    sub_img: &impl GenericImage<Pixel = Rgba<u8>>,
    bitmaps: &[(u32, char)],
    blend: bool,
) -> Block {
    // Determine the best color
    // First, determine the best color range
    let mut max = [0u8; 3];
    let mut min = [255u8; 3];
    for (_, _, p) in sub_img.pixels() {
        let p = utils::premultiply(p);
        for i in 0..3 {
            max[i] = max[i].max(p[i]);
            min[i] = min[i].min(p[i]);
        }
    }

    let mut split_index = 0;
    let mut best_split = 0;
    for i in 0..3 {
        let diff = max[i] - min[i];
        if diff > best_split {
            best_split = diff;
            split_index = i
        }
    }
    let split_value = min[split_index] + best_split / 2;

    // Then use the median of the range to find the average of the forground and background
    // The median value is used to convert the 4x8 image to a bitmap
    let mut fg_count = 0;
    let mut bg_count = 0;
    let mut fg_color = [0u32; 3];
    let mut bg_color = [0u32; 3];
    let mut bits = 0u32;

    for y in 0..sub_img.height() {
        for x in 0..sub_img.width() {
            bits <<= 1;
            let pixel = sub_img.get_pixel(x, y);
            let pixel = utils::premultiply(pixel);
            if pixel[split_index] > split_value {
                bits |= 1;
                fg_count += 1;
                for i in 0..3 {
                    fg_color[i] += u32::from(pixel[i]);
                }
            } else {
                bg_count += 1;
                for i in 0..3 {
                    bg_color[i] += u32::from(pixel[i]);
                }
            }
        }
    }

    // Get the averages
    for i in 0..3 {
        if fg_count != 0 {
            fg_color[i] /= fg_count;
        }

        if bg_count != 0 {
            bg_color[i] /= bg_count;
        }
    }

    // A perfect match is 0x0 so start at max
    let mut best_diff = 0xffff_ffffu32;
    let mut best_char = ' ';
    // The best match may be inverted
    let mut invert = false;

    // Determine the difference between the calculated bitmap and the character map
    for (bitmap, ch) in bitmaps.iter() {
        let diff = (bitmap ^ bits).count_ones();
        if diff < best_diff {
            best_diff = diff;
            best_char = *ch;
            invert = false
        }
        // Check the inverted bitmap
        let inverted_diff = (!bitmap ^ bits).count_ones();
        if inverted_diff < best_diff {
            best_diff = inverted_diff;
            best_char = *ch;
            invert = true;
        }
    }

    if blend {
        // If the bitmap does not fit "well", use a gradient,w
        if best_diff > 10 {
            invert = false;
            best_char = [' ', '\u{2591}', '\u{2592}', '\u{2593}', '\u{2588}']
                [4.min(fg_count as usize * 5 / 32)];
        }
    }

    // If best map is inverted, swap the colors
    if invert {
        std::mem::swap(&mut fg_color, &mut bg_color);
    }

    Block {
        ch: best_char,
        fg: Rgb(fg_color[0] as u8, fg_color[1] as u8, fg_color[2] as u8),
        bg: Rgb(bg_color[0] as u8, bg_color[1] as u8, bg_color[2] as u8),
    }
}

pub fn render(mut img: image::DynamicImage, blend: bool, style: u32) -> Vec<Vec<Block>> {
    let bitmap = match style {
        1 => blocks::BITMAPS_NO_SLOPES,
        2 => blocks::BITMAPS_BLOCKS,
        3 => blocks::BITMAPS_HALFS,
        _ => blocks::BITMAPS_NO_SLOPES,
    };

    let mut out = Vec::with_capacity(img.height() as usize / 8);

    for y in (0..img.height()).step_by(8) {
        let mut row = Vec::with_capacity(img.width() as usize / 4);
        for x in (0..img.width()).step_by(4) {
            let sub_img = img.sub_image(x, y, 4, 8);
            let block = process_block(&sub_img, bitmap, blend);
            row.push(block);
        }
        out.push(row);
    }
    out
}
