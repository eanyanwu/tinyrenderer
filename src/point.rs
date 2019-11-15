use std::ops::Sub;

use crate::vector;

// 2-dimensional points
#[derive(Clone, Copy)]
pub struct Point2D { 
    pub x: f64, 
    pub y: f64 
}

// 3-dimensional points
#[derive(Clone, Copy)]
pub struct Point3D { 
    pub x: f64,
    pub y: f64, 
    pub z: f64 
}

// Implement subtraction for Points
// PointB - PointA = Vector AB
impl Sub for Point2D {
    type Output = vector::Vector2D;

    fn sub(self, other: Point2D) -> vector::Vector2D {
        vector::Vector2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Sub for Point3D {
    type Output = vector::Vector3D;

    fn sub (self, other: Point3D) -> vector::Vector3D {
        vector::Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
