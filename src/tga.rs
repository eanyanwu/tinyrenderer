use std::io::prelude::*;
use std::fs::File;

use crate::color;

// Tga image specification: http://www.dca.fee.unicamp.br/~martino/disciplinas/ea978/tgaffs.pdf

pub struct TGAImage {
    // HEADER
    id_length: u8, // Field 1: Optional. Identifies the number of bytes contained in field 6. Value of 0 means no field 6
    color_map_type: u8, // Field 2: Value of 0: "No color map included". Value of 1: "Color map included"
    image_type: u8,// // Field 3: Type of the image e.g ( compressed/uncompressed, true-color/color-mapped etc.. )
    color_map_spec: [u8; 5], // Field 4: Color map spec.
    x_origin: u16, // Field 5 x-coordinate of the lower-left corner of our image
    y_origin: u16, // Field 5 y-coordinate os the lower-left corner of our images
    image_width: u16, // Field 5: Width
    image_height: u16, // Field 5: Height
    image_bits_per_pixel: u8, // Field 5: Also referred to as pixel depth
    image_descriptor: u8, // Field 5: Presence of an alpha channel + Screen destination of first pixel
    
    // DATA
    image_data: Vec<u32>, // Field 8: Image data!

    // FOOTER: 
    developer_dictionary_offset: [u8; 4], 
    extension_area_offset: [u8; 4], 
    signature: [u8; 18]
}

impl TGAImage {
    pub fn new (width: u16, height: u16) -> TGAImage {
        let id_length = 0; 
        let color_map_type = 0; 
        let image_type = 2; 
        let color_map_spec = [0, 0, 0, 0, 0]; 
        let x_origin = 0;
        let y_origin = 0;
        let image_width = width;
        let image_height = height;
        let image_bits_per_pixel = 32;
        let image_descriptor = 0b0000_1000; // Bits 0-3: Alpha channel, Bits 5-6: order of moving pixels to screen
        // The number of bits in `usize` is the number of bits that it takes to reference any
        // location in memory. Since vectors are locations in memory, it's size could technically
        // be the whole of memory, so I guess it makes sense that you would need a `usize` to
        // define its size.
        let image_data = vec![0x000000FF; image_width as usize * image_height as usize];
        let extension_area_offset = [0, 0, 0, 0];
        let developer_dictionary_offset = [0, 0, 0, 0];
        let signature: [u8; 18] = [b'T', b'R', b'U', b'E', b'V', b'I', b'S', b'I', b'O', b'N', b'-', b'X', b'F', b'I', b'L', b'E', b'.', b'\0'];
        
        // Create the struct and return it.
        TGAImage { 
            id_length,
            color_map_type,
            image_type,
            color_map_spec,
            x_origin,
            y_origin,
            image_width,
            image_height,
            image_bits_per_pixel,
            image_descriptor,
            image_data,
            extension_area_offset,
            developer_dictionary_offset,
            signature
        }
    }
   
    // Create a TGAImage object from byte array 
    pub fn from_bytes(image_data: Vec<u8>) -> TGAImage {
        read_byte_array(image_data).unwrap()
    }

    pub fn get_width(&self) -> u16 {
        self.image_width
    }

    pub fn get_height(&self) -> u16 {
        self.image_height
    }

    // Get the color of a TGAImage at a point
    pub fn get(&self, x: u16, y: u16) -> color::Color32 {
        // Find the index
        let index = y as usize * self.image_width as usize + x as usize;

        let pixel_value = self.image_data[index];

        color::Color32::from_pixel_value(pixel_value)
    }
    
    // Set the color of a TGAImage at a point
    // Note: x and y are u16 because they can't be bigger than the width and height
    pub fn set (&mut self, x: u16, y: u16, color: &color::Color32) -> Result<(), String> {
        if  x >= self.image_width {
            return Err(format!("Invalid x value {}. It is greater than the width of the image.", x))
        }
        
        if  y >= self.image_height { 
            return Err(format!("Invalid y value {}. It is greater than the height of the image.", y))
        }
    
        // 2d coordinates to 1D  index.
        // Again this convertion is needed because we can only index with usize. 
        // Consider the case where the multiplication of these three values yields a valid index
        // number, but one too large to fit in u16.
        let index = y as usize * self.image_width as usize + x as usize;
    
        self.image_data[index] = color.get_pixel_value();

        Ok(())
    }

    // The specification says that TGA files are stored in little-endian format (Intel byte
    // ordering)
    // This was a bit confusing to me at first, but I think I figured it out. 
    // If the value you are writing is one-byte long, nothing changes. You write it out to the file
    // as you would normally.
    // If the value you are writing contains multiple bytes (e.g. the width, height, x_origin,
    // y_origin, and pixel values), the bytes need to inverted.
    // Not doing this (especially for the width and the height) makes your file unreadable because
    // the size is interpreted in the wrong way.
    pub fn write_tga_file(&self, filename: &str) {
        let mut data = Vec::new();
        
        // [0] Notice that when writing values that are more than 1-byte long, we are writing the
        // bytes in reverse order as per the TGA specification
        // [1] Use the AND bitwise operator to select the right-most byte in this 2-byte value and
        // shove it into a 1-byte value
        // [2] Use the and bitwise operator to select the left-most byte in this 2-byte value,
        // shift it to the right and shove it into a 1-byte value.
        // The shift is so that the u16 -> u8 conversion works properly
        // If after the bitwise operation we got: 1111_0001 0000_0000, the value we want 
        // is the 1111_0001, so we shift it all the way to the left so that the cast gets the
        // correct part of the value

        data.push(self.id_length);
        data.push(self.color_map_type);
        data.push(self.image_type);
        data.extend_from_slice(&self.color_map_spec);
        data.extend_from_slice(&[ // See [0]
            (self.x_origin & 0x00FF) as u8, // See [1]
            ((self.x_origin & 0xFF00) >> 8) as u8, // See [2]
            (self.y_origin & 0x00FF) as u8,
            ((self.y_origin & 0xFF00) >> 8) as u8]);
        data.extend_from_slice(&[
            (self.image_width & 0x00FF) as u8,
            ((self.image_width & 0xFF00) >> 8) as u8,
            (self.image_height & 0x00FF) as u8,
            ((self.image_height & 0xFF00) >> 8) as u8]);
        data.push(self.image_bits_per_pixel);
        data.push(self.image_descriptor);
        
        let mut pixel_data: Vec<u8> = self.image_data.iter()
            .flat_map(|rgba| {vec![
                    ((rgba & 0x0000FF00) >> 8) as u8,
                    ((rgba & 0x00FF0000) >> 16) as u8,
                    ((rgba & 0xFF000000) >> 24) as u8,
                    (rgba & 0x000000FF) as u8]})
            .collect();

        data.append(&mut pixel_data);
        data.extend_from_slice(&self.extension_area_offset);
        data.extend_from_slice(&self.developer_dictionary_offset);
        data.extend_from_slice(&self.signature);

        
        let mut file = match File::create(filename) {
            Err(x) => panic!("Could not create file! {}", x),
            Ok(f) => f
        };

        if let Err(e) = file.write_all(&data) {
            panic!("Could not write data to file! {}", e);
        }
    }
}

fn read_byte_array(byte_array: Vec<u8>) -> Result<TGAImage, String> {
    let id_length = byte_array[0];

    if id_length != 0 {
        return Err(String::from("Id lengths other than 0 are not supported"));
    }

    let color_map_type = byte_array[1];

    if color_map_type != 0 {
        return Err(format!("TGA Images with color maps are not supported. Color Map Type: {}", color_map_type));
    }

    let image_type = byte_array[2];
    
    if !(image_type == 2 || image_type == 10) {
        return Err(format!("Only Compressed and Uncompressed TrueColor images are supposrted. Image Type: {}", image_type));
    }
    
    let color_map_spec = &byte_array[3..8];
    // Verify that they are all zero. 
    if let false = color_map_spec.iter().all(|&x| x == 0) {
        return Err(String::from("Colr map is not supported. Please set all color map scecification bytes to zero"));
    }

    // Don't forget TGA files are saved in little endian format, so
    // the bytes are reversed in multi-byte values
    let x_origin = (byte_array[9] as u16) << 8 | byte_array[8] as u16;

    let y_origin = (byte_array[11] as u16) << 8 | byte_array[10] as u16;

    let image_width = (byte_array[13] as u16) << 8 | byte_array[12] as u16;

    let image_height: u16 = (byte_array[15] as u16) << 8 | byte_array[14] as u16;

    let pixel_depth = byte_array[16];

    let image_descriptor = byte_array[17];

    let (header, rest) = byte_array.split_at(18);

    println!("Header len: {}. Rest len: {}", header.len(), rest.len());

    let mut pixel_data = Vec::new();

    // The casting is because image_width and image_height are a u16
    // but multiplying them could give a number as big as u32
    let pixel_count = image_width as u32 * image_height as u32;

    println!("Pixel Count: {}", pixel_count);

    // Run-length encoded pixel values
    if image_type == 10 {
        pixel_data = extract_rle_pixels(
            rest,
            pixel_count,
            pixel_depth);

    } else { // Uncompressed pixel values
        pixel_data = extract_uncompressed_pixels(
            rest,
            pixel_count,
            pixel_depth);
    }

    Ok(TGAImage {
        id_length,
        color_map_type,
        image_type,
        color_map_spec: [0, 0, 0, 0, 0],
        x_origin,
        y_origin,
        image_width,
        image_height,
        image_bits_per_pixel: pixel_depth,
        image_descriptor,
        image_data: pixel_data,
        extension_area_offset: [0; 4],
        developer_dictionary_offset: [0; 4],
        signature: [0; 18] 
    })
}

fn extract_uncompressed_pixels(
    byte_array: &[u8],
    pixel_count: u32,
    pixel_depth: u8) -> Vec<u32> {

    let mut num_processed_pixels: u32 = 0;

    let mut byte_array_iter = byte_array.iter();

    let mut extracted_pixels: Vec<u32> = Vec::new();

    while num_processed_pixels < pixel_count {
        let b = *(byte_array_iter.next().unwrap());
        let g = *(byte_array_iter.next().unwrap());
        let r = *(byte_array_iter.next().unwrap());

        // My implementation always saves pixels in 32 bits.
        // If the file we are reading is missing the alpha channel
        // default to opaque i.e a value of 255
        let a = if pixel_depth == 24 {
            255
        } else {
            *(byte_array_iter.next().unwrap())
        };

        extracted_pixels.push(color::Color32::new(r, g, b, a).get_pixel_value());

        num_processed_pixels += 1;
    }

    extracted_pixels
}
// Expand Run-Length Encoded pixels
fn extract_rle_pixels(
    byte_array: &[u8],
    pixel_count: u32,
    pixel_depth: u8) -> Vec<u32> {
    
    let mut num_processed_pixels: u32 = 0;

    let mut byte_array_iter = byte_array.iter();

    let mut extracted_pixels: Vec<u32> = Vec::new();

    while num_processed_pixels < pixel_count {

        let rle_repetition_count = byte_array_iter.next().unwrap();

        let run_count = (rle_repetition_count & 0b0111_1111) + 1;

        let is_rle_packet = (rle_repetition_count & 0b1000_0000) >> 7 == 1;

        // Raw Packet 
        if !is_rle_packet {
            for _i in 0..run_count {
                // Recall that bytes are stored in 
                // little endian format. So the bytes for 
                // RGB are actually stored in reverse
                let b = *(byte_array_iter.next().unwrap());
                let g = *(byte_array_iter.next().unwrap());
                let r = *(byte_array_iter.next().unwrap());
                let a = if pixel_depth == 24 {
                    255 
                } else {
                    *(byte_array_iter.next().unwrap())
                };

                let pixel_value = color::Color32::new(r, g, b, a)
                    .get_pixel_value();

                extracted_pixels.push(pixel_value);

                num_processed_pixels += 1;
            }
        } else { // Run-Length encoded packet
            let b = *(byte_array_iter.next().unwrap());
            let g = *(byte_array_iter.next().unwrap());
            let r = *(byte_array_iter.next().unwrap());
            let a = if pixel_depth == 24 {
                255 
            } else {
                *(byte_array_iter.next().unwrap())
            };

            let pixel_value = color::Color32::new(r, g, b, a)
                .get_pixel_value();

            for _i in 0..run_count {
                extracted_pixels.push(pixel_value);

                num_processed_pixels += 1;
            }
        }
    }

    extracted_pixels
}
