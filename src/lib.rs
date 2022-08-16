mod utils;

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
pub fn get_ascii_by_image(raw: Vec<u8>, scale: u32, revert: bool) -> String {
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

    if revert {
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

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
