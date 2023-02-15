use std::{fs::File, path::Path};

use image::{self, GenericImageView, Luma};

use crate::{line_sequence, renderer, sequence::Sequence};

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
    
    let center_color = img.get_pixel(img.width()/2, img.height()/2).0[0];
    let bg_color =get_bg_color(img);
    let fg_color = inverse_color(bg_color);
    new_img.fill(bg_color);

    // TOP MARGIN
    for x0 in (0..img.width()).step_by(STEP) {
        let seq = line_sequence::plot_line(x0, 0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        //let color = if edge.len() == seq.len() { center_color } else { fg_color };
        for (x, y) in edge {
            new_img.put_pixel(x, y, Luma([color]));
        }
    }
    // LEGT MARGIN
    for y0 in (0..img.height()).step_by(STEP) {
        let seq = line_sequence::plot_line(0, y0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        //let color = if edge.len() == seq.len() { center_color } else { fg_color };
        for (x, y) in edge {
            new_img.put_pixel(x, y, Luma([color]));
        }
    }
    // BOTTOM MARGIN
    for x0 in (0..img.width()).step_by(STEP) {
        let seq = line_sequence::plot_line(x0, img.height() - 1, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        //let color = if edge.len() == seq.len() { center_color } else { fg_color };
        for (x, y) in edge {
            new_img.put_pixel(x, y, Luma([color]));
        }
    }
    // RIGHT MARGIN
    for y0 in (0..img.height()).step_by(STEP) {
        let seq = line_sequence::plot_line(img.width() - 1, y0, img.width() / 2, img.height() / 2);

        let edge = get_edge_line(img, &seq, bg_color);
        //let color = if edge.len() == seq.len() { center_color } else { fg_color };
        for (x, y) in edge {
            new_img.put_pixel(x, y, Luma([color]));
        }
    }
    return new_img;
}

fn get_bg_color(img: &image::GrayImage) -> u8 {
    let mut avg: f64 = 0.0;
    let mut pixels: u32 = 0;
    for x in 0..img.width() {
        avg += img.get_pixel(x, 0).0[0] as f64;
        avg += img.get_pixel(x, img.height() - 1).0[0] as f64;
        pixels += 2;
    }
    for y in 1..(img.height()-1) {
        avg += img.get_pixel(0, y).0[0] as f64;
        avg += img.get_pixel(img.width() - 1, y).0[0] as f64;
        pixels += 2;
    }

    return ((avg/(pixels as f64)/255f64).round() as u8) * 255;
}

fn inverse_color(color: u8) -> u8 {
    u8::MAX - color
}

const THRESHOLD: u8 = 100;
fn get_edge_line(img: &mut image::GrayImage, seq: &Vec<(u32, u32)>, bg_color:u8) -> Vec<(u32, u32)> {
    //let base_color: u8 = img.get_pixel(seq[0].0, seq[0].1).0[0];

    for (index, (x, y)) in seq.iter().enumerate() {
        // println!("({},{}): {}",x,y,img.get_pixel(x, y).0[0]);
        let color = img.get_pixel(*x, *y).0[0];
        if (base_color < u8::MAX - THRESHOLD && base_color + THRESHOLD < color)
            || (base_color > u8::MIN + THRESHOLD && base_color - THRESHOLD > color)
        {
            return seq[index..seq.len()].to_vec();
        }
    }
    return seq.clone();
}
