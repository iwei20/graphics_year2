#[derive(Clone, Copy, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64, 
    pub z: f64
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn from_point(p: (f64, f64, f64)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            z: p.2
        }
    }

    pub fn from_points(p0: (f64, f64, f64), p1: (f64, f64, f64)) -> Self {
        Self {
            x: p1.0 - p0.0,
            y: p1.1 - p0.1,
            z: p1.2 - p0.2
        }
    }

    pub fn dot(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}
