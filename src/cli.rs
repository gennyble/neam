use getopts::Options;

pub struct CliArgs {
    pub input_file: String,
    pub size: (u32, u32),
}

impl CliArgs {
    fn usage<S: AsRef<str>>(program_name: S, opts: &Options) {
        let brief = format!("usage: {} FILE [options]\n\nYour output file will be the name of your input, the new size, then .png.\nSo example.png scaled to 512x512 will be example_512x512.png", program_name.as_ref());
        eprint!("{}", opts.usage(&brief));
    }

    pub fn parse() -> Option<Self> {
        let prgm = std::env::args().next().unwrap();
        let args: Vec<String> = std::env::args().skip(1).collect();

        let mut opts = Options::new();
        opts.optopt("s", "size", "The new size of the image\nAccepted values are 0 to 4294967295 for either dimension. You can seperate with/height with an x or a coma. Ex: 512x512 or 512,512", "SIZE");
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
            matches.free[0].clone()
        } else {
            Self::usage(prgm, &opts);
            return None;
        };

        Some(Self { input_file, size })
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
