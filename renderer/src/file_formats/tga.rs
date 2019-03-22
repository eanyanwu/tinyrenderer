use std::fs::File;
use std::io::prelude::*;
use crate::drawing;

pub struct TGAImage {
    // HEADER
    id_length: u8, // Field 1: Optional. Identifies the number of bytes contained in field 6. Value of 0 means no field 6
    color_map_type: u8, // Field 2: Value of 0: "No color map included". Value of 1: "Color map included"
    image_type: u8,// // Field 3: Type of the image e.g ( compressed/uncompressed, true-color/color-mapped etc.. )
    color_map_spec: [u8; 5], // Field 4: Color map spec.
    x_origin: u16, // Field 5: Not entirely sure what this refers to yet.
    y_origin: u16, // Field 5: Same as above. These two values are part of the same field in the specification.
    image_width: u16, // Field 5: Width
    image_height: u16, // Field 5: Height
    image_bits_per_pixel: u8, // Field 5: Also referred to as pixel depth
    image_descriptor: u8, // Field 5: Presence of an alpha channel + Screen destination of first pixel
    
    // DATA 
    image_data: Vec<u32>, // Field 8: Image data!

    // FOOTER: According to the specification, the presence of these last 26 bits helps determine
    // that this is a TGA file of Version 2.
    developer_dictionary_offset: [u8; 4], 
    extension_area_offset: [u8; 4], 
    signature: [u8; 18]
}

impl TGAImage {
    pub fn new (width: u16, height: u16, bytes_per_pixel: u8) -> TGAImage
    {
        let id_length = 0; 
        let color_map_type = 0; 
        let image_type = 2; 
        let color_map_spec = [0, 0, 0, 0, 0]; 
        let x_origin = 0;
        let y_origin = 0;
        let image_width = width;
        let image_height = height;
        let image_bits_per_pixel = bytes_per_pixel * 8;
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

    pub fn from_file(image_data: &Vec<u8>) {
        let id_length = image_data[0];

        println!("Id Length: {}", id_length);

        if id_length != 0 {
            panic!("Id lengths other than 0 are not supported!");
        }

        let color_map_type = image_data[1];

        println!("Color Map Type: {}", color_map_type);
        
        if color_map_type != 0 {
            panic!("TGA Images with color maps are not supported! Color map: {}", color_map_type);
        }

        let image_type = image_data[2];
        
        println!("Image Type: {}", image_type);

        if !(image_type == 2 || image_type == 10) {
            panic!("Image Type: {}. Only Uncompressed and Compressed Truecolor images are supported", image_type);
        }
        
        // Take the 5 bytes that represent the color map specification.
        let color_map_spec = &image_data[3..8];
        // Verify that they are all zero. 
        if let false = color_map_spec.iter().all(|&x| x == 0) {
            panic!("Color map is not supported. Please set all color map spec bytes to zero");
        }

        // Don't forget TGA files are saved in little endian format, so
        // the bytes are reversed in multi-byte values
        let x_origin = (image_data[9] as u16) << 8 | image_data[8] as u16;

        println!("X Origin: {}", x_origin);

        let y_origin = (image_data[11] as u16) << 8 | image_data[10] as u16;

        println!("Y Origin: {}", y_origin);

        let image_width = (image_data[13] as u16) << 8 | image_data[12] as u16;

        println!("Image Width: {}", image_width);

        let image_height: u16 = (image_data[15] as u16) << 8 | image_data[14] as u16;

        println!("Image Height: {}", image_height);

        let pixel_depth = image_data[16];

        println!("Pixel Depth: {}", pixel_depth);

        let image_descriptor = image_data[17];

        println!("Image Descriptor: {}", image_descriptor);

        // Extract image data
        let pixelCount = image_width as u32 * image_height as u32;
        
        let mut pixel_values: Vec<u32> = Vec::new();
        
        // Run length encoded image
        if image_type == 10 {
            let counter = 0;
            let index = 18;
            while counter < pixelCount {
               let rle_repetition_count = image_data[index];
                
               // Raw Packet
               if (rle_repetition_count & 0b1000_0000) == 0 {
                   println!("Raw packet");
               }
               // Run lenght Packet
               else {
                   println!("Run Length Packet");
                   // The run count recorded is one less than the actua number of repeated pixels.
                   // A run count of 0, actually means there is one pixel in the next field
                   let run_count = (rle_repetition_count & 0b0111_1111) + 1;

                   // TODO: Extract the pixel value field.
               }
            }
        }
        // Uncompressed image
        else {
        }
    }

    pub fn get_width(&self) -> u16 {
        self.image_width
    }

    pub fn get_height(&self) -> u16 {
        self.image_height
    }

    // x and y are u16 because they can't be bigger than the width and height
    pub fn set (&mut self, x: u16, y: u16, color: &drawing::Color32) -> Result<(), String> {
        //if x == 0 {
        //    return Err(format!("Invalid x value 0. Please use values 1-{}", self.image_width))
        //}

        //if y == 0 {
        //    return Err(format!("Invalid y value 0. Please use values 1-{}", self.image_height))
        //}

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
    
        // However it is safe because u16 < usize (for modern computers as far as i know)
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
