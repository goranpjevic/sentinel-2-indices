// the functions in this file calculate the output pixel value for each index.
// every function recieves the argument 'one_px_data' which is a vector containing the pixel values
// in the same location for each band that is required to calculate the output value of the index.
// the pixel values in the 'one_px_data' argument are sorted by the band number.

// enhanced vegetation index
pub fn evi(one_px_data: Vec<u8>) -> u8 {
    let b02: f64 = one_px_data[0] as f64 / 255.0;
    let b04: f64 = one_px_data[1] as f64 / 255.0;
    let b08: f64 = one_px_data[2] as f64 / 255.0;
    let result: f64 = (b08 - b04) / (b08 + (6.0 * b04) - (7.5 * b02) + 1.0);
    (result * 255.0) as u8
}

// normalised difference vegetation index
pub fn ndvi(one_px_data: Vec<u8>) -> u8 {
    let b04: f64 = one_px_data[0] as f64 / 255.0;
    let b08: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b08 - b04) / (b08 + b04);
    (result * 255.0) as u8
}

// green normalised difference vegetation index
pub fn gndvi(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b08: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b08 - b03) / (b08 + b03);
    (result * 255.0) as u8
}

// moisture stress index
pub fn msi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = b11 / b08;
    (result * 255.0) as u8
}

// normalised difference water index
pub fn ndwi(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b03 - b11) / (b03 + b11);
    (result * 255.0) as u8
}

// normalised difference built-up index
pub fn ndbi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b11 - b08) / (b11 + b08);
    (result * 255.0) as u8
}

// normalised difference mud index
pub fn ndmi(one_px_data: Vec<u8>) -> u8 {
    let b08: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b09 - b08) / (b09 + b08);
    (result * 255.0) as u8
}

// adjusted transformed soil-adjusted VI
pub fn atsavi(one_px_data: Vec<u8>) -> u8 {
    let b05: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let a: f64 = 1.22;
    let b: f64 = 0.03;
    let x: f64 = 0.08;
    let result: f64 = a * ((b09-(a*b05)-b) / ((a*b09)+b05-(a*b)+(x*(1.0+(a.powf(2.0))))));
    (result * 255.0) as u8
}

// aerosol free vegetation index 1600
pub fn afri1600(one_px_data: Vec<u8>) -> u8 {
    let b09: f64 = one_px_data[0] as f64 / 255.0;
    let b11: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = b09 - (0.66 * (b11 / (b09+(0.66*b11))));
    (result * 255.0) as u8
}

// aerosol free vegetation index 2100
pub fn afri2100(one_px_data: Vec<u8>) -> u8 {
    let b09: f64 = one_px_data[0] as f64 / 255.0;
    let b12: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = b09 - (0.5 * (b12 / (b09+(0.56*b12))));
    (result * 255.0) as u8
}

// anthocyanin reflectance index
pub fn ari(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b05: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (1.0 / b03) - (1.0 / b05);
    (result * 255.0) as u8
}

// ashburn vegetation index
pub fn avi(one_px_data: Vec<u8>) -> u8 {
    let b04: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (2.0 * b09) - b04;
    (result * 255.0) as u8
}

// atmospherically resistant vegetation index 2
pub fn arvi2(one_px_data: Vec<u8>) -> u8 {
    let b05: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (-0.18) + (1.17 * ((b09-b05)/(b09+b05)));
    (result * 255.0) as u8
}

// browning reflectance index
pub fn bri(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b05: f64 = one_px_data[1] as f64 / 255.0;
    let b09: f64 = one_px_data[2] as f64 / 255.0;
    let result: f64 = ((1.0 / b03) - (1.0 / b05)) / b09;
    (result * 255.0) as u8
}

// chloropyll green
pub fn chlgreen(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b07: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b07 / b03).powf(-1.0);
    (result * 255.0) as u8
}

// chloropyll index green
pub fn cigreen(one_px_data: Vec<u8>) -> u8 {
    let b03: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b09 / b03) - 1.0;
    (result * 255.0) as u8
}

// chloropyll indexrededge
pub fn cirededge(one_px_data: Vec<u8>) -> u8 {
    let b05: f64 = one_px_data[0] as f64 / 255.0;
    let b09: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b09 / b05) - 1.0;
    (result * 255.0) as u8
}

// chloropyll red-edge
pub fn chlrededge(one_px_data: Vec<u8>) -> u8 {
    let b05: f64 = one_px_data[0] as f64 / 255.0;
    let b07: f64 = one_px_data[1] as f64 / 255.0;
    let result: f64 = (b07 / b05).powf(-1.0);
    (result * 255.0) as u8
}
