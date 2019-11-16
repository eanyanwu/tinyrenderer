mod tga;
mod wavefront;
mod drawing;
mod color;
mod point;
mod vector;
mod bytereader;

use std::fs;

fn main() {
    let bytes = fs::read("obj/head_diffuse.tga").unwrap();
    tga::TGAFile::from_bytes(bytes).unwrap();
}

pub fn face() {
    let model = wavefront::WaveFrontFile::new("obj/head.obj").unwrap();
    
    let bytes = fs::read("obj/head_diffuse.tga").unwrap();
    let mut texture = tga::TGAFile::from_bytes(bytes).unwrap();

    let width: u16 = 800;
    let height: u16 = 800;

    let mut zbuffer = vec![std::f64::MIN; width as usize * height as usize];

    let mut image = tga::TGAFile::new(width, height);

    for i in 0..model.face_count() {
        let face = model.get_face(i);
        let v0 = model.get_vertex(face.vertices[0]);
        let v1 = model.get_vertex(face.vertices[1]);
        let v2 = model.get_vertex(face.vertices[2]);

        let t0 = model.get_texture(face.textures[0]);
        let t1 = model.get_texture(face.textures[1]);
        let t2 = model.get_texture(face.textures[2]);
        
        let r0 = point::Point3D { x: (v0.x + 1.0) * ((width as f64)/2.0 - 1.0), y: (v0.y + 1.0) * ((height as f64)/2.0 - 1.0), z: v0.z }; 
        let r1 = point::Point3D { x: (v1.x + 1.0) * ((width as f64)/2.0 - 1.0), y: (v1.y + 1.0) * ((height as f64)/2.0 - 1.0), z: v1.z };
        let r2 = point::Point3D { x: (v2.x + 1.0) * ((width as f64)/2.0 - 1.0), y: (v2.y + 1.0) * ((height as f64)/2.0 - 1.0), z: v2.z };

        let vector_ab = v1 - v0;

        let vector_ac = v2 - v0;

        let normalized_cross_product = vector::cross_product(&vector_ab, &vector_ac).normalized();

        let intensity = normalized_cross_product.z * 1.0;

        if intensity > 0.0 {
            drawing::triangle(r0, r1, r2, t0, t1, t2, &mut image, &mut zbuffer, &mut texture, &color::Color32::new((intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8, 255));
        }
    }

    image.write_tga_file("model.tga");
    texture.write_tga_file("texture.tga");
}
