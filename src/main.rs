use tinyrenderer::tga;

fn main() {
    let red = tga::TGAColor::new(255, 0, 0, 255);

    let mut image = tga::TGAImage::new(100, 100, 4);

    image.set(50, 50, &red);

    image.flip_vertically();

    image.write_tga_file("output.tga");
}
