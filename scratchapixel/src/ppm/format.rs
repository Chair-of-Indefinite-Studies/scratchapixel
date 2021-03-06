use std::fs::File;
use std::io;
use std::io::Write;
use super::rgb::RGB;

pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl PPM {
    pub fn new(height: u32, width: u32) -> PPM {
        let size = 3 * height * width;
        let buffer = vec![0; size as usize];
        PPM { height: height, width: width, data: buffer }
    }

    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (3 * self.width * y) + (3 * x);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.data[offset + 0];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB { r: r, g: g, b: b })
            },
            None => None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset + 0] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            },
            None => false
        }
    }

    pub fn write_file(&self, mut file: File) -> io::Result<()> {
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.data));
        Ok(())
    }
}
