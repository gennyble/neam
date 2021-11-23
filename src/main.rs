mod cli;
mod nearest;

use std::{fs::File, path::PathBuf};

use png::{BitDepth, ColorType, Decoder, Encoder};

fn main() {
    let cli = match cli::CliArgs::parse() {
        Some(c) => c,
        None => return,
    };

    scale(cli.input_file, cli.output_file, cli.size);
}

fn scale(input_file: PathBuf, output_file: PathBuf, size: (u32, u32)) {
    let file = match File::open(&input_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open '{}': {}", input_file.to_string_lossy(), e);
            return;
        }
    };

    let decoder = Decoder::new(file);
    let mut reader = match decoder.read_info() {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!(
                "Failed to read PNG header from '{}': {}",
                input_file.to_string_lossy(),
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
                input_file.to_string_lossy(),
                e
            );
            return;
        }
    };

    let new = nearest::nearest(
        &buf,
        info.color_type.samples(),
        info.width,
        info.height,
        size.0,
        size.1,
    );

    let ofile = match File::create(&output_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Could not create the output file '{}': {}",
                output_file.to_string_lossy(),
                e
            );
            return;
        }
    };

    let mut encoder = Encoder::new(ofile, size.0, size.1);
    encoder.set_color(info.color_type);
    encoder.set_depth(BitDepth::Eight);

    match encoder.write_header() {
        Err(e) => {
            eprintln!(
                "Failed to write PNG header to '{}': {}",
                output_file.to_string_lossy(),
                e
            );
            return;
        }
        Ok(mut w) => match w.write_image_data(&new) {
            Err(e) => {
                eprintln!(
                    "Failed to write image data to '{}': {}",
                    output_file.to_string_lossy(),
                    e
                );
                return;
            }
            Ok(()) => (),
        },
    }
}
