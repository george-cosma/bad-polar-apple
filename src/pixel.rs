// RGBA Pixel
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Pixel {
    pub fn getBrightness(&self) -> u8 {
        (R+R+B+G+G+G)/6
    }

    pub fn bufferFromPNG(input: Vec<u8>) -> Vec<Pixel> {
        let mut buf = vec![0; reader.output_buffer_size()];
    }
}