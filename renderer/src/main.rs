use renderer::file_formats::tga;
use renderer::file_formats::wavefront;
use renderer::drawing;
use renderer::point;
use renderer::vector;

fn main() {
    face();
    //triangles();
}
pub fn triangles() {
    let width = 200;
    let height = 200;
    
    let mut image = tga::TGAImage::new(width, height, 4);

    let white = drawing::Color32::new(255, 255, 255, 255);
    let red = drawing::Color32::new(255, 0, 0, 255);
    let blue = drawing::Color32::new(0, 255, 0, 255);

    let triangle1 = [
        point::Point3D { x: 10.0, y: 70.0, z: 0.0 },
        point::Point3D { x: 50.0, y: 160.0, z: 0.0 },
        point::Point3D { x: 70.0, y: 80.0, z: 0.0 }];

    let triangle2 = [
        point::Point3D { x: 180.0, y: 50.0, z: 0.0 },
        point::Point3D { x: 150.0, y: 1.0, z: 0.0 },
        point::Point3D { x: 70.0, y: 180.0, z: 0.0 }];

    let triangle3 = [
        point::Point3D { x: 180.0, y: 150.0, z: 0.0 },
        point::Point3D { x: 120.0, y: 160.0, z: 0.0 },
        point::Point3D { x: 130.0, y: 180.0, z: 0.0 }]; 

    drawing::triangle(triangle1[0], triangle1[1], triangle1[2], &mut image, &white); 
    drawing::triangle(triangle2[0], triangle2[1], triangle2[2], &mut image, &blue); 
    drawing::triangle(triangle3[0], triangle3[1], triangle3[2], &mut image, &red); 

    image.write_tga_file("triangle.tga");
}

pub fn face() {
    let model = wavefront::WaveFrontFile::new("obj/head.obj").unwrap();
    
    let white = drawing::Color32::new(255, 255, 255, 255);
    
    let width = 800;
    let height = 800;

    let mut image = tga::TGAImage::new(width, height, 4);

    for i in 0..model.face_count() {
        let face = model.get_face(i);
        let v0 = model.get_vertex(face.vertices[0]);
        let v1 = model.get_vertex(face.vertices[1]);
        let v2 = model.get_vertex(face.vertices[2]);
        
        let r0 = point::Point3D { x: (v0.x + 1.0) * (width - 1) as f64/2.0 + 1.0, y: (v0.y + 1.0) * (height - 1) as f64/2.0 + 1.0, z: v0.z }; 
        let r1 = point::Point3D { x: (v1.x + 1.0) * (width - 1) as f64/2.0 + 1.0, y: (v1.y + 1.0) * (height - 1) as f64/2.0 + 1.0, z: v1.z };
        let r2 = point::Point3D { x: (v2.x + 1.0) * (width - 1) as f64/2.0 + 1.0, y: (v2.y + 1.0) * (height - 1) as f64/2.0 + 1.0, z: v2.z };

        let vector_ab = v1 - v0;

        let vector_ac = v2 - v0;

        let normalized_cross_product = vector::cross_product(&vector_ab, &vector_ac).normalized();

        let intensity = normalized_cross_product.z * 1.0;

        if intensity > 0.0 {
            drawing::triangle(r0, r1, r2, &mut image, &drawing::Color32::new((intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8, 255));
        }
    }

    image.write_tga_file("model.tga");
}
