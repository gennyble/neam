mod cli;

use std::{fs::File, path::PathBuf};

use cli::Scale;
use gifed::{
	block::Block,
	reader::{Decoder, ReadBlock},
	writer::{ImageBuilder, Writer},
};

fn main() {
	let cli = match cli::CliArgs::parse() {
		Some(c) => c,
		None => return,
	};

	let ext = match cli.input_file.extension() {
		None => {
			eprintln!("No extension! Not yet smart enough to discerne file type like this, sorry!");
			return;
		}
		Some(ext) => ext.to_str().unwrap(),
	};

	match ext.to_lowercase().as_str() {
		"png" => scale_png(cli.input_file, cli.output_file, cli.scale),
		"gif" => scale_gif(cli.input_file, cli.output_file, cli.scale),
		_ => {
			eprintln!("cannot yet scale {ext} type files");
			return;
		}
	}
}

fn scale_png(input_file: PathBuf, output_file: PathBuf, scale: Scale) {
	let file = match File::open(&input_file) {
		Ok(file) => file,
		Err(e) => {
			eprintln!("Failed to open '{}': {}", input_file.to_string_lossy(), e);
			return;
		}
	};

	let decoder = png::Decoder::new(file);
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

	let (new_width, new_height) = scale.get(info.width, info.height);

	let new = neam::nearest(
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

	let mut encoder = png::Encoder::new(ofile, new_width, new_height);
	encoder.set_color(info.color_type);
	encoder.set_depth(info.bit_depth);

	let png_info = reader.info();
	if let Some(palette) = png_info.palette.as_deref() {
		encoder.set_palette(palette.clone());
	}

	if let Some(trns) = png_info.trns.as_deref() {
		encoder.set_trns(trns.clone())
	}

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

fn scale_gif(input_file: PathBuf, output_file: PathBuf, scale: Scale) {
	let mut gif = Decoder::file(input_file).unwrap().read().unwrap();

	let (new_gif_width, new_gif_height) = scale.get(
		gif.screen_descriptor.width as u32,
		gif.screen_descriptor.height as u32,
	);

	if new_gif_width.max(new_gif_height) > u16::MAX as u32 {
		eprintln!("New dimensions are too large! Resulting size is {new_gif_width}x{new_gif_height}, but both dimensions must be below {}", u16::MAX);
		return;
	}

	let file = File::create(output_file).unwrap();
	let mut out = Writer::new(
		file,
		new_gif_width as u16,
		new_gif_height as u16,
		gif.palette.clone(),
	)
	.unwrap();

	loop {
		match gif.block().unwrap() {
			None => break,
			Some(ReadBlock {
				block: Block::CompressedImage(img),
				..
			}) => {
				let indexed = img.decompress().unwrap();
				let (new_width, new_height) =
					scale.get(indexed.width() as u32, indexed.height() as u32);

				let new_indicies = neam::nearest(
					&indexed.indicies,
					1,
					indexed.width() as u32,
					indexed.height() as u32,
					new_width,
					new_height,
				);

				let mut builder = ImageBuilder::new(new_width as u16, new_height as u16);

				if indexed.left() != 0 || indexed.top() != 0 {
					let (new_left, new_top) =
						scale.get(indexed.left() as u32, indexed.top() as u32);
					builder = builder.offset(new_left as u16, new_top as u16);
				}

				if let Some(palette) = indexed.local_color_table {
					builder = builder.palette(palette);
				}

				let scaled = builder.build(new_indicies).unwrap();
				out.image(scaled).unwrap();
			}
			Some(ReadBlock { block, .. }) => {
				out.block(block).unwrap();
			}
		}
	}

	out.done().unwrap();
}
