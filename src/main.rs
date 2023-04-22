extern crate image;

use colors_transform::{Color, Rgb};
use image::GenericImageView;
use std::fs;
use std::path::Path;
#[derive(Clone)]
struct Point {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}
struct ImgInfo {
    width: u32,
    height: u32,
    points: Vec<Point>,
}
fn process_image(input_path: &str) -> Option<ImgInfo> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let mut points: Vec<Point> = Vec::new();
    let width = img.as_ref().unwrap().width();
    let height = img.as_ref().unwrap().height();

    for p in img.as_ref().unwrap().pixels() {
        // print rgb value of pixel
        points.push(Point {
            x: p.0,
            y: height - p.1,
            r: p.2[0],
            g: p.2[1],
            b: p.2[2],
        });
    }
    return Some(ImgInfo {
        points,
        width,
        height,
    });
}
fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Please enter image path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let img_info_result = process_image(&s);
    if img_info_result.is_none() {
        println!("Could not process image {}", s);
        return;
    }
    let img_info = img_info_result.unwrap();
    let mut min_found_lightness: f32 = 80.0;
    let mut max_found_lightness: f32 = 20.0;
    for p in img_info.points.iter() {
        let rgb = Rgb::from(p.r.into(), p.g.into(), p.b.into());
        let hsl = rgb.to_hsl();
        let pixel_lightness = hsl.get_lightness();
        if pixel_lightness < min_found_lightness {
            min_found_lightness = pixel_lightness;
        }
        if pixel_lightness > max_found_lightness {
            max_found_lightness = pixel_lightness;
        }
    }

    let mut input_contrast = String::new();
    let contrast_val: f32;
    print!("Please enter contrast value (from -100 to 100): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_contrast)
        .expect("Did not enter a correct string");
    if let Some('\n') = input_contrast.chars().next_back() {
        input_contrast.pop();
    }
    if let Some('\r') = input_contrast.chars().next_back() {
        input_contrast.pop();
    }

    let trimmed = input_contrast.trim();
    match trimmed.parse::<i16>() {
        Ok(i) => {
            if i > 100 || i < -100 {
                println!("Contrast value must be from -100 to 100");
                return;
            }
            contrast_val = -(i as f32) / 2.0;
        }
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            return;
        }
    };

    let max_lightness_cap: f32 = 100.0 - contrast_val;
    let min_lightness_cap: f32 = contrast_val;
    let new_img_points = img_info
        .points
        .iter()
        .map(|p| {
            let rgb = Rgb::from(p.r.into(), p.g.into(), p.b.into());
            let hsl = rgb.to_hsl();
            let pixel_lightness = hsl.get_lightness();
            let mut new_lightness = (pixel_lightness - min_found_lightness)
                * ((max_lightness_cap - min_lightness_cap)
                    / (max_found_lightness - min_found_lightness))
                + min_lightness_cap;
            if new_lightness > 100.0 {
                new_lightness = 100.0;
            }
            if new_lightness < 0.0 {
                new_lightness = 0.0;
            }
            let transformed = hsl.set_lightness(new_lightness);
            let new_rgb = transformed.to_rgb();
            Point {
                x: p.x,
                y: p.y,
                r: new_rgb.get_red() as u8,
                g: new_rgb.get_green() as u8,
                b: new_rgb.get_blue() as u8,
            }
        })
        .collect::<Vec<Point>>();

    let mut imgbuf = image::ImageBuffer::new(img_info.width * 2, img_info.height);
    for p in new_img_points.iter() {
        let pixel = imgbuf.get_pixel_mut(p.x + img_info.width, img_info.height - p.y);
        *pixel = image::Rgb([p.r, p.g, p.b]);
    }
    for p in img_info.points.iter() {
        let pixel = imgbuf.get_pixel_mut(p.x, img_info.height - p.y);
        *pixel = image::Rgb([p.r, p.g, p.b]);
    }
    fs::create_dir_all("images").unwrap();
    imgbuf.save("images/comparison.png").unwrap();
}
