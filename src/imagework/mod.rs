extern crate image;
extern crate num;

use std;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use num::complex::Complex;

use image::GenericImage;
use image::Pixel;

const WATER_MARK_ROOT: &'static str = r"water_mark_source_image";
const WATER_MARK_OPACITY: f64 = 0.3;
const PROGRAM_DATA: &'static str = r#"program_data"#;

struct WaterMarkType<'a> {
	first: &'a str, 
	second: &'a str, 
	third: &'a str, 
	forth: &'a str, 
	fifth: &'a str, 
}

struct Item<T>
where T: std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::ops::Rem {
    first: T,
    second: T,
}

impl <T> Item<T>
where T: std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::ops::Rem {
    fn new(first: T, second:T) -> Item<T> {
    	Item{first: first, second: second}
    }
}

struct WaterMarkSize<T>
where T: std::ops::Add + std::ops::Sub + std::ops::Mul + std::ops::Div + std::ops::Rem {
    first: Item<T>,
    second: Item<T>,
    third: Item<T>,
    forth: Item<T>,
    fifth: Item<T>,
}

pub struct ImageWork<'a> {
    source: Box<image::DynamicImage>,
    template: Box<image::DynamicImage>,
    output: Box<image::DynamicImage>,

    width: u32,
    height: u32,

    output_file: &'a str,
    opacity: f64,
}

impl <'a> ImageWork<'a> {

	pub fn new(source: &'a str, output_file: &'a str) -> std::option::Option<ImageWork<'a>> {
		let base = match image::open(&Path::new(source)) {
	    	Ok(img) => Box::new(img),
	    	Err(_) => {
	    		println!("{:?}", "The path is invalid!");
	    		return None;
	    	},
	    };

		let width = base.dimensions().0;
	    let height = base.dimensions().1;
	    let base_ratio = width as f64 / height as f64;

	    

	    let water_mark_size_ratio: WaterMarkSize<f64> = WaterMarkSize {
			first: Item::new(4.0, 3.0),
			second: Item::new(16.0, 9.0),
			third: Item::new(1.0, 1.0),
			forth: Item::new(3.0, 4.0),
			fifth: Item::new(9.0, 16.0),
		};

	    let new = Box::new(image::DynamicImage::new_rgb8(width, height));

		let water_mark_type: WaterMarkType = WaterMarkType {
			first: "Horzontal4_3.png", 
			second: "Horzontal16_9.png",
			third: "Square1_1.png",
			forth: "Vertical3_4.png",
			fifth: "Vertical9_16.png",
		};

		let water_mark_size_ratio: WaterMarkSize<f64> = WaterMarkSize {
			first: Item::new((water_mark_size.first.first / (water_mark_size.first.second + TOLERANCE)), 
							(water_mark_size.first.first / (water_mark_size.first.second - TOLERANCE))),
			second: Item::new((water_mark_size.second.first / (water_mark_size.second.second + TOLERANCE)), 
							(water_mark_size.second.first / (water_mark_size.second.second - TOLERANCE))),
			third: Item::new(((water_mark_size.third.first - TOLERANCE) / water_mark_size.third.second), 
							(water_mark_size.third.first / (water_mark_size.third.second - TOLERANCE))),
			forth: Item::new(((water_mark_size.forth.first - TOLERANCE) / water_mark_size.forth.second), 
							(water_mark_size.forth.first / (water_mark_size.forth.second + TOLERANCE))),
			fifth: Item::new(((water_mark_size.fifth.first - TOLERANCE) / water_mark_size.fifth.second), 
							(water_mark_size.fifth.first / (water_mark_size.fifth.second - TOLERANCE))),
		};

		let water_mark_root_path = Path::new(WATER_MARK_ROOT);
		let mut water_mark_template_type = std::string::String::new();

		if base_ratio >= water_mark_size_ratio.first.first && base_ratio <= water_mark_size_ratio.first.second {
			water_mark_template_type.push_str(water_mark_type.first);
		} else if base_ratio >= water_mark_size_ratio.second.first && base_ratio <= water_mark_size_ratio.second.second {
			water_mark_template_type.push_str(water_mark_type.second);
		} else if base_ratio >= water_mark_size_ratio.third.first && base_ratio <= water_mark_size_ratio.third.second {
			water_mark_template_type.push_str(water_mark_type.third);
		} else if base_ratio >= water_mark_size_ratio.forth.first && base_ratio <= water_mark_size_ratio.forth.second {
			water_mark_template_type.push_str(water_mark_type.forth);
		} else if base_ratio >= water_mark_size_ratio.fifth.first && base_ratio <= water_mark_size_ratio.fifth.second {
			water_mark_template_type.push_str(water_mark_type.fifth);
		} else {
			println!("{:?}", "UFO object detected!");
			return None;
		}

		let water_mark_template_path = water_mark_root_path.join(Path::new(&water_mark_template_type));
		let water_mark_template = match image::open(&water_mark_template_path) {
			Ok(img) => Box::new(img),
	    	Err(_) => {
	    		println!("{:?}", "Somthing Wrong with template image!");
	    		return None;
	    	},
		};

		let out_put = ImageWork {
			source: base,
			template: water_mark_template,
			output: new,
			output_file: output_file,

			width: width,
			height: height,

			opacity: WATER_MARK_OPACITY,
		};

		Some(out_put)
	}

	pub fn pixel_operation(&mut self) {;
		for x in 0..self.width {
			for y in 0..self.height {
				let r = self.template.get_pixel(x, y).channels4().0;
	    		let g = self.template.get_pixel(x, y).channels4().1;
	    		let b = self.template.get_pixel(x, y).channels4().2;

	    		if r != 0 && g != 0 && b != 0 {
	    			let (nr, ng, nb, na) = self.pixel_blend(x, y);
	    			self.output.put_pixel(x, y, image::Rgba::<u8>{data: [nr, ng, nb, na]});
	    		} else {
	    			self.output.put_pixel(x, y, self.source.get_pixel(x, y));
	    		}
			}
		}
	}

	pub fn image_save(&mut self) {
		let ref mut out_put_file = File::create(&Path::new(&self.output_file)).unwrap();
    	let _ = self.output.save(out_put_file, image::JPEG);
	}

    fn pixel_blend(&mut self, width: u32, height: u32) -> (u8, u8, u8, u8) {
		let (sr, sg, sb, _) = self.source.get_pixel(width, height).channels4();
		let (tr, tg, tb, _) = self.template.get_pixel(width, height).channels4();

		let (fsr, fsg, fsb): (f64, f64, f64) = self.pixel_normalize(sr, sg, sb);
		let (ftr, ftg, ftb): (f64, f64, f64) = self.pixel_normalize(tr, tg, tb);

		let nr: f64 = 1.0 - ((1.0 - fsr) * (1.0 - ftr * self.opacity));
		let ng: f64 = 1.0 - ((1.0 - fsg) * (1.0 - ftg * self.opacity));
		let nb: f64 = 1.0 - ((1.0 - fsb) * (1.0 - ftb * self.opacity));

		let rr: u8 = self.pixel_clamp((nr * 255.0));
		let rg: u8 = self.pixel_clamp((ng * 255.0));
		let rb: u8 = self.pixel_clamp((nb * 255.0));

		(rr, rg, rb, 255)
	}

	fn pixel_clamp(&mut self, input: f64) -> u8 {
		let output = match input {
			inside @ 0f64...255f64 => {
				inside
			},
			outside @ _ => {
				if outside > 255.0 {
					255.0
				} else {
					0.0
				}
			},
		};
		output as u8
	}

	fn pixel_normalize(&mut self, r: u8, g: u8, b:u8) -> (f64, f64, f64) {
		let nr: f64 = r as f64 / 255.0;
		let ng: f64 = g as f64 / 255.0;
		let nb: f64 = b as f64 / 255.0;
		(nr, ng, nb)
	}
}

pub fn fractal_generation() {
	let max_iterations = 2048u16;

    let imgx = 1280;
    let imgy = 1280;

    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cy = y as f32 * scaley - 2.0;
        let cx = x as f32 * scalex - 2.0;

        let mut z = Complex::new(cx, cy);
        let c = Complex::new(-0.835, -0.2321);

        let mut i = 0;

        for t in (0..max_iterations) {
            if z.norm() > 4.0 {
                break
            }
            z = z * z + c;
            i = t;
        }

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = image::Luma([(i * 2) as u8]);

    }

    // Save the image as “fractal.png”
    let ref mut fout = std::fs::File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}
