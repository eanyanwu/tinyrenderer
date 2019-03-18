use std::fs;
use std::str::FromStr;
use std::string::ToString;
use regex::Regex;
use crate::point;


pub struct WaveFrontFile {
    vertices: Vec<point::Point3D>,
    faces: Vec<Face>,
}

pub struct Face {
    pub vertices: [usize; 3] 
}

impl WaveFrontFile {
    pub fn new(filename: &str) -> Result<WaveFrontFile, String> {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => e.to_string()
        };

        let mut vertices: Vec<point::Point3D> = Vec::new();

        let mut faces: Vec<Face> = Vec::new();
        
        let vertex_regex = Regex::new(r"(?x)
                                      v                 # The literal letter v
                                      \s                # Whitespace
                                      (?P<x>[0-9-\.e]+) # A floating point number - x
                                      \s                # Whitespace
                                      (?P<y>[0-9-\.e]+) # A floating point number - y
                                      \s                # Whitespace
                                      (?P<z>[0-9-\.e]+) # A floating point number -z
                                      ").unwrap();

        let face_regex = Regex::new(r"(?x)
                                    f                               # The literal letter f
                                    \s                              # Whitespace
                                    (?P<v0>[0-9]*)/[0-9]*/[0-9]*    # <number>/<number>/<number
                                    \s                              # Whitespace
                                    (?P<v1>[0-9]*)/[0-9]*/[0-9]*    # <number>/<number>/<number>
                                    \s                              # Whitespace
                                    (?P<v2>[0-9]*)/[0-9]*/[0-9]*    # <number>/<number>/<number>
                                    ").unwrap();
        
        for line in contents.lines() {
            if line.starts_with("v ") {
                let captures = vertex_regex.captures(line).unwrap();

                vertices.push(point::Point3D {
                    x: f64::from_str(&captures["x"]).unwrap(),
                    y: f64::from_str(&captures["y"]).unwrap(),
                    z: f64::from_str(&captures["z"]).unwrap()
                });

            } else if line.starts_with("f ") {
                let captures = face_regex.captures(line).unwrap();
                
                // The indicies saved in the file are 1-indexed instead of 0 indexed. 
                // So we substract 1 from each one.
                faces.push(Face { vertices: [
                    usize::from_str(&captures["v0"]).unwrap() - 1, 
                    usize::from_str(&captures["v1"]).unwrap() - 1,
                    usize::from_str(&captures["v2"]).unwrap() - 1]
                });
            } else {
                //println!("Have not yet implemented parsing {}", line);
            }
        }

        Ok(WaveFrontFile { vertices , faces })
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn get_vertex(&self, idx: usize) -> point::Point3D {
        self.vertices[idx]
    }

    pub fn get_face(&self, idx: usize) -> &Face {
        &self.faces[idx]
    }
}
