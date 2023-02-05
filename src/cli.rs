use core::fmt;
use std::{path::PathBuf, str::FromStr};

pub struct CliArgs {
	pub input_file: PathBuf,
	pub output_file: PathBuf,
	pub scale: Scale,
}

impl CliArgs {
	fn usage() {
		eprintln!(include_str!("usage.txt"));
	}

	pub fn parse() -> Option<Self> {
		let prgm = std::env::args().next().unwrap();
		let args: Vec<String> = std::env::args().skip(1).collect();

		match args.len() {
			0 | 1 => {
				Self::usage();

				None
			}
			2 | 3 => {
				let scale = match args[1].parse() {
					Ok(width_height) => width_height,
					Err(e) => {
						eprintln!("{}", e);
						return None;
					}
				};

				let input_file = PathBuf::from(&args[0]);

				let output_file = if let Some(out) = args.get(2) {
					PathBuf::from(out)
				} else {
					let mut out = input_file.clone();
					let input_stem = out.file_stem().unwrap().to_string_lossy().to_string();
					out.set_file_name(format!("{}_{}.png", input_stem, scale));

					out
				};

				Some(Self {
					input_file,
					output_file,
					scale,
				})
			}
			_ => {
				eprintln!("Too many arguments!");
				Self::usage();

				None
			}
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Scale {
	Absolute(u32, u32),
	Percent(f32),
}

impl Scale {
	pub fn get(&self, current_width: u32, current_height: u32) -> (u32, u32) {
		match self {
			Scale::Absolute(width, height) => (*width, *height),
			Scale::Percent(mut perc) => {
				perc /= 100.0;

				(
					(current_width as f32 * perc) as u32,
					(current_height as f32 * perc) as u32,
				)
			}
		}
	}
}

impl FromStr for Scale {
	type Err = &'static str;

	fn from_str(raw: &str) -> Result<Scale, &'static str> {
		let format_err = Err("Scale cannot be parsed from string. Please format as WidthxHeight; Width,Height; or scale%");
		let splitchar = if raw.contains(',') { ',' } else { 'x' };

		match raw.split_once(splitchar) {
			Some((width_s, height_s)) => {
				let width: u32 = width_s
					.parse()
					.map_err(|_| "Failed to parse width as a number!")?;
				let height: u32 = height_s
					.parse()
					.map_err(|_| "Failed to parse height as a number!")?;

				Ok(Self::Absolute(width, height))
			}
			None if raw.ends_with('%') => {
				let percent: f32 = raw
					.strip_suffix('%')
					.unwrap()
					.parse()
					.map_err(|_| "Failed to parse scale percent!")?;

				Ok(Self::Percent(percent))
			}
			None => format_err,
		}
	}
}

impl fmt::Display for Scale {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Absolute(width, height) => write!(f, "{}x{}", width, height),
			Self::Percent(perc) => write!(f, "{:.1}x", perc / 100.0),
		}
	}
}
