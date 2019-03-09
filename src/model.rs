use std::error::Error;
use std::fs;
use std::str::FromStr;

pub struct WaveFront {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
}

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64 
}

pub struct Face {
    pub vertices: Vec<usize>
}

impl WaveFront {
    pub fn new(filename: &str) -> Result<WaveFront, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let vertex_matches: Vec<&str> = contents.lines()
            .filter(|line| line.contains("v "))
            .collect();

        let face_matches: Vec<&str> = contents.lines()
            .filter(|line| line.contains("f "))
            .collect();
        
        for &vertex_entry in vertex_matches.iter() {
            let parts: Vec<&str> = vertex_entry.split(' ').collect();

            let x = f64::from_str(parts[1]).unwrap();
            let y = f64::from_str(parts[2]).unwrap();
            
            vertices.push(Vertex { x, y, z: 0.0 });
        }

        for &face_entry in face_matches.iter() {
            let parts: Vec<&str> = face_entry.split(' ').collect();
            
            let first_part: Vec<&str> = parts[1].split('/').collect();
            let second_part: Vec<&str> = parts[2].split('/').collect();
            let third_part: Vec<&str> = parts[3].split('/').collect();

            let index_v0 = usize::from_str(first_part[0]).unwrap();
            let index_v1 = usize::from_str(second_part[0]).unwrap();
            let index_v2 = usize::from_str(third_part[0]).unwrap();

            let face = Face { vertices: vec![index_v0 - 1 , index_v1 - 1, index_v2 - 1] };
            
            faces.push(face);
        }
        println!("Hih");
        Ok(WaveFront { vertices , faces })
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn get_vertex(&self, idx: usize) -> &Vertex {
        &self.vertices[idx]
    }

    pub fn get_face(&self, idx: usize) -> &Face {
        &self.faces[idx]
    }
}
