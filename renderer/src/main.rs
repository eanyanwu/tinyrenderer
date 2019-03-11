use renderer::file_formats::tga;
use renderer::file_formats::wavefront;
use renderer::drawing;

fn main() {
    let model = wavefront::WaveFrontFile::new("obj/head.obj").unwrap();
    
    let white = drawing::Color32::new(255, 255, 255, 255);
    
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

            drawing::Line::new(x0 as u16, y0 as u16, x1 as u16, y1 as u16, &mut image, &white);
        }
    }

    image.write_tga_file("model.tga");
}

