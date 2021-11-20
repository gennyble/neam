use std::path::PathBuf;

use getopts::Options;

pub struct CliArgs {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub size: (u32, u32),
}

impl CliArgs {
    fn usage<S: AsRef<str>>(program_name: S, opts: &Options) {
        let brief = format!("usage: {} FILE [options]", program_name.as_ref());
        eprint!("{}", opts.usage(&brief));
    }

    pub fn parse() -> Option<Self> {
        let prgm = std::env::args().next().unwrap();
        let args: Vec<String> = std::env::args().skip(1).collect();

        let mut opts = Options::new();
        opts.optopt(
			"s",
			"size",
			"The new size of the image.\nYou can separate width/height with an x or a comma.\nEx: 512x512 or 512,512",
			"SIZE"
		);
        opts.optopt(
            "o",
            "output",
            "The name of the output file.\nDefaults to the input name with _widthxheight appended",
            "PATH",
        );
        opts.optflag("h", "help", "Print this help message");
        let matches = match opts.parse(&args) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("error: {}", e);
                return None;
            }
        };

        if matches.opt_present("help") {
            Self::usage(prgm, &opts);
            return None;
        }

        let size = if let Some(scale) = matches.opt_str("size") {
            match Self::parse_scale_string(scale) {
                Ok(width_height) => width_height,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        } else {
            Self::usage(prgm, &opts);
            return None;
        };

        let input_file = if !matches.free.is_empty() {
            PathBuf::from(&matches.free[0])
        } else {
            Self::usage(prgm, &opts);
            return None;
        };

        let output_file = if let Some(out) = matches.opt_str("output") {
            PathBuf::from(out)
        } else {
            let mut out = input_file.clone();
            let input_stem = out.file_stem().unwrap().to_string_lossy().to_string();
            out.set_file_name(format!("{}_{}x{}.png", input_stem, size.0, size.1));

            out
        };

        Some(Self {
            input_file,
            output_file,
            size,
        })
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
}
