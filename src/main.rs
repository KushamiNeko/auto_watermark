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

	

	// for i in paths {
	// 	println!("{}", i.unwrap().path().display());
	// }


    // let base = match image::open(&Path::new(TEST_BASE)) {
    // 	Ok(img) => img,
    // 	Err(err) => {panic!("{:?}", err.to_string())},
    // };

    // let mark: image::DynamicImage = match image::open(&Path::new(WATER_MARK)) {
    // 	Ok(img) => img,
    // 	Err(err) => {panic!("{:?}", err.to_string())},
    // };

    // let width = base.dimensions().0;
    // let height = base.dimensions().1;

    // let mut new = image::DynamicImage::new_rgb8(width, height);
    // // let mut new = Box::new(image::ImageBuffer::new(width, height));

    // let mut image_comp = ImageWork::new(Box::new(base), Box::new(mark), Box::new(new), width, height, TEST_OUTPUT, MARK_OPACITY);
    // image_comp.pixel_operation();
    // image_comp.image_save();

    // for x in 0..width {
    // 	for y in 0..height {
    // 		let r = mark.get_pixel(x, y).channels4().0;
    // 		let g = mark.get_pixel(x, y).channels4().1;
    // 		let b = mark.get_pixel(x, y).channels4().2;

    // 		if r != 0 && g != 0 && b != 0 {
    // 			// println!("{:?}", mark.get_pixel(x, y).channels4());
    // 			let (nr, ng, nb, na) = pixel_blend(&base, &mark, MARK_OPACITY, x, y);
    // 			new.put_pixel(x, y, image::Rgba::<u8>{data: [nr, ng, nb, na]});
    // 		} else {
    // 			new.put_pixel(x, y, base.get_pixel(x, y));
    // 		}
    // 	}
    // }

    // for x in 0..width {
    // 	for y in 0..height {
    // 		let r = new.get_pixel(x, y).channels4().0;
    // 		let g = new.get_pixel(x, y).channels4().1;
    // 		let b = new.get_pixel(x, y).channels4().2;

    // 		if r >= 255 || g >= 255 || b >= 255 {
    // 			println!("{:?}", new.get_pixel(x, y).channels4());
    // 		} else if r <= 0 || g <= 0 || b <= 0 {
    // 			println!("{:?}", new.get_pixel(x, y).channels4());
    // 		}
    // 	}
    // }

    // let ref mut out_put = File::create(&Path::new(TEST_OUTPUT)).unwrap();
    // let _ = image::ImageRgba8(new).save(out_put, image::PNG);
    // let encoder = image::png::PNGEncoder::new(out_put);
    // let encoder = image::jpeg::JPEGEncoder::new(out_put);
    // let _ = encoder.encode(&new.raw_pixels()[..], width, height, ColorType::RGB(8));
    // let _ = mark.save(out_put, image::ImageFormat::JPEG).unwrap();

}

