use image::DynamicImage;
use image::imageops;

pub fn process_img(path: &str, scale_mod: u32, contrast: f32) -> DynamicImage {
    let img = image::open(&path).expect("File not found.");
    let img_width = img.width() / scale_mod;
    let img_height = img.height() / scale_mod;

    img.adjust_contrast(contrast).resize(img_width, img_height, imageops::FilterType::Gaussian)
}
