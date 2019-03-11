// 32 Bit Color
pub struct Color32 { r: u8, g: u8, b: u8, a: u8 }

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
}

pub struct Line;

impl Line {
    pub fn new(x0: u16, y0: u16, x1: u16, y1: u16, img: &mut crate::file_formats::tga::TGAImage, color: &Color32) {
        // Start by getting the first pixel to plot
        let mut x = x0;
        let mut y = y0; 

        let dy = y1 as f64 - y0 as f64;
        let dx = x1 as f64 - x0 as f64;

        let m = if dx == 0.0 { 0.0 } else { dy/dx };
        let inverse_m = if m == 0.0 { 0.0 } else { 1.0/m };

        let mut error = 0.0;
        
        // x-axis is "driving" so to speak -- meaning its values on the line between the two points 
        // is increasing faster than the y values (or at an equal rate)
        // If the x values are increasing at a greater rate than the y-values, then:
        // As x increases, by 1 pixel, the real value of y increases by the slope of the line, m. 
        // However, we can only set pixels at integer intervals. So we won't always be able to set the
        // next y value to the previous y value + 1. 
        // Instead, we keep track of an "error" value that corresponds to how much we haven't
        // incremented y.
        // If we increment x, but don't increment y, we add the value of the slope to that error.
        // Once the value of the slope is more than 1, we increment y and reduce the value of the slope
        // by 1.
        // This will also work if the y values are increasing at a greater rate than the x-values.
        // Except we will be adding 1/slope at each iteration.
        if dx.abs() >= dy.abs() {
            while if x0 < x1 { x < x1 } else { x > x1 } {
                // Set the pixel.
                img.set(x as u16, y as u16, &color);
                // Set x to the next pixel.
                if x0 < x1 { x += 1; } else { x -= 1; }
                // Increment the error by the slope
                error += m.abs();

                if error >= 1.0 {
                    // Error is greater than 1, increment/decrement y based on the direction of the
                    // line.
                    if y0 < y1 { y+= 1; } else { y -= 1; }
                    // Decrement error
                    error -= 1.0;
                }
            }
            
            // See explanation below.
            img.set(x1, y1, &color);
        }
        else {
            while if y0 < y1 { (y as u16) < y1 } else { y as u16 > y1 } {
                img.set(x as u16, y as u16, &color);
                
                if y0 < y1 { y += 1; } else { y -= 1; }

                // Increment the error by the inverse of the slope
                error += inverse_m.abs();

                if error >= 1.0 {
                    // Error is greater than 1, increment x
                    if x0 < x1 { x += 1; } else { x -= 1; } 
                    // Decrement error
                    error -= 1.0;
                }
            }
            // Since the loop stops right before the endpoints, we draw the endpoints manually.
            // Reason: Consider the situation where y0 is 0. If we looped until y <= y0. y would be 0,
            // yet we would stilll try to loop and eventually hit an error when we try to decrement
            // from an unsinged integer.
            img.set(x1, y1, &color);
        }
    }
}
