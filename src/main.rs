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

mod indices;
use indices::*;

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
    println!("    evi        : enhanced vegetation index");
    println!("    ndvi       : normalised difference vegetation index");
    println!("    gndvi      : green normalised difference vegetation index");
    println!("    msi        : moisture stress index");
    println!("    ndwi       : normalised difference water index");
    println!("    ndbi       : normalised difference built-up index");
    println!("    ndmi       : normalised difference mud index");
    println!("    atsavi     : adjusted transformed soil-adjusted VI");
    println!("    afri1600   : aerosol free vegetation index 1600");
    println!("    afri2100   : aerosol free vegetation index 2100");
    println!("    ari        : anthocyanin reflectance index");
    println!("    avi        : ashburn vegetation index");
    println!("    arvi2      : atmospherically resistant vegetation index 2");
    println!("    bri        : browning reflectance index");
    println!("    chlgreen   : chloropyll green");
    println!("    cigreen    : chloropyll index green");
    println!("    cirededge  : chloropyll indexrededge");
    println!("    chlrededge : chloropyll red-edge");
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
        "atsavi" => (["B05", "B09"].to_vec(), atsavi),
        "afri1600" => (["B09", "B11"].to_vec(), afri1600),
        "afri2100" => (["B09", "B12"].to_vec(), afri2100),
        "ari" => (["B03", "B05"].to_vec(), ari),
        "avi" => (["B04", "B09"].to_vec(), avi),
        "arvi2" => (["B05", "B09"].to_vec(), arvi2),
        "bri" => (["B03", "B05", "B09"].to_vec(), bri),
        "chlgreen" => (["B03", "B07"].to_vec(), chlgreen),
        "cigreen" => (["B03", "B09"].to_vec(), cigreen),
        "cirededge" => (["B05", "B09"].to_vec(), cirededge),
        "chlrededge" => (["B05", "B07"].to_vec(), chlrededge),
        _ => {
            usage();
            panic!("unknown index name: {}_", index_name)
        },
    };

    let out_image: GrayImage = calculate_index(images_dir_path, output_image_width,
                                               output_image_height, &band_nums, index_eq);

    out_image.save(output_image_path).unwrap();
}
