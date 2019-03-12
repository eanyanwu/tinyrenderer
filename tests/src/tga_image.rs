use renderer::file_formats::tga;
use renderer::drawing;

#[test]
fn new_tgafile_height_and_width_are_correct() {
   let tga_file = tga::TGAImage::new(10, 10, 4);

   assert_eq!(tga_file.get_width(), 10);
   assert_eq!(tga_file.get_height(), 10);
}

#[test]
fn setting_y_value_greater_than_height_fails() -> Result<(), String> {
    let mut tga_file = tga::TGAImage::new(10, 10, 4);

    let color = drawing::Color32::new(0, 0, 0, 255);

    let result =  tga_file.set(1, 11, &color);

    if result.is_err() {
        let error = result.err().unwrap();

        assert!(error.contains("Invalid y value 11")
                format!("Assertion failed. Got: {}", error));

        Ok(())
    }
    else {
        Err(String::from("Expected to get an error when the y value is greater than the image height"))
    }
}

#[test]
fn setting_x_value_greater_than_width_fails() -> Result<(), String> {
    let mut tga_file = tga::TGAImage::new(10, 10, 4);

    let color = drawing::Color32::new(0, 0, 0, 255);

    let result = tga_file.set(11, 1, &color);

    if result.is_err() {
        let error = result.err().unwrap();

        assert!(error.contains("Invalid x value 11"), 
                format!("Assertion failed. Got: {}", error));

        Ok(())
    } else {
        Err(String::from("Expected to get an error when the x value is greater than the image width"))
    }
}

#[test]
fn setting_x_value_of_zero_fails() -> Result<(), String> {
    let mut tga_file = tga::TGAImage::new(10, 10, 4);

    let color = drawing::Color32::new(0, 0, 0, 255);

    let result = tga_file.set(0, 1, &color);

    if result.is_err() {
        let error = result.err().unwrap();

        assert!(error.contains("Invalid x value 0"),
                format!("Assertion failed. Got: {}", error));

        Ok(())
    } else {
        Err(String::from("Expected to get an error when the x value is zero"))
    }
}

#[test]
fn setting_y_value_of_zero_fails() -> Result<(), String> {
    let mut tga_file = tga::TGAImage::new(10, 10, 4);

    let color = drawing::Color32::new(0, 0, 0, 255);

    if let Err(e) = tga_file.set(1, 0, &color) {
        assert!(e.contains("Invalid y value 0"),
                format!("Assertion failed. Got: {}", e));
        Ok(())
    } else {
        Err(String::from("Expected to get an error when y value is zero"))
    }
}
