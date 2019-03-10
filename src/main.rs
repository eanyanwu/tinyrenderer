use tinyrenderer::tga;
use tinyrenderer::model;

fn main() {
    let model = model::WaveFront::new("african_head.obj").unwrap();

    let white = tga::TGAColor::new(255, 255, 255, 255);
    
    let width = 800;
    let height = 800;

    let mut image = tga::TGAImage::new(width, height, 4);
    


    for i in 0..model.face_count() {
        let face = model.get_face(i);
        for j in 0..3 {
            let v0 = model.get_vertex(face.vertices[j]);
            let v1 = model.get_vertex(face.vertices[(j + 1) % 3]);
            
            let x0 = (v0.x + 1.0) * width as f64/2.0;
            let y0 = (v0.y + 1.0) * height as f64/2.0;
            let x1 = (v1.x + 1.0) * width as f64/2.0;
            let y1 = (v1.y + 1.0) * height as f64/2.0;

            line_second_try(x0 as u16, y0 as u16, x1 as u16, y1 as u16, &mut image, &white);
        }
    }

    image.write_tga_file("model.tga");
}

fn line_second_try(x0: u16, y0: u16, x1: u16, y1: u16, img: &mut tga::TGAImage, color: &tga::TGAColor) {
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
            img.set(x as u16, y as u16, color);
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
        img.set(x1, y1, color);
    }
    else {
        while if y0 < y1 { (y as u16) < y1 } else { y as u16 > y1 } {
            img.set(x as u16, y as u16, color);
            
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
        img.set(x1, y1, color);
    }
}

