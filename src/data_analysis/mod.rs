extern crate image;
// extern crate num_cpus;

use std;
use std::path::Path;
use std::collections::LinkedList;
use std::io::Write;

use image::GenericImage;
use image::Pixel;

const TOLERANCE_DATA: &'static str = r#"tolerance.txt"#;
const NUM_THREADS: u8 = 8;

pub fn data_generation(program_data_root: &str, template_root: &str, tolerance: f64) {
	let mut num_file = 0;

	let data_files = match std::fs::read_dir(Path::new(program_data_root)) {
	    Ok(data) => data,
	    Err(_) => {
	    	println!("{:?}", "The data generation process has failed!");
			return;
	    },
	};

	for i in data_files {
		let file_path = i.unwrap().path();
		{
			let file_name = file_path.file_name().unwrap();
			let file_name_str = file_name.to_str().unwrap();
			if file_name_str == TOLERANCE_DATA {
				continue;
			}
		}
		let _ = std::fs::remove_file(file_path);
	}

	let files = match std::fs::read_dir(Path::new(template_root)) {
		Ok(path) => path,
		Err(_) => {
			println!("{:?}", "The data generation process has failed!");
			return;
		},
	};

	// let arc_tolerance = std::sync::Arc::new(tolerance);
	// let arc_output_data = std::sync::Arc::new(program_data_root);

	let ptr_tolerance: *const f64 = &tolerance;
	let ptr_output_data: *const str = &*program_data_root;

	let (t_done, s_done) = std::sync::mpsc::channel();

	for i in files {
		// let tolerance = arc_tolerance.clone();
		// let program_data_root = arc_output_data.clone();
		let done = t_done.clone();

		// let arc_path = std::sync::Arc::new(i.unwrap());
		// let arc_num_file = std::sync::Arc::new(num_file);

		let ptr_path: *const std::fs::DirEntry = &i.unwrap();
		let ptr_num_file: *const i64 = &num_file;

		// let arc_i = arc_path.clone();
		// let arc_num = arc_num_file.clone();

		let _ = std::thread::spawn(move || {
			unsafe {
				let mut influence_area : LinkedList<(u32, u32)> = LinkedList::new();
				let image = match image::open((*ptr_i).path()) {
			    	Ok(img) => img,
			    	Err(_) => {
			    		println!("{:?}", "The path is invalid!");
			    		return;
			    	},
				};

				let width = image.dimensions().0;
				let height = image.dimensions().1;

				let lower_bound;
				let upper_bound;

				if width > height {
					lower_bound = width as f64 / (height as f64 + *ptr_tolerance);
					upper_bound = width as f64 / (height as f64 - *ptr_tolerance);
				} else if width == height {
					lower_bound = (width as f64 - *ptr_tolerance) / height as f64;
					upper_bound = width as f64 / (height as f64 - *ptr_tolerance);
				} else {
					lower_bound = (width as f64 - *ptr_tolerance) / height as f64;
					upper_bound = width as f64 / (height as f64 + *ptr_tolerance);
				}

				for x in 0..width {
					for y in 0..height {
						let r = image.get_pixel(x, y).channels4().0;
			    		let g = image.get_pixel(x, y).channels4().1;
			    		let b = image.get_pixel(x, y).channels4().2;

			    		if r != 0 && g != 0 && b != 0 {
			    			influence_area.push_back((x, y));
			    		}
					}
				}

				let output_data = format!("{}.txt", *ptr_num_file);
				let data_root_path = Path::new(*ptr_program_data_root);
				let output_data_path = data_root_path.join(Path::new(&(*output_data)));

				let mut output_file = std::fs::File::create(output_data_path).unwrap();

				let dimensions_data = format!("{},{} \r\n", width, height);
				let _ = output_file.write(&dimensions_data.into_bytes());

				let bound_data = format!("{},{} \r\n", lower_bound, upper_bound);
				let _ = output_file.write(&bound_data.into_bytes());

				for i in influence_area.iter() {
					let area_data = format!("{},{} \r\n", i.0, i.1);
					let _ = output_file.write(&area_data.into_bytes());
				}

				let _ = output_file.sync_all();
				let _ = done.send(());
			}
		});

		num_file += 1;

		if num_file >= NUM_THREADS {
			let _ = s_done.recv();
		}
	}

	for _ in 0..NUM_THREADS {
		let _ = s_done.recv();
		num_file -= 1;
		if num_file <= 0 {
		    return;
		}
	}
}

pub fn tolerance_generation(program_data_root: &str, tolerance: f64) {
	let program_data_path = Path::new(program_data_root);
	let tolerance_data_path = program_data_path.join(&Path::new(TOLERANCE_DATA));
	let mut output_file = std::fs::File::create(tolerance_data_path).unwrap();

	let data = format!("{} \r\n", tolerance);
	let _ = output_file.write(&data.into_bytes());
}