use std::{fs::File, io::Read};

use minifb::{Key, Window};

use crate::sequence::Sequence;

pub fn render_sequence(window: &mut Window, seq: &mut Sequence) -> () {
    let (width, height) = get_size();
    while let Some(i) = seq.next() {
        let buf = get_png_frame(format!("{}{:0>4}{}", seq.prefix, i, seq.suffix));

        window.update_with_buffer(&buf, width, height).unwrap();

        if window.is_key_down(Key::Escape) {
            break;
        }
    }
}

pub fn get_size() -> (usize, usize) {
    let decoder = png::Decoder::new(File::open("frames/frame.0001.png").unwrap());
    let reader = decoder.read_info().unwrap();
    (reader.info().width as usize, reader.info().height as usize)
}

pub fn get_png_frame(file: String) -> Vec<u32> {
    let decoder = png::Decoder::new(File::open(file).unwrap());

    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];

    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();
    // convert buffer to u32

    buf.chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect()
}

pub fn get_png_reader(file: String) -> png::Reader<File> {
    let decoder = png::Decoder::new(File::open(file).unwrap());

    decoder.read_info().unwrap()
}
