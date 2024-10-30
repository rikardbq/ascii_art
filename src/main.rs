use std::io::Write;
use std::env;

mod helpers;
use helpers::{
    misc_helper::{ normalize_ascii_table, build_output_str, create_output_file },
    img_helper::process_img,
};

fn main() -> std::io::Result<()> {
    let mut scale_mod: u32 = 10;
    let mut contrast: f32 = 0.0;

    let args: Vec<String> = env::args().collect();
    let file_path_arg = &args[1];

    if args.len() > 3 {
        for i in 0..args.capacity() {
            let _ = match &args[i] as &str {
                "--scale" => {
                    scale_mod = args[i + 1].parse::<u32>().unwrap();
                }
                "--contrast" => {
                    contrast = args[i + 1].parse::<f32>().unwrap();
                }
                _ => (),
            };
        }

        // makeshift solution because i just wanted to use split
        // let mut scale_arg_iter = args.split(|a| -> bool { a == "--scale" });
        // let mut contrast_arg_iter = args.split(|a| -> bool { a == "--contrast" });
        // scale_arg_iter.next();
        // contrast_arg_iter.next();
        // let scale_mod_arg_val = scale_arg_iter.next().unwrap();
        // let contrast_arg_val = contrast_arg_iter.next().unwrap();
        // scale_mod = scale_mod_arg_val[0].parse::<u32>().unwrap();
        // contrast = contrast_arg_val[0].parse::<u32>().unwrap();
    }

    let mut char_table = String::with_capacity(256);
    char_table.push_str(normalize_ascii_table().as_str());
    let img = process_img(file_path_arg, scale_mod, contrast);
    let output_str = build_output_str(img, char_table);
    let mut output_file = create_output_file(file_path_arg)?;

    write!(output_file, "{}", output_str)?;

    Ok(())
}
