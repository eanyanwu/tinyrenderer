use crate::file_formats::wavefront;
use crate::file_formats::tga;
use crate::point;

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
    v0: point::Point3D,
    v1: point::Point3D,
    image: &mut crate::file_formats::tga::TGAImage,
    color: &Color32)
{
    line(v0.x as u16, v0.y as u16, v1.x as u16, v1.y as u16, image, color);
}

pub fn triangle(v0: point::Point3D, v1: point::Point3D, v2: point::Point3D, image: &mut tga::TGAImage, color: &Color32) {
    
    // First outline the triangle
    line_from_vertices(v0, v1, image, &color);
    line_from_vertices(v1, v2, image, &color);
    line_from_vertices(v2, v0, image, &color);

    // Then figure out how to color it in :p
    // Step 1: Figure out the "bounding box" of the triangle.
    // I am lazy and don't feel like doing this in a scholarly 
    // way, so here we are.
    let mut min_y = 1000000.0;
    let mut max_y = 0.0;
    let mut min_x = 1000000.0;
    let mut max_x = 0.0;
    
    // Yes, I know this is horrid to look at
    // but bear with me.
    if v0.y < min_y { min_y = v0.y; }
    if v1.y < min_y { min_y = v1.y; }
    if v2.y < min_y { min_y = v2.y; }
    
    if v0.x < min_x { min_x = v0.x; }
    if v1.x < min_x { min_x = v1.x; }
    if v2.x < min_x { min_x = v2.x; }

    if v0.y > max_y { max_y = v0.y; }
    if v1.y > max_y { max_y = v1.y; }
    if v2.y > max_y { max_y = v2.y; }
    
    // Almost there...
    if v0.x > max_x { max_x = v0.x; }
    if v1.x > max_x { max_x = v1.x; }
    if v2.x > max_x { max_x = v2.x; }

    // Get rid of any degenerate triangles...
    if (min_x - max_x).abs() < 1.0 || (min_y - max_y).abs() < 1.0 {
        return;
    }
    

    // Ok sweet, I have found the bounding box.
    // Step 2: Determine if a given point P is inside the triangle.
    // This is my take in barycentric coordinates since I don't 
    // don't get barycentric coordinates 100%
    //
    // Given a triangle ABC, any point P, can be reached via a vector
    // that is a linear combination of AB and AC.
    // Meaning for any point P, we will have AP = uAB + vAC.
    // where u and v are some constants.
    // This is nice and all, but how does it help me?
    // Well turns out that if the constants u and v are positive and 
    // their sum is less than 1, then the point P will be in the triangle
    // "enclosed" by vectors AB and AC. Draw it out to see why this is at least
    // intuitive. I don't claim to have proved this, it just sounds right
    //
    // In sum, to figure out if a point P is in the triangle ABC, I need to find
    // the constants u and v. If they are positive and sum up to less than 1, the point
    // is inside the triangle.
    // 
    // To find the constants u and v, I think in the following manner.
    // I consider the vectors AB and AC to be transformed versions of the basis vectors
    // i and j (the 1-unit vectors in the x and y direction respectively).
    // The matrix that defines this transformation is:
    // |AB_x AB_x| "Let's call this matrix A"
    // |AB_y AC_y|
    // 
    // If that doesn't make sense, take a look at the first 3~4 episodes of 3blue1brown's
    // "Essense of Linear Algebra" playlist. It shed a whole new light on linear algebra
    // for me.
    //
    // So i could ask myself the question:
    // What is the vector t that when that transformation is applied, gives me the vector AP?
    // (Deciding what question to ask is often the hardest thing. It took me one week of 
    // looking at triangles and drawing vectors to come up with this. This is partly due to the
    // fact that I'm practically a newbie at linear algebra)
    // 
    // The equation that asks that question is: Matrix A * t = Vector AP.
    // To find vector t, we can find the inverse of the Matrix A, Matrix A^-1 and "multiply"
    // it by Vector AP, which is exactly what I am doing below.
    // Now, since the linear transformation that Matrix A defines takes the i and j vectors and 
    // transforms them into AB and AC, that same linear transformation takes the t vector and 
    // transforms it to AP. Well, we just found t. What comes next is another totally unproven
    // assumption that i made. It just seemed right.
    // If t_x and t_y are greater than 0 and their sum is less than 1, then the vector t is within
    // the triangle formed by the i and j vectors.
    // Additionally, If the point at the tip of vector t is within the triangle formed by the i and j vectors,
    // THEN the transformed version of t, AP, must be also within the triangle formed by the
    // transformed versions of the i and j vectors, AB and AC.
    // 
    // BOOM. DONE. EZE OUT.
    let ab = v1 - v0;
    let ac = v2 - v0;

    let inverse_determinant = 1.0 / (ab.x * ac.y - ac.x * ab.y);

    let matrix_inverse = [(ac.y * inverse_determinant, ab.y * -1.0 * inverse_determinant),
                            (ac.x * -1.0 * inverse_determinant, ab.x * inverse_determinant)];

    let mut p_x = min_x;
     
    while p_x < max_x {
        let mut p_y = min_y;

        while p_y < max_y {
            let (ap_x, ap_y) = (p_x - v0.x, p_y - v0.y);

            let u = ap_x * matrix_inverse[0].0 + ap_y * matrix_inverse[1].0;

            let v = ap_x * matrix_inverse[0].1 + ap_y * matrix_inverse[1].1;

            if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
                image.set(p_x as u16, p_y as u16, &color).unwrap();
            }

            p_y += 1.0;
        }

        p_x += 1.0;
    }
}
