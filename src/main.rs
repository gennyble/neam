mod cli;
mod nearest;

use std::{fs::File, path::PathBuf};

use png::{BitDepth, ColorType, Decoder, Encoder};

fn main() {
    let cli = match cli::CliArgs::parse() {
        Some(c) => c,
        None => return,
    };

    scale(cli.input_file, cli.size);
}

fn scale<P: Into<PathBuf>>(input_file: P, size: (u32, u32)) {
    let mut filepath: PathBuf = input_file.into();

    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open '{}': {}", filepath.to_string_lossy(), e);
            return;
        }
    };

    let decoder = Decoder::new(file);
    let mut reader = match decoder.read_info() {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!(
                "Failed to read PNG header from '{}': {}",
                filepath.to_string_lossy(),
                e
            );
            return;
        }
    };

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = match reader.next_frame(&mut buf) {
        Ok(info) => info,
        Err(e) => {
            eprintln!(
                "Failed to read an image from '{}': {}",
                filepath.to_string_lossy(),
                e
            );
            return;
        }
    };

    if info.color_type != ColorType::Rgba {
        panic!("FIXME: Cannot handle color types other than Rgba!");
    }

    let new = nearest::nearest(&buf, info.width, info.height, size.0, size.1);

    // We're reusing the input files PathBuf here
    let input_stem = filepath.file_stem().unwrap().to_string_lossy().to_string();
    filepath.set_file_name(format!("{}_{}x{}.png", input_stem, size.0, size.1));

    let ofile = match File::create(&filepath) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Could not create the output file '{}': {}",
                filepath.to_string_lossy(),
                e
            );
            return;
        }
    };

    let mut encoder = Encoder::new(ofile, size.0, size.1);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    match encoder.write_header() {
        Err(e) => {
            eprintln!(
                "Failed to write PNG header to '{}': {}",
                filepath.to_string_lossy(),
                e
            );
            return;
        }
        Ok(mut w) => match w.write_image_data(&new) {
            Err(e) => {
                eprintln!(
                    "Failed to write image data to '{}': {}",
                    filepath.to_string_lossy(),
                    e
                );
                return;
            }
            Ok(()) => (),
        },
    }
}
