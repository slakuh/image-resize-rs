extern crate image;

use constants;
use image::ImageFormat;
use help;
use std::env;
use std::fs::metadata;
use std::path::PathBuf;
use img_2::{self, Resize};

#[derive(Clone)]
pub struct Arguments {
    pub width: u32,
    pub height: u32,
    // pub image_size: img::ImgSize,
    pub filter: image::FilterType,
    paths: Vec<PathBuf>,
    pub image_format: ImageFormat,
    pub resize: Resize,
    pub max_parallel_img: usize,
}

impl Arguments {
    pub fn new() -> Arguments {
        let mut arg = Arguments {
            width: 0,
            height: 0,
            filter: image::FilterType::Nearest,
            paths: Vec::new(),
            image_format: ImageFormat::JPEG,
            resize: Resize::Decrease,
            max_parallel_img: constants::MAX_PARALLEL_IMG, // thread_num(),
        };

        for argument in env::args().skip(1) {
            if let Some(index_0) = argument.chars().nth(0) {
                if index_0 == '-' {
                    println!("Input argument: {}", argument);

                    let str_vec: Vec<&str> = argument.split(':').collect();

                    match str_vec[0] {
                        "-h" => arg.height = str_vec[1].parse::<u32>().unwrap(),
                        "-w" => arg.width = str_vec[1].parse::<u32>().unwrap(),
                        "-help" => println!("{}", help::help()),
                        "-f" => arg.filter = filter(str_vec[1]),
                        "-if" => arg.image_format = image_format(str_vec[1]),
                        "-r" => arg.resize = resize_type(str_vec[1]),
                        "-about" => help::about(),
                        "-mpi" => arg.max_parallel_img = str_vec[1].parse::<usize>().unwrap(),
                        _ => println!("Unsupported argument: {}", argument),
                    }
                    continue;
                }
            }

            match metadata(&argument) {
                Err(e) => println!("Argument isn't file, error message: {}", e),
                Ok(v) => {
                    if v.is_file() {
                        arg.paths.push(PathBuf::from(argument));
                        continue; // znam da je nepotrebno ... možda ću dodavati još argumenata
                    }
                }            
            }
        }

        arg
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }

    pub fn paths_clear(&mut self) {
        self.paths.clear();
    }
}

fn image_format(s: &str) -> ImageFormat {
    let image_format: ImageFormat;
    match s {
        "jpg" => image_format = ImageFormat::JPEG,
        "png" => image_format = ImageFormat::PNG,
        _ => panic!("Unsupported argument: image format."),
    }

    image_format
}

fn resize_type(s: &str) -> img_2::Resize {
    let resize: img_2::Resize;
    match s {
        "0" => resize = Resize::Neather,
        "1" => resize = Resize::Decrease,
        "2" => resize = Resize::Increase,
        "3" => resize = Resize::Eather,
        _ => panic!("Unsupported argument: resize type."),
    }

    resize
}

fn filter(s: &str) -> image::FilterType {
    let ft: image::FilterType;//::Nearest;
    match s {
        "n" => ft = image::FilterType::Nearest,
        "t" => ft = image::FilterType::Triangle,
        "c" => ft = image::FilterType::CatmullRom,
        "g" => ft = image::FilterType::Gaussian,
        "l" => ft = image::FilterType::Lanczos3,
        _ => panic!("Unsupported argument filter"),
    }

    ft
}
