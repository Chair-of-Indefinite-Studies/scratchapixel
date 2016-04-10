extern crate scratchapixel;

use std::fs::File;

use scratchapixel::ppm::rgb::RGB;
use scratchapixel::ppm::format::PPM;


fn main() {
    let white = RGB { r: 255, g: 255, b: 255 };
    let black = RGB { r: 0, g: 0, b: 0 };
    let red   = RGB { r: 255, g: 0, b: 0 };
    let mut image: PPM = PPM::new(64, 64);
    for y in 0..64 {
        let parity_y = (y / 8) % 2;
        for x in 0..64 {
            let parity_x = (x / 8) % 2;
            let color = match (parity_y + parity_x) % 2 {
                0 => black,
                1 => white,
                _ => red
            };
            image.set_pixel(x, y, color);
        }
    }

    match File::create("resources/example.use_ppm.ppm") {
        Ok(file) => {
            match image.write_file(file) {
                Ok(_) => println!("wrote file"),
                Err(_) => println!("could not write file")
            }
        }
        Err(_) => println!("could not create file")
    }
}
