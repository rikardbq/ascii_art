use std::iter;
use std::fs::File;
use std::io::Result;

use image::DynamicImage;

const ASCII_TABLE: &str = "          ¨'^_,.-;:/!iIlLuUoOöÖ?¤½%&£tTpPzZnNsSxXrRbBwWmMåÅäÄ$€@";
const PATH_SEPARATOR: (char, char) = ('/', '\\');

pub fn normalize_ascii_table() -> String {
    let mut padded_str = String::with_capacity(256);
    let _ = &ASCII_TABLE.chars().for_each(
        |c| -> () {
            padded_str.push_str(iter::repeat(c).take(4).collect::<String>().as_str());
        }
    );

    padded_str
}

pub fn build_output_str(img: DynamicImage, char_table: String) -> String {
    let mut output = String::new();
    img.to_luma8()
        .pixels()
        .enumerate()
        .for_each(
            |(idx, val)| -> () {
                let ch_idx = val.0[0] as usize;
                let _ = output.push(
                    if (idx + 1) % (img.width() as usize) == 0 {
                        '\n'
                    } else {
                        char_table.chars().nth(ch_idx).unwrap()
                    }
                );
            }
        );

    output
}

pub fn create_output_file(path: &str) -> Result<File> {
    let file_name = path
        .split(|c| (c == PATH_SEPARATOR.0 || c == PATH_SEPARATOR.1))
        .last()
        .unwrap()
        .replace(".", "_");

    File::create(format!("{}_ascii.txt", file_name))
}
