// 32-bit color
#[derive(Copy, Clone)]
pub struct Color32 { r: u8, g: u8, b: u8, a: u8 }

// 24-but color
#[derive(Copy, Clone)]
pub struct Color24 { r: u8, g: u8, b: u8 }

impl Color32 {
    pub fn new (r: u8, g: u8, b: u8, a: u8) -> Color32
    {
        Color32 { r, g, b, a }
    }

    // Pack the pixel values into a u32 value and return it.
    // We are packing them in the form RGBA
    pub fn get_pixel_value(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | self.a as u32
    }

    // Unpack a 32-bit pixel value into a Color32 struct
    pub fn from_pixel_value(value: u32) -> Color32 {
        let r = ((value & 0b11111111_00000000_00000000_00000000) >> 24) as u8;
        let g = ((value & 0b00000000_11111111_00000000_00000000) >> 16) as u8;
        let b = ((value & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
        let a = (value & 0b00000000_00000000_00000000_11111111) as u8;

        Color32 { r, g, b, a }
    }

    // Convert a Color24 struct into Color32 by adding an alpha channel.
    pub fn from_color24(color: &Color24) -> Color32 {
        Color32 {
            r: color.r,
            g: color.g,
            b: color.b,
            a: 255
        }
    }
}
