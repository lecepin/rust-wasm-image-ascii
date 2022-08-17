mod tai;
mod utils;

use crate::tai::{
    arguments::config::Config,
    operations::{ascii::img_to_ascii, braille::img_to_braille, onechar::img_to_onechar},
};
use crate::utils::log;
use image::{
    codecs::jpeg::JpegEncoder, imageops::FilterType, load_from_memory, ColorType, GenericImageView,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_gray_image(raw: Vec<u8>, scale: u32) -> Vec<u8> {
    let img = load_from_memory(&raw).unwrap();
    let img = img
        .resize(
            (img.width() * scale / 100) as u32,
            (img.height() * scale / 100) as u32,
            FilterType::Nearest,
        )
        .grayscale();
    let (width, height) = (img.width(), img.height());
    let img_raw = img.into_bytes();

    // 给编码器一块内存空间，用来写入数据
    let mut output_buffer = vec![];
    // 创建一个编码器，jpg 的可以指定质量
    let mut encoder = JpegEncoder::new_with_quality(&mut output_buffer, 100);

    // 编码输出
    encoder
        .encode(&img_raw, width, height, ColorType::L8)
        .unwrap();
    // 直接把内存输出就行
    output_buffer
}

#[wasm_bindgen]
pub fn get_ascii_by_image(raw: Vec<u8>, scale: u32, reverse: bool) -> String {
    let img = load_from_memory(&raw).unwrap();
    let img = img
        .resize(
            (img.width() * scale / 100) as u32,
            (img.height() * scale / 100) as u32,
            FilterType::Nearest,
        )
        .grayscale();
    let mut pallete = [' ', '.', '\\', '*', '#', '$', '@'];
    let mut current_line = 0;
    let mut result = "".to_string();

    if reverse {
        pallete.reverse();
    }

    for (_, line, rgba) in img.pixels() {
        if current_line != line {
            result.push('\n');
            current_line = line;
        }

        let r = 0.2126 * (rgba.0[0] as f32);
        let g = 0.7152 * (rgba.0[0] as f32);
        let b = 0.0722 * (rgba.0[0] as f32);
        let gray = r + g + b;
        let caracter = ((gray / 255.0) * (pallete.len() - 1) as f32).round() as usize;

        result.push(pallete[caracter]);

        // 填充一下，有些扁
        if caracter < (pallete.len() - 2) {
            result.push('.');
        } else {
            result.push(' ');
        }
    }

    result
}

#[wasm_bindgen]
pub fn get_ascii_by_image_tai(raw: Vec<u8>, scale: u32, reverse: bool, style: &str) -> String {
    let mut config = Config::default();

    config.image_file_u8 = raw;
    config.scale = scale;
    config.reverse = reverse;

    match style {
        "OneChar" => img_to_onechar(config, reverse),
        "Braille" => img_to_braille(config),
        "Ascii" => {
            let mut table = vec![
                ' ', ' ', ' ', ' ', '.', '.', '.', ',', ',', ',', '\'', ';', ':', '7', '3', 'l',
                'o', 'b', 'd', 'x', 'k', 'O', '0', 'K', 'X', 'N', 'W', 'M',
            ];

            if config.reverse {
                table.reverse();
            }

            img_to_ascii(config, &table)
        }
        "Numbers" => {
            let mut table = vec![
                ' ', ' ', ' ', ' ', '0', '1', '7', '6', '9', '4', '2', '3', '8',
            ];

            if config.reverse {
                table.reverse();
            }

            img_to_ascii(config, &table)
        }
        "Blocks" => {
            let mut table = vec![' ', ' ', ' ', ' ', '░', '▒', '▓', '█'];

            if config.reverse {
                table.reverse();
            }

            img_to_ascii(config, &table)
        }
        _ => "".to_string(),
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
