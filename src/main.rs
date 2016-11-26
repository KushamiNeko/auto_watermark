extern crate image;
extern crate num;
// extern crate num_cpus;

use std::path::Path;

// use image::GenericImage;
// use image::Pixel;
// use image::ColorType;

mod imagework;
mod data_analysis;

const WATER_MARK_ROOT: &'static str = r#"water_mark_source_image"#;
const PROGRAM_DATA: &'static str = r#"program_data"#;
const TOLERANCE: f64 = 5.0;

fn main() {
	data_analysis::tolerance_generation(PROGRAM_DATA, TOLERANCE);
	data_analysis::data_generation(PROGRAM_DATA, WATER_MARK_ROOT, TOLERANCE);
	return;

	println!("{}", "Please enter a directories!");
	
	let mut user_input = std::string::String::new();
	match std::io::stdin().read_line(&mut user_input) {
		Ok(_) => {},
		Err(e) => {
			println!("{}", e);
		},
	}
	
	let input_path = user_input.trim();
	let mut directoy = true;
	match input_path {
		"" => {
			println!("{:?}", "Please enter a valid path!");
			return;
		},
		_ => {
			let metadata = std::fs::metadata(Path::new(&input_path));
			match metadata {
				Ok(m) => {
					if m.is_dir() {
						println!("{:?}", "a directories");
					} else if m.is_file() {
						println!("{:?}", "a file");
						directoy = false;
					};
				},

				Err(_) => {
					println!("{:?}", "The path does not exist!");
					return;
				},
			};
		},
	}

	if directoy {
		let paths = std::fs::read_dir(Path::new(&input_path)).unwrap();
		for i in paths {
			println!("{}", i.unwrap().path().display());
		}
	} else {
		let mut working_task = match imagework::ImageWork::new(&input_path, &input_path) {
			Some(image) => image,
			None => {
				println!("{:?}", "invalid input!");
				return;
			},
		};

		working_task.pixel_operation();
		working_task.image_save();
	}
}

