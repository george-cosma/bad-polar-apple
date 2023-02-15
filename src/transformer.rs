use std::path::Path;

use image::{self, Luma};

use crate::{line_sequence, sequence::Sequence};

/// Transform a frame sequence into the "polar" representation.
///
/// # Panics
///
/// Panics if the app does not have write access to the "out" folder.
pub fn transform_sequence(seq: &mut Sequence) {
    while let Some(i) = seq.next() {
        let filename = format!("{}{:0>4}{}", seq.prefix, i, seq.suffix);
        let mut img = image::open(&filename).unwrap().grayscale();

        let path_str = format!("./out/frame.{:0>4}{}", i, seq.suffix);
        let path = Path::new(&path_str);
        println!("Processing {:?}", path);

        let new_img = transform_frame(img.as_mut_luma8().unwrap());
        new_img.save(path).unwrap();
    }
}

const STEP:usize = 4;

pub fn transform_frame(img: &mut image::GrayImage) -> image::GrayImage {
    let mut new_img = image::GrayImage::new(img.width(), img.height());
    
    let center_color = (((img.get_pixel(img.width() / 2, img.height() / 2).0[0] as f64) / 255.0).round() as u8) * 255;

    // let bg_color = get_bg_color(img);
    // let fg_color = invert_color(bg_color);
    let bg_color = invert_color(center_color);
    let fg_color = center_color;
    new_img.fill(bg_color);

    // TOP MARGIN
    for x0 in (0..img.width()).step_by(STEP) {
        let seq = line_sequence::plot_line(x0, 0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        for (x, y) in &edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let first_fill = get_edge_line(img, &edge, fg_color);
        for (x, y) in &first_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let second_edge = get_edge_line(img, &first_fill, bg_color);
        for (x, y) in &second_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let second_fill = get_edge_line(img, &second_edge, fg_color);
        for (x, y) in &second_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let third_edge = get_edge_line(img, &second_fill, bg_color);
        for (x, y) in &third_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }
    }
    // LEGT MARGIN
    for y0 in (0..img.height()).step_by(STEP) {
        let seq = line_sequence::plot_line(0, y0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        for (x, y) in &edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let first_fill = get_edge_line(img, &edge, fg_color);
        for (x, y) in &first_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let second_edge = get_edge_line(img, &first_fill, bg_color);
        for (x, y) in &second_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let second_fill = get_edge_line(img, &second_edge, fg_color);
        for (x, y) in &second_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let third_edge = get_edge_line(img, &second_fill, bg_color);
        for (x, y) in &third_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }
    }
    // BOTTOM MARGIN
    for x0 in (0..img.width()).step_by(STEP) {
        let seq = line_sequence::plot_line(x0, img.height() - 1, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        for (x, y) in &edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let first_fill = get_edge_line(img, &edge, fg_color);
        for (x, y) in &first_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let second_edge = get_edge_line(img, &first_fill, bg_color);
        for (x, y) in &second_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let second_fill = get_edge_line(img, &second_edge, fg_color);
        for (x, y) in &second_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let third_edge = get_edge_line(img, &second_fill, bg_color);
        for (x, y) in &third_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }
    }
    // RIGHT MARGIN
    for y0 in (0..img.height()).step_by(STEP) {
        let seq = line_sequence::plot_line(img.width() - 1, y0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        for (x, y) in &edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let first_fill = get_edge_line(img, &edge, fg_color);
        for (x, y) in &first_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let second_edge = get_edge_line(img, &first_fill, bg_color);
        for (x, y) in &second_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }

        let second_fill = get_edge_line(img, &second_edge, fg_color);
        for (x, y) in &second_fill {
            new_img.put_pixel(*x, *y, Luma([bg_color]));
        }

        let third_edge = get_edge_line(img, &second_fill, bg_color);
        for (x, y) in &third_edge {
            new_img.put_pixel(*x, *y, Luma([fg_color]));
        }
    }
    return new_img;
}

/// Returns the average color on the outside border (0 or 255)
fn get_bg_color(img: &image::GrayImage) -> u8 {
    let mut avg: f64 = 0.0;
    let mut pixels: u32 = 0;

    // Top and bottom
    for x in 0..img.width() {
        avg += img.get_pixel(x, 0).0[0] as f64;
        avg += img.get_pixel(x, img.height() - 1).0[0] as f64;
        pixels += 2;
    }

    // left and right
    for y in 1..(img.height()-1) {
        avg += img.get_pixel(0, y).0[0] as f64;
        avg += img.get_pixel(img.width() - 1, y).0[0] as f64;
        pixels += 2;
    }

    return ((avg/(pixels as f64)/255.0).round() as u8) * 255;
}

/// Inverts the color given as a luminace value.
fn invert_color(color: u8) -> u8 {
    u8::MAX - color
}

const THRESHOLD: u8 = 200;
/// From a given line, returns all the points that span from the center to the last pixel that is different from the background color.
fn get_edge_line(img: &mut image::GrayImage, seq: &Vec<(u32, u32)>, bg_color:u8) -> Vec<(u32, u32)> {
    for (index, (x, y)) in seq.iter().enumerate() {
        let color = img.get_pixel(*x, *y).0[0];
        if (bg_color < u8::MAX - THRESHOLD && bg_color + THRESHOLD < color)
            || (bg_color > u8::MIN + THRESHOLD && bg_color - THRESHOLD > color)
        {
            return seq[index..seq.len()].to_vec();
        }
    }
    return vec![]
}
