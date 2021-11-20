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

fn scale<P: Into<PathBuf>>(infile: P, size: (u32, u32)) {
    let infile: PathBuf = infile.into();

    let file = match File::open(&infile) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open '{}': {}", infile.to_string_lossy(), e);
            return;
        }
    };

    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let original_width = info.width;
    let original_height = info.height;

    let new = nearest::nearest(&buf, original_width, original_height, size.0, size.1);

    let newstem = format!(
        "{}_{}x{}.png",
        infile.file_stem().unwrap().to_string_lossy(),
        size.0,
        size.1
    );
    let mut newname = infile.clone();
    newname.set_file_name(newstem);

    let ofile = File::create(newname).unwrap();

    let mut encoder = Encoder::new(ofile, size.0, size.1);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&new).unwrap();
}
