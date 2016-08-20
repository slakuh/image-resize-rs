extern crate image;

use constants;
use image::{GenericImage, ImageFormat};
use image::FilterType;
// use std::io::{self, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::thread;
use args_pars::Arguments;

#[derive(Copy, Clone)]
pub enum Resize {
    // samo će povečati sliku
    Increase,
    // samo će smanjiti sliku
    Decrease,
    // povećat ili umanjit sliku na ćeljenu veličinu
    Either,
    // neće mijenjati veličinu slike
    Neither,
}

pub struct ImageSize {
    pub width: u32,
    pub height: u32,
    path: PathBuf,
    resize: Resize,
    filter: FilterType,
    image_format: ImageFormat,
}

impl ImageSize {
    pub fn new() -> ImageSize {
        ImageSize {
            width: constants::OUTPUT_WIDTH,
            height: constants::OUTPUT_HEIGHT,
            path: PathBuf::new(),
            resize: Resize::Decrease,
            image_format: ImageFormat::JPEG,
            filter: FilterType::Nearest,
        }
    }

    pub fn from(path: PathBuf, arguments: &Arguments) -> ImageSize {
        ImageSize {
            width: arguments.width,
            height: arguments.height,
            path: path,
            resize: arguments.resize,
            image_format: arguments.image_format,
            filter: arguments.filter,
        }
    }

    fn resize_to_width(&mut self, new_width: u32) -> bool {
        let calc_new_size;// = false;
        match self.resize {
            Resize::Increase => calc_new_size = self.width < new_width,                
            Resize::Decrease => calc_new_size = self.width > new_width,
            Resize::Either => calc_new_size = true,
            Resize::Neither => calc_new_size = false,
        }

        if calc_new_size {
            self.height = new_width * self.height / self.width;
            self.width = new_width;
        }
        calc_new_size
    }

    fn resize_to_height(&mut self, new_height: u32) -> bool {
        let calc_new_size;// = false;

        match self.resize {
            Resize::Increase => calc_new_size = self.height < new_height,
            Resize::Decrease => calc_new_size = self.height > new_height,
            Resize::Either => calc_new_size = true,    
            Resize::Neither => calc_new_size = false,                   
        }

        if calc_new_size {
            self.width = new_height * self.width / self.height;
            self.height = new_height;
        }
        calc_new_size
    }

    pub fn resize_image(&mut self) {
        if !self.is_supported() {
            println!("File is not supported:\n{:?}\n", self.path);
            return;
        }

        if let Some(img) = image::open(&self.path).ok() {
            let mut width_from_args = self.width;
            let mut height_from_args = self.height;
            let mut is_size_changed = true;

            if width_from_args == 0 && height_from_args == 0 {
                width_from_args = constants::OUTPUT_WIDTH;
                height_from_args = constants::OUTPUT_HEIGHT;
            }

            // ovdje vrijesnost structa od argumenat mijenjam u velićinu slike
            self.width = img.dimensions().0;
            self.height = img.dimensions().1;

            if width_from_args != 0 && height_from_args != 0 {
                if self.width > self.height {
                    is_size_changed = self.resize_to_width(width_from_args);
                } else {
                    is_size_changed = self.resize_to_height(height_from_args);
                }
            } else if width_from_args != 0 {
                is_size_changed = self.resize_to_width(width_from_args);
            } else if height_from_args != 0 {
                is_size_changed = self.resize_to_height(height_from_args);
            } else {
                unreachable!("img_2::ImageSize::resize_image");
            }

            let out_path_buf = out_file_name(&self.path, self.image_format);
            let out_path = Path::new(out_path_buf.to_str().unwrap());

            // kopirat će i preimenovati file ukoliko se veličina nije promjenila i extenzija je ostala ista
            if !is_size_changed &&
               image_format_to_string(self.image_format) == self.file_extension() {
                let _ = fs::copy(self.path.clone(), out_path);
            } else {
                let ref mut fout = File::create(&out_path).unwrap();
                let resized_image = img.resize_exact(self.width, self.height, self.filter);
                let _ = resized_image.save(fout, self.image_format).unwrap();
                println!("Image {:?}\nresized to: {}x{}\n",
                         &self.path.file_name().unwrap(),
                         self.width,
                         self.height);
            }
        } else {
            println!("error opening image {:?}", &self.path);
        }
    }

    pub fn file_extension(&self) -> String {
        let mut extension = String::new();
        if let Some(ext_osstr) = self.path.extension() {
            extension = string_to_lower(ext_osstr.to_str().unwrap());
        }
        extension
    }

    pub fn is_supported(&self) -> bool {
        let ext: &str = &self.file_extension();
        let suported_files = constants::SUPPORTED_FILES;
        for i in 0..suported_files.len() {
            if ext == suported_files[i] {
                return true;
            }
        }
        false
    }
}

pub fn resize_images(mut args: Arguments) {
    let mut threads = Vec::new();
    let mut i: usize = 0;
    let paths = args.paths();
    args.paths_clear();
    for path in paths {
        let arg = args.clone();
        threads.push(thread::spawn(move || {
            let mut is = ImageSize::from(path, &arg);
            is.resize_image();
        }));

        // maximalan broj slika koje će se odjednom učitati i obraditi
        i += 1;
        if i % args.max_parallel_img == 0 {
            for thread in threads {
                thread.join().unwrap();
            }
            threads = Vec::new();
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

fn out_file_name(path: &Path, image_format: ImageFormat) -> PathBuf {
    let name: &str = path.file_stem().unwrap().to_str().unwrap();

    let out_name = String::from(name) + constants::FILE_NAME_SUFFIX + "." +
                   &image_format_to_string(image_format);
    let mut path_buf = PathBuf::from(path);
    path_buf.set_file_name(&out_name);

    path_buf
}

fn image_format_to_string(image_format: ImageFormat) -> String {
    let extension: String;
    match image_format {
        ImageFormat::JPEG => extension = "jpg".to_string(),
        ImageFormat::PNG => extension = "png".to_string(),
        _ => panic!("Unsuported image format"),
    }
    extension
}

fn string_to_lower(s: &str) -> String {
    s.to_string().to_lowercase()
}
