use std::error;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;

use crate::color;
use crate::bytereader;

/// Structure of a simple TGA file according to the specification:  
/// http://www.dca.fee.unicamp.br/~martino/disciplinas/ea978/tgaffs.pdf
pub struct TGAFile {
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

impl TGAFile {
    pub fn new (width: u16, height: u16) -> TGAFile {
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
        TGAFile { 
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
    pub fn from_bytes(image_data: Vec<u8>) -> Result<TGAFile, TGAFileParsingError> {
        TGAFileParser::parse(&image_data)
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

struct TGAFileParser<'a> {
    inner: bytereader::ByteReader<'a>
}

impl<'a> TGAFileParser<'a> {
    /// Parse the TGA file
    /// Pixel data is returned as sequence of 32-bit numbers 
    /// If the bit-depth is 24, the alpha channel is set to 255
    /// Bit depths other than 32 and 24 are not supported
    pub fn parse(bytes: &'a[u8]) -> Result<TGAFile, TGAFileParsingError> {
        let mut parser = TGAFileParser {
            inner: bytereader::ByteReader::new(bytes)
        };

        parser.accept_byte(0,"Id lengths other than 0 are not supported")?;

        parser.accept_byte(0,"TGA Images with color maps are not supported")?;

        let image_type = parser.read_u8("Could not read the image type")?;

        parser.accept_bytes(&[0; 5], "Color map is not supported. Please set all color map scecification bytes to zero")?;

        let x_origin = parser.read_u16("Could not read x-origin")?;

        let y_origin = parser.read_u16("Could not read y-origin")?;

        let image_width = parser.read_u16("Could not read the image width")?;

        let image_height = parser.read_u16("Could not read the image height")?;

        let pixel_depth = parser.read_u8("Could not read the pixel depth")?;

        let image_descriptor = parser.read_u8("Could not read the image descriptor")?;

        // I convert everything to usize for two reasons
        // (a) multiplication might cause overflow if we don't have enough bits
        // (b) Though you can verify that this calculation won't overflow a u64
        // using the `usize` type seems more correct. `byte_count` cannot be greater than 
        // the maximum memory we can address (which is what usize's MAX is)
        let pixel_count = image_width as usize * image_height as usize;

        let pixel_data = parser.read_pixel_data(pixel_depth, image_type, pixel_count)?;

        Ok(TGAFile {
            id_length: 0,
            color_map_type: 0,
            image_type: image_type,
            color_map_spec: [0; 5],
            x_origin: x_origin,
            y_origin: y_origin,
            image_width: image_width,
            image_height: image_height,
            image_bits_per_pixel: pixel_depth,
            image_data: pixel_data,
            image_descriptor: image_descriptor,
            extension_area_offset: [0; 4],
            developer_dictionary_offset: [0; 4],
            signature: [0; 18] 
        })
    }

    fn accept_byte(&mut self, expected_byte: u8, error_msg: &str) -> Result<(), TGAFileParsingError> {
        self.accept_bytes(&[expected_byte], error_msg)
    }

    fn accept_bytes(&mut self, expected_bytes: &[u8], error_msg: &str) -> Result<(), TGAFileParsingError> {
        self.inner.accept(expected_bytes).map_err(TGAFileParsingError::with(error_msg))
    }

    fn read_u8(&mut self, error_msg: &str) -> Result<u8, TGAFileParsingError> {
        Ok(self.inner.read(1)
                    .map_err(TGAFileParsingError::with(error_msg))?
                    .iter()
                    .cloned()
                    .next()
                    .unwrap())
    }

    fn read_u16(&mut self, error_msg: &str) -> Result<u16, TGAFileParsingError> {
        Ok(u16::from_le_bytes([self.read_u8(error_msg)?, self.read_u8(error_msg)?]))
    }

    fn read_bytes(&mut self, count: usize, error_msg: &str) -> Result<&'a[u8], TGAFileParsingError> {
        self.inner.read(count).map_err(TGAFileParsingError::with(error_msg))
    }

    fn read_pixel_data(
        &mut self, 
        pixel_depth: u8,
        image_type: u8,
        pixel_count: usize) -> Result<Vec<u32>, TGAFileParsingError> 
    {
        if image_type == 2 {
            self.read_uncompressed_data(pixel_depth, pixel_count)
        }
        else if image_type == 10 {
            self.read_compressed_data(pixel_depth, pixel_count)
        }
        else {
            Err(TGAFileParsingError::new("Only compressed and uncompressed True-color images are suppored"))
        }
    }

    fn read_uncompressed_data(
        &mut self, 
        pixel_depth: u8,
        pixel_count: usize) -> Result<Vec<u32>, TGAFileParsingError> 
    {
        if pixel_depth != 24 && pixel_depth != 32 {
            Err(TGAFileParsingError::new("Only 24-bit and 32-bit pixels are supported"))
        }
        else {
            Ok(self.read_bytes(pixel_count, "Could not read uncompressed pixel data")?
            .chunks(pixel_depth as usize/ 8)
            .map(|x| {
                let x = x.iter().cloned().rev().collect::<Vec<u8>>();

                if pixel_depth == 24 {
                    u32::from_be_bytes([255, x[0], x[1], x[2]])
                }
                else {
                    u32::from_be_bytes([x[1], x[2], x[3], x[0]])
                }
            })
            .collect::<Vec<u32>>())
        }
    }

    fn read_compressed_data(
        &mut self,
        pixel_depth: u8,
        mut pixel_count: usize) -> Result<Vec<u32>, TGAFileParsingError> 
    {
        if pixel_depth != 24 && pixel_depth != 32 {
            Err(TGAFileParsingError::new("Only 24-bit and 32-bit pixels are supported"))
        }
        else {
            let mut pixel_data = Vec::new();

            while pixel_count > 0 {
                let rle_repetition_byte = self.read_u8("Could not read the repetition byte")?;
                let run_count = ((rle_repetition_byte & 0b0111_1111) + 1) as usize;
                let bytes_per_pixel = pixel_depth as usize/ 8;
                let is_rle_packet = (rle_repetition_byte & 0b1000_0000) >> 7 == 1;

                if !is_rle_packet {
                    pixel_data.append(&mut self.read_bytes(run_count * bytes_per_pixel, "Could not read rle raw pixel data")?
                            .chunks(bytes_per_pixel)
                            .map(|x| {
                                let x = x.iter().cloned().rev().collect::<Vec<u8>>();

                                if pixel_depth == 24 {
                                    u32::from_be_bytes([255, x[0], x[1], x[2]])
                                }
                                else {
                                    u32::from_be_bytes([x[1], x[2], x[3], x[0]])
                                }
                            })
                            .collect::<Vec<u32>>());
                }
                else {
                    let msg = "Could not read rle encoded pixel";
                    let b = self.read_u8(msg)?;
                    let g = self.read_u8(msg)?;
                    let r = self.read_u8(msg)?;
                    let a;
                    if pixel_depth == 24 {
                        a = 255;
                    }
                    else {
                        a = self.read_u8(msg)?;
                    }

                    let pixel = u32::from_be_bytes([r, g, b, a]);

                    pixel_data.append(&mut vec![pixel; run_count]);
                }

                pixel_count -= run_count;
            }

            Ok(pixel_data)
        }
    }
}

#[derive(Debug)]
pub struct TGAFileParsingError {
    inner: Option<bytereader::ByteReaderError>,
    msg: String,
}

impl TGAFileParsingError {
    pub fn new(msg: &str) -> Self {
        TGAFileParsingError {
            msg: msg.to_string(),
            inner: None,
        }
    }

    pub fn with(msg: &str) -> impl FnOnce(bytereader::ByteReaderError) -> Self + '_ {
        move | err | {
            TGAFileParsingError {
                msg: msg.to_string(),
                inner: Some(err),
            }
        }
    }
}

impl error::Error for TGAFileParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.inner.as_ref().unwrap())
    }
}

impl fmt::Display for TGAFileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileParsingError: {}.", self.msg)
    }
}