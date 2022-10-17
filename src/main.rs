use std::{
    env,
    fs,
    path
};

use jpeg2k::{
    Image,
    ImageData
};

use image::{
    ImageBuffer,
    GrayImage,
    Luma
};

use regex::Regex;

// enhanced vegetation index
fn evi(one_px_data: Vec<u8>) -> u8 {
    let b02: f64 = one_px_data[0] as f64 / 255.0;
    let b04: f64 = one_px_data[1] as f64 / 255.0;
    let b08: f64 = one_px_data[2] as f64 / 255.0;
    let result: f64 = (b08 - b04) / (b08 + (6.0 * b04) - (7.5 * b02) + 1.0);
    (result * 255.0) as u8
}

// normalised difference vegetation index
fn ndvi(one_px_data: Vec<u8>) -> u8 {
    let b04: f64 = one_px_data[0] as f64 / 255.0;
    let b08: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b08 - b04) / (b08 + b04);
    (result * 255.0) as u8
}

// green normalised difference vegetation index
fn gndvi(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b08: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b08 - b03) / (b08 + b03);
    (result * 255.0) as u8
}

// moisture stress index
fn msi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = b11 / b08;
    (result * 255.0) as u8
}

// normalised difference water index
fn ndwi(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b03 - b11) / (b03 + b11);
    (result * 255.0) as u8
}

// normalised difference built-up index
fn ndbi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b11 - b08) / (b11 + b08);
    (result * 255.0) as u8
}

// normalised difference mud index
fn ndmi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b09 - b08) / (b09 + b08);
    (result * 255.0) as u8
}

fn get_image(image_paths: fs::ReadDir, band_num: &str) -> Option<Image> {
    for image_path in image_paths {
        let image_path_path: &path::PathBuf = &image_path.unwrap().path();
        let image_path_string: &str = image_path_path.to_str().unwrap();
        let band_string_to_match: Regex = Regex::new(band_num).unwrap();
        if band_string_to_match.is_match(image_path_string) {
            let jpeg2k_img: Image = Image::from_file(image_path_string)
                .expect("failed to load jpeg 2000 image.");
            return Some(jpeg2k_img);
        }
    }
    None
}

fn calculate_index<F>(images_dir_path: &String, output_image_width: u32, output_image_height: u32,
                      band_nums: &[&str], index_eq: F) -> GrayImage
    where F: Fn(Vec<u8>) -> u8 {
    let band_imgs: Vec<Image> = band_nums.iter().map(|band_num| {
        get_image(fs::read_dir(images_dir_path).unwrap(), band_num)
            .expect(&format!("no image file with band number '{}'", band_num))
    }).collect();

    let band_imgs_px: Vec<ImageData> = band_imgs.iter().map(|i| i.get_pixels(None).unwrap()).collect();

    let mut out_image: ImageBuffer<Luma<u8>, Vec<u8>> = GrayImage::new(output_image_width,
                                                                       output_image_height);
    for y in 0..output_image_height {
        for x in 0..output_image_width {
            let one_px_data: Vec<u8> = band_imgs_px.iter().map(|i| {
                    let px_x_index: f64 = ((x as f64/output_image_width as f64)/(1.0/i.width as
                                                                                 f64)).floor();
                    let px_y_index: f64 = ((y as f64/output_image_height as f64)/(1.0/i.height as
                                                                                  f64)).floor();
                    let px_index: f64 = (px_y_index*i.height as f64)+px_x_index;
                    return i.data[px_index as usize];
            }).collect();
            out_image.put_pixel(x, y, Luma([index_eq(one_px_data)]));
        }
    }

    out_image
}

fn usage() {
    println!("usage:\n");
    print!("    sentinel-2-indices [images_directory_path] [index] [output_image_path] ");
    print!("[output_image_width] [output_image_height]\n\n");
    println!("available indices:\n");
    println!("    evi   : enhanced vegetation index");
    println!("    ndvi  : normalised difference vegetation index");
    println!("    gndvi : green normalised difference vegetation index");
    println!("    msi   : moisture stress index");
    println!("    ndwi  : normalised difference water index");
    println!("    ndbi  : normalised difference built-up index");
    println!("    ndmi  : normalised difference mud index");
}

fn main() {
    if env::args().len() != 6 {
        usage();
        panic!("incorrect arguments")
    }

    let args: Vec<String> = env::args().collect();
    let images_dir_path: &String = &args[1];
    let index_name: &String = &args[2];
    let output_image_path: &String = &args[3];
    let output_image_width: u32 = args[4].parse::<u32>().unwrap();
    let output_image_height: u32 = args[5].parse::<u32>().unwrap();

    let (band_nums, index_eq): (Vec<&str>, fn(Vec<u8>) -> u8) = match index_name.as_ref() {
        "evi" => (["B02", "B04", "B08"].to_vec(), evi),
        "ndvi" => (["B04", "B08"].to_vec(), ndvi),
        "gndvi" => (["B03", "B08"].to_vec(), gndvi),
        "msi" => (["B08", "B11"].to_vec(), msi),
        "ndwi" => (["B03", "B11"].to_vec(), ndwi),
        "ndbi" => (["B08", "B11"].to_vec(), ndbi),
        "ndmi" => (["B08", "B09"].to_vec(), ndmi),
        _ => {
            usage();
            panic!("unknown index name: {}_", index_name)
        },
    };

    let out_image: GrayImage = calculate_index(images_dir_path, output_image_width,
                                               output_image_height, &band_nums, index_eq);

    out_image.save(output_image_path).unwrap();
}
