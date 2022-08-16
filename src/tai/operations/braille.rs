use crate::tai::arguments::config::Config;
use crate::tai::operations::dither::Dither;
use crate::tai::utils::*;
use image::{codecs::gif::GifDecoder, AnimationDecoder, DynamicImage, RgbaImage};
use std::{fs::File, thread::sleep, time::Duration};

use super::otsu_threshold::OtsuThreshold;

/* Image to braille:
   source: https://en.wikipedia.org/wiki/Braille_Patterns

- open the image
- loop on the image buffer
- collect a chunck of pixels (2*4)
- calculate the chunck above and return a binary
- parse the binary and turn it to a valid number
- calculate the number and print a char based on it
*/

pub fn img_to_braille(config: Config) -> String {
    // checking if its animated
    if config.image_file.ends_with(".gif") {
        print_animated_image(&config);
    } else {
        // checking the image file is valid,if so opening the image.
        let img = image::load_from_memory(&config.image_file_u8).unwrap();

        // resizing the image and converting it to "imagebuffer",
        let mut img = resize(img, &config);
        // checking if the user wants to dither the image.
        if config.dither {
            img.dither(config.dither_scale);
        };

        return print_static(&img, &config);
    }
    "".to_string()
}

// taking a threshold value, image buffer, and origin pixel coordinates(x,y);
// will calculate the pixels from the origin pixel(the x,y is the pixel coordinates) and
// return a block of signals for everypixel.
fn get_block_signals(threshold: u8, img: &RgbaImage, coord_x: u32, coord_y: u32) -> [[u8; 2]; 4] {
    let mut pixel_map = [[0u8; 2]; 4];
    for iy in 0..4 {
        for ix in 0..2 {
            let [red, green, blue, _] = img.get_pixel(coord_x + ix, coord_y + iy).0;
            pixel_map[(iy) as usize][(ix) as usize] =
                if get_luminance(red, green, blue) > threshold as f32 {
                    1
                } else {
                    continue;
                };
        }
    }
    pixel_map
}

// this is the core parser function it will take a blocks of pixels converted to signals
// (1 = raised pixel, 0 = unraised pixel), and then convert it to a binary and then to a valid char.
fn translate(map: &mut [[u8; 2]; 4]) -> char {
    /* our pixel block(map) look like this:
          ---------
          | 0 | 1 |
          | 2 | 3 |
          | 4 | 5 |
          | 6 | 7 |
          ---------
        we want to convert it to this:
          ---------
          | 0 | 3 |
          | 1 | 4 |
          | 2 | 5 |
          | 6 | 7 |
          ---------
    */
    // making a copy to to not mess up the indexes of the original pixel matrix.
    let cloned = *map;
    map[0][1] = cloned[1][1];
    map[1][0] = cloned[0][1];
    map[1][1] = cloned[2][0];
    map[2][0] = cloned[1][0];
    // converting to string
    let mut tmp = String::new();
    for i in map {
        for j in i {
            tmp.push_str(&j.to_string());
        }
    }
    // reverse the "raised dots" to get a valid binary. (read wikipedia link above)
    let tmp = tmp.chars().rev().collect::<String>();
    // converting from base2 to integer
    let c = (isize::from_str_radix(&tmp, 2).unwrap()) as u32;
    std::char::from_u32(c + 0x2800).unwrap()
}

// process a static image
fn print_static(img: &RgbaImage, config: &Config) -> String {
    let best_threshold = DynamicImage::ImageRgba8(img.clone())
        .into_luma8()
        .get_otsu_value();
    let mut result = String::new();

    for y in (0..img.height() - 4).step_by(4) {
        for x in (0..img.width() - 2).step_by(2) {
            let mut map = get_block_signals(best_threshold, img, x, y);
            let ch = translate(&mut map);
            if config.colored {
                let [r, g, b, _] = img.get_pixel(x, y).0;
                print!("{}", colorize(&[r, g, b], ch, config.background));
            } else {
                // print!("{}", ch);
                result.push(ch);
            }
        }
        result.push('\n');
    }

    result
}

fn loop_the_animation(config: &Config, frames: &[String]) {
    for frame in frames {
        print!("{}", frame);
        sleep(Duration::from_millis(config.sleep))
    }
}

// process animated image
fn print_animated_image(config: &Config) {
    let frames = get_animated_frames(config);
    if config.once {
        loop_the_animation(config, &frames);
    } else {
        loop {
            loop_the_animation(config, &frames);
        }
    }
}

fn get_animated_frames(config: &Config) -> Vec<String> {
    let mut out_frames = Vec::new();
    let file_in = match File::open(&config.image_file) {
        Ok(file) => file,
        Err(_) => return out_frames,
    };
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .expect("error decoding gif");
    // pushing this ansi code to clear the screen in the start of the frames
    out_frames.push("\x1B[1J".to_string());

    for frame in frames {
        // prolly this is not efficient, need to read image crate docs more!
        let img = DynamicImage::ImageRgba8(frame.buffer().clone());
        let mut img = resize(img, config);
        if config.dither {
            img.dither(config.dither_scale);
        }
        let translated_frame = translate_frame(&img, config);
        // this ansi code will seek/save the cursor position to the start of the art
        // so for each frame will override the old one in stdout
        out_frames.push(format!("\x1B[r{}", translated_frame));
    }
    out_frames
}

fn translate_frame(img: &RgbaImage, config: &Config) -> String {
    let mut out = String::new();
    let best_threshold = DynamicImage::ImageRgba8(img.clone())
        .into_luma8()
        .get_otsu_value();

    for y in (0..img.height() - 4).step_by(4) {
        for x in (0..img.width() - 2).step_by(2) {
            let mut map = get_block_signals(best_threshold, img, x, y);
            let ch = translate(&mut map);

            if config.colored {
                let [r, g, b, _] = img.get_pixel(x, y).0;
                out.push_str(&colorize(&[r, g, b], ch, config.background));
            } else {
                out.push(ch);
            }
        }
        out.push('\n');
    }
    out
}
