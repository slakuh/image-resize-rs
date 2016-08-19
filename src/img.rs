extern crate image;

use constants;
use image::{GenericImage, ImageFormat};
use image::FilterType;
//use std::io::{self, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread;
use args_pars::Arguments;

#[derive(Debug, Copy, Clone)]
pub enum Resize {
    // samo �e pove�ati sliku
    Increase,
    // samo �e smanjiti sliku
    Decrease,
    // pove�at ili umanjit sliku na �eljenu veli�inu
    Eather,
    // ne�e mijenjati veli�inu slike
    Neather,
}

#[derive(Debug, Copy, Clone)]
pub struct ImgSize
{
    pub width: u32,
    pub height: u32,
}

impl ImgSize {
    
    fn new(width: u32, height: u32) -> ImgSize
    {
        ImgSize{width: width, height: height}
    }
    
    fn resize_to_width(&mut self, new_width: u32,  rsz_type: &Resize)
    {
        let calc_new_size;// = false;
        match rsz_type {
            &Resize::Increase => calc_new_size = self.width < new_width,                
            &Resize::Decrease => calc_new_size =  self.width > new_width,
            &Resize::Eather => calc_new_size = true,
            &Resize::Neather => calc_new_size = false,
        }
            //_ => println!("NE DIRAJ"),
            if calc_new_size
            {
                self.height = new_width * self.height / self.width;
                self.width = new_width;
            }
            //println!("img::resize_to_width\nwidith: {}, height: {}", self.width, self.height);        
    }
    
    fn resize_to_height(&mut self, new_height: u32, rsz_type: &Resize)
    {
        let calc_new_size;// = false;

        match rsz_type {
            &Resize::Increase => calc_new_size = self.height < new_height,
            &Resize::Decrease => calc_new_size = self.height > new_height,
            &Resize::Eather => calc_new_size = true,    
            &Resize::Neather => calc_new_size = false,                   
        }

	    if calc_new_size
	    {
            self.width = new_height * self.width / self.height;
            self.height = new_height;
        }
    }
}
/*
pub fn resize_threaded(arguments: Arguments)
{
    let mut size_from_arguments = ImgSize {
        width: arguments.width,
        height: arguments.height,
    };    
    
    // u slučaju da argumentima nije određena vrijednost koristit će defaultnu vrijednost
    if size_from_arguments.width == 0 && size_from_arguments.height == 0
    {
        size_from_arguments.width = constants::OUTPUT_WIDTH;
    }

    let mut threads = Vec::new();

    for path in &arguments.images
    {
        if is_supported(&path)
        {
            threads.push(thread::spawn(move || {
                resize_image (
                    size_from_arguments,
                   *path,
                    arguments.filter,
                    arguments.image_format,
                    arguments.resize,
                );
        
            }));
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }
}
*/
pub fn resize(arguments: &Arguments)
{
    let mut size_from_arguments = ImgSize {
        width: arguments.width,
        height: arguments.height,
    };    
    
    // u slučaju da argumentima nije određena vrijednost koristit će defaultnu vrijednost
    if size_from_arguments.width == 0 && size_from_arguments.height == 0
    {
        size_from_arguments.width = constants::OUTPUT_WIDTH;
    }

    for path in &arguments.images
    {
        if is_supported(&path)
        {
            resize_image (
                size_from_arguments,
                path,
                arguments.filter,
                arguments.image_format,
                arguments.resize
            );
        }
    }

}

fn resize_image(size:ImgSize, path: &PathBuf, filter: FilterType, image_format: ImageFormat, resize: Resize)
{
    if let Some(img) = image::open(&path).ok()
    {
        let mut image_size= ImgSize::new(img.dimensions().0, img.dimensions().1);
        let image_size_original = image_size.clone();

        if size.width != 0 && size.height != 0
        {
            if image_size.width > image_size.height
            {
                image_size.resize_to_width(size.width, &resize);
            }
            else
            {
                image_size.resize_to_height(size.height, &resize);
            }
        }        
        else if size.width != 0
        {            
            image_size.resize_to_width(size.width, &resize);
        }
        else if size.height != 0
        {
            image_size.resize_to_height(size.height, &resize);   
        }

        /*
        // ukoliko veličina nije promjenjena testira da li je potrebna konverzija
        if image_size_original.width == image_size.width &&
           image_size_original.height == image_size.height
           {
               
               let ext_in = path.extension().unwrap().to_str().unwrap();
               let ext_out = image_format_to_string(image_format);
               
               // ukoiko su ulazna i izlazna externzija iste nije potrebna konverzija
               if ext_in == ext_out
               {
                   //println!("img::resize_image: {} == {}", ext_in, ext_out);
                   return;
               }

        } 
        
        */
        let resized_image = img.resize_exact(image_size.width, image_size.height, filter);

        let out_path_buf = out_file_name(&path, image_format);
        let out_path =  Path::new(out_path_buf.to_str().unwrap());//Path = Path::new(out_file_name(&path).to_str().unwrap());
        let ref mut fout = File::create(&out_path).unwrap();

        let _ = resized_image.save(fout, image_format).unwrap();
        println!("Image {:?}\nresized to: {}x{}\n", path.file_name().unwrap(), image_size.width, image_size.height);

    } else {
        println!("error opening image {:?}", path);
    }
}
/*
pub fn resize_img(size: ImgSize, file_vec: Vec<PathBuf>, filter: image::FilterType, num_threads: u32)
{
    //println!("");
    //let size_out = ImgSize{width:0, height:0};

    let mut threads = Vec::new();
    let mut i: u32 = 0;

    for path_buf in file_vec
    {
        i += 1;
        threads.push(thread::spawn(move || {

            let path = Path::new(path_buf.to_str().unwrap());
            if is_supported(&path)
            {

                if let Some(img) = image::open(&path).ok()
                {
                    // The dimensions method returns the images width and height
                    let size_in = ImgSize{width: img.dimensions().0, height: img.dimensions().1};
                    
                    let size_out =  calc_resize(&size_in, &size);

                    //print!(" |"); // println!("File {:?} is supported.", path);
                    //print!(" |"); // println!("Resizing image from {}x{} to {}x{}.", size_in.width, size_in.height, size_out.width, size_out.height);
                    //io::stdout().flush().unwrap();
                    let resized_image = img.resize_exact(size_out.width, size_out.height, filter);

                    let out_path_buf = out_file_name(&path, ImageFormat::JPEG);
                    let out_path =  Path::new(out_path_buf.to_str().unwrap());//Path = Path::new(out_file_name(&path).to_str().unwrap());
                    let ref mut fout = File::create(&out_path).unwrap();

                    // Write the contents of this image to the Writer in JPEG format.
                    let _ = resized_image.save(fout, image::JPEG).unwrap();
                    print!("|");
                    io::stdout().flush().unwrap();
                }
                else
                {
                    print!("X");
                    io::stdout().flush().unwrap();
                    //println!("\nError opening {:?}\nSkipping file...\n", path.file_name());
                }
            }

        /*
        else
        {
           println!("Unsupported file type: {:?}\nSkipping file...\n", path);
        }
        */
        }));

        if i % num_threads == 0
        {
            for thread in threads
            {
                thread.join().unwrap();
            }
            threads = Vec::new();
        }
    }

    for thread in threads {
    thread.join().unwrap();
    }
}
*/
fn out_file_name(path: &Path, image_format: ImageFormat) -> PathBuf
{
    let suffiks = "-m";
    let name: &str = path.file_stem().unwrap().to_str().unwrap();// + suffiks + ".jpg";
    


    let out_name = String::from(name) + suffiks + "." + &image_format_to_string(image_format);
    let mut path_buf = PathBuf::from(path);
    path_buf.set_file_name(&out_name);
    //print!(" |"); // println!("New file name: {:?}\n", path_buf);
    //io::stdout().flush().unwrap();
    path_buf
    //Path::new(path_buf.to_str().unwrap())
}

fn image_format_to_string(image_format: ImageFormat) -> String
{
    let extension: String;
    match image_format {
        ImageFormat::JPEG => extension = "jpg".to_string(),
        ImageFormat::PNG => extension = "png".to_string(),
        _ => panic!("Unsuported image format"),
    }
    extension
}

fn is_supported(path: &Path) -> bool
{
    if let Some(ext_osstr) = path.extension()
    {
        let ext: &str = &string_to_lower(ext_osstr.to_str().unwrap());
        let suported_files = constants::SUPPORTED_FILES;//["jpg","png","jpeg", "gif", "bmp", "tiff"]; // ovo je definirano kao konstanta
        for i in 0..suported_files.len()
        {
            if ext == suported_files[i]
            {
                return true;
            }
        }
    }
    false
}

fn string_to_lower(s: &str) -> String
{
    s.to_string().to_lowercase()
}

/*
fn calc_resize(size_in: &ImgSize, size_out: &ImgSize) -> ImgSize
{
    let mut new_size = ImgSize{width:0, height:0};

    if size_out.width == 0
    {
        new_size.width = size_in.width*size_out.height/size_in.height;
        if new_size.width % 2 != 0
        {
           new_size.width += 1;
        }

        new_size.height = size_out.height;
    }
    else if size_out.height == 0
    {
        new_size.width = size_out.width;

        new_size.height = size_out.width*size_in.height/size_in.width;
        if new_size.height % 2 != 0
        {
           new_size.height += 1;
        }
    }
    else
    {
        new_size.width = size_out.width;
        new_size.height = size_out.height;
    }

    new_size
}
*/