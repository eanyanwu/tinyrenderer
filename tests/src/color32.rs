use renderer::drawing::Color32;

#[test]
pub fn test_color_white() {
    let color = Color32::new(0, 0, 0, 0);
    
    let pixel_value = color.get_pixel_value();

    assert_eq!(0b00000000_00000000_00000000_00000000, pixel_value);
}

#[test]
pub fn test_color_black() {
    let color = Color32::new(255, 255, 255, 255);

    let pixel_value = color.get_pixel_value();

    assert_eq!(0b11111111_11111111_11111111_11111111, pixel_value);
}

#[test]
pub fn test_color_red() {
    let color = Color32::new(255, 0, 0, 0);

    let pixel_value = color.get_pixel_value();

    assert_eq!(0b11111111_00000000_00000000_00000000, pixel_value);
}
