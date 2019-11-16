// 32-bit color
#[derive(Copy, Clone)]
pub struct Color32 { r: u8, g: u8, b: u8, a: u8 }

// 24-bit color
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
        u32::from_be_bytes([self.r, self.g, self.b, self.a])
    }

    // Unpack a 32-bit pixel value into a Color32 struct
    pub fn from_pixel_value(value: u32) -> Color32 {
        let bytes = value.to_be_bytes();

        Color32 { r: bytes[0], g: bytes[1] , b: bytes[2], a: bytes[3] }
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
