use tinyrenderer::tga;

fn main() {
    let red = tga::TGAColor::new(255, 0, 0, 255);

    let white = tga::TGAColor::new(255, 255, 255, 255);
    
    let green = tga::TGAColor::new(0, 255, 0, 255);

    let mut image = tga::TGAImage::new(100, 100, 4);

    line_second_try(0, 49, 99, 49, &mut image, &white);

    line_second_try(0, 99, 99, 0, &mut image, &white);

    line_second_try(49, 99, 49, 0, &mut image, &white);

    line_second_try(99, 99, 0, 0, &mut image, &white);
    
    line_second_try(0, 24, 99, 74, &mut image, &red);
    
    line_second_try(99, 24, 0, 74, &mut image, &red);

    line_second_try(74, 99, 24, 0, &mut image, &green);

    line_second_try(24, 99, 75, 0, &mut image, &green);

    image.write_tga_file("line.tga");
}

fn line_second_try(x0: isize, y0: isize, x1: isize, y1: isize, img: &mut tga::TGAImage, color: &tga::TGAColor) {
    // Start by getting the first pixel to plot
    let mut x = x0;
    let mut y = y0; 

    let dy = y1 as f64 - y0 as f64;
    let dx = x1 as f64 - x0 as f64;

    let m = if dx == 0.0 { 0.0 } else { dy/dx };
    let inverse_m = if m == 0.0 { 0.0 } else { 1.0/m };

    let mut error = 0.0;
    
    let x_step = if x0 < x1 { 1 } else { -1 };
    let y_step = if y0 < y1 { 1 } else { -1 };

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
        while if x0 < x1 { x <= x1 } else { x >= x1 } {
            // Set the pixel.
            img.set(x, y, color);
            // Set x to the next pixel.
            x += x_step;
            // Increment the error by the slope
            error += m.abs();

            if error >= 1.0 {
                // Error is greater than 1, increment y
                y += y_step;
                // Decrement error
                error -= 1.0;
            }
        }
    }
    else {
        while if y0 < y1 { y <= y1 } else { y >= y1 } {
            img.set(x, y, color);

            y += y_step;
            // Increment the error by the inverse of the slope
            error += inverse_m.abs();

            if error >= 1.0 {
                // Error is greater than 1, increment x
                x += x_step;
                // Decrement error
                error -= 1.0;
            }
        }
    }
}

