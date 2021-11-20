mod nearest;

use std::{fs::File, path::PathBuf};

use getopts::Options;
use png::{BitDepth, ColorType, Decoder, Encoder};

fn main() {
    let prgm = std::env::args().next().unwrap();
    let args: Vec<String> = std::env::args().skip(1).collect();

    let print_help = |opt: Options| {
        let brief = format!("usage: {} FILE [options]\n\nYour output file will be the name of your input, the new size, then .png.\nSo example.png scaled to 512x512 will be example_512x512.png", prgm);
        print!("{}", opt.usage(&brief));
    };

    let mut opts = Options::new();
    opts.optopt("s", "size", "The new size of the image\nAccepted values are 0 to 4294967295 for either dimension. You can seperate with/height with an x or a coma. Ex: 512x512 or 512,512", "SIZE");
    opts.optflag("h", "help", "Print this help message");
    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(e) => panic!("{}", e),
    };

    if matches.opt_present("help") {
        print_help(opts);
        return;
    }

    let size = if let Some(scale) = matches.opt_str("size") {
        match parse_scale_string(scale) {
            Ok(width_height) => width_height,
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    } else {
        print_help(opts);
        return;
    };

    let infile = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_help(opts);
        return;
    };

    scale(infile, size);
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

    println!("{:?}", info.color_type);

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

fn parse_scale_string<S: AsRef<str>>(raw: S) -> Result<(u32, u32), &'static str> {
    let raw = raw.as_ref();

    let format_err = Err("Scale is not formatted as widthxheight or width,height! Please format your size as one of these.");
    let splitchar = if raw.contains(',') {
        ','
    } else if raw.contains('x') {
        'x'
    } else {
        return format_err;
    };

    match raw.split_once(splitchar) {
        Some((width_s, height_s)) => {
            let width: u32 = width_s
                .parse()
                .map_err(|_| "Failed to parse width as a number!")?;
            let height: u32 = height_s
                .parse()
                .map_err(|_| "Failed to parse height as a number!")?;

            Ok((width, height))
        }
        None => format_err,
    }
}
