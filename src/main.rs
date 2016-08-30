// #![allow(dead_code)]
// #![allow(unused_variables)]

extern crate image;
extern crate rayon;
extern crate user;

mod args_pars;
mod constants;
mod help;
mod img_2;
use args_pars::Arguments;

// fn pause() {
// println!("\nPritisni ENTER za izlaz: ");
// io::stdout().flush().unwrap();
// let mut input = String::new();
// io::stdin().read_line(&mut input).unwrap();
// }
//
fn main() {

    // prekinut će program ukoliko user nije definiran u user modulu
    user::user();

    println!("\n • Use \"-help\" argument for more information.\n");
    // let arg = Arguments::new();
    img_2::resize_images(Arguments::new());

    println!("\n\t{}", constants::AUTHOR);
}
