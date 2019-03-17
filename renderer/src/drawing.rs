use crate::file_formats::wavefront;
use crate::file_formats::tga;

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

pub fn line(
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16, 
    img: &mut crate::file_formats::tga::TGAImage,
    color: &Color32) 
{
    // Start by getting the first pixel to plot
    let mut x = x0;
    let mut y = y0; 

    let dy = y1 as f64 - y0 as f64;
    let dx = x1 as f64 - x0 as f64;

    let m = if dx == 0.0 { 0.0 } else { dy/dx };
    let inverse_m = if m == 0.0 { 0.0 } else { 1.0/m };

    let mut error = 0.0;
    
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
        while x != x1 {
            // Set the pixel.
            img.set(x as u16, y as u16, &color).unwrap();
            // Set x to the next pixel
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
        img.set(x1, y1, &color).unwrap();
    }
    else {
        while y != y1 {
            img.set(x as u16, y as u16, &color).unwrap();
            
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
        // Since the loop stops right before drawing the endpoints, we draw the endpoints manually.
        // Reason: Consider the situation where y0 is 0. If we looped until y <= y0. y would be 0,
        // yet we would stilll try to loop and eventually hit an error when we try to decrement
        // from an unsinged integer.
        img.set(x1, y1, &color).unwrap();
    }
}

pub fn line_from_vertices(
    v0: &wavefront::Vertex,
    v1: &wavefront::Vertex,
    image: &mut crate::file_formats::tga::TGAImage,
    color: &Color32)
{
    line(v0.x as u16, v0.y as u16, v1.x as u16, v1.y as u16, image, color);
}

pub fn triangle(v0: &wavefront::Vertex, v1: &wavefront::Vertex, v2: &wavefront::Vertex, image: &mut tga::TGAImage, color: &Color32) {
      
    let mut min_y = 1000000;
    let mut max_y = 0;
    let mut min_x = 1000000;
    let mut max_x = 0;

    if (v0.y as u64) < min_y { min_y = v0.y as u64; }
    if (v1.y as u64) < min_y { min_y = v1.y as u64; }
    if (v2.y as u64) < min_y { min_y = v2.y as u64; }
    
    if (v0.x as u64) < min_x { min_x = v0.x as u64; }
    if (v1.x as u64) < min_x { min_x = v1.x as u64; }
    if (v2.x as u64) < min_x { min_x = v2.x as u64; }

    if (v0.y as u64) > max_y { max_y = v0.y as u64; }
    if (v1.y as u64) > max_y { max_y = v1.y as u64; }
    if (v2.y as u64) > max_y { max_y = v2.y as u64; }
    
    if (v0.x as u64) > max_x { max_x = v0.x as u64; }
    if (v1.x as u64) > max_x { max_x = v1.x as u64; }
    if (v2.x as u64) > max_x { max_x = v2.x as u64; }
    
    if min_x  == max_x || min_y == max_y {
        return;
    }

    let ab = (v1.x as i64 - v0.x as i64, v1.y as i64 - v0.y as i64, v1.z as i64 - v0.z as i64);
    let ac = (v2.x as i64 - v0.x as i64, v2.y as i64 - v0.y as i64, v2.z as i64 - v0.z as i64);
    let (ab_x, ab_y, ab_z) = ab;
    let (ac_x, ac_y, ac_z) = ac;
    let determinant = ab_x * ac_y - ac_x * ab_y;
    let inverse_determinant = 1.0 / determinant as f64;

    let matrix_inverse = [(ac_y as f64 * inverse_determinant, ab_y as f64 * -1.0 * inverse_determinant),
                            (ac_x as f64 * -1.0 * inverse_determinant, ab_x as f64 * inverse_determinant)];

    for p_x in min_x..=max_x {
        for p_y in min_y..=max_y {
            let (ap_x, ap_y) = (p_x as i64 - v0.x as i64, p_y as i64 - v0.y as i64);

            let u = ap_x as f64 * matrix_inverse[0].0 + ap_y as f64 * matrix_inverse[1].0;
            let v = ap_x as f64 * matrix_inverse[0].1 + ap_y as f64 * matrix_inverse[1].1;

            if u > 0.0 && v > 0.0 && u + v < 1.0 {
                image.set(p_x as u16, p_y as u16, &color).unwrap();
            }
        }
    }
}
