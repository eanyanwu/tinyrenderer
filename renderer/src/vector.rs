use std::ops::Mul;

// 2-dimensional vectors
#[derive(Copy, Clone)]
pub struct Vector2D { 
    pub x: f64,
    pub y: f64
}

// 3-dimensional vectors
#[derive(Copy, Clone)]
pub struct Vector3D { 
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// Methods for finding the length of the vector
impl Vector3D {
    pub fn normalized(&self) -> Vector3D {
        *self * (1.0/self.length())
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Vector2D {
    pub fn normalized(&self) -> Vector2D {
        *self * (1.0/self.length())
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Scalar multiplication 
impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

// Perform the cross product of two 3-dimensional vectors
// The Copy trait is specified to ensure that the concrete types 
// don't need to be "moved"
pub fn cross_product(a: &Vector3D, b: &Vector3D) -> Vector3D
{
    Vector3D {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x
    }
}
