use projects::png::png;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut png = png::Png::new("PNG_transparency_demonstration_1.png");
    if png.is_png() {
        println!("This file is a png image");
    } else {
        println!("File is not a png image");
    }
}
