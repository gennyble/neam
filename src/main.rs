mod cli;
mod nearest;

use std::{fs::File, path::PathBuf};

use cli::Scale;
use png::{BitDepth, Decoder, Encoder};

fn main() {
    let cli = match cli::CliArgs::parse() {
        Some(c) => c,
        None => return,
    };

    scale(cli.input_file, cli.output_file, cli.scale);
}

fn scale(input_file: PathBuf, output_file: PathBuf, scale: Scale) {
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

    let (new_width, new_height) = match scale {
        Scale::Absolute(width, height) => (width, height),
        Scale::Percent(mut perc) => {
            perc /= 100.0;

            (
                (info.width as f32 * perc) as u32,
                (info.height as f32 * perc) as u32,
            )
        }
    };

    let new = nearest::nearest(
        &buf,
        info.color_type.samples(),
        info.width,
        info.height,
        new_width,
        new_height,
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

    let mut encoder = Encoder::new(ofile, new_width, new_height);
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
