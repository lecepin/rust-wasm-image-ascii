use crate::tai::arguments::config::Config;
use crate::tai::operations::otsu_threshold::OtsuThreshold;
use crate::tai::utils::get_luma_buffer;
use image::Luma;

//  will make the image to ONLY black and white
//  by converting the the "grays" to black or white based on the scale.
// source: https://en.wikipedia.org/wiki/Thresholding_(image_processing)
// below we are using Otsu's thresholding which is automatically finds
// the best threshold value
// https://en.wikipedia.org/wiki/Otsu%27s_method
pub fn img_to_onechar(config: Config) -> String {
    let mut img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = match get_luma_buffer(&config) {
        Some(img) => img,
        None => return "".to_string(),
    };
    let mut result = "".to_string();

    img.threshold();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            if *pixel == Luma([255]) {
                result.push(config.onechar);
            } else {
                result.push('\n');
            }
        }
        result.push('\n');
    }
    result.push('\n');

    result
}
