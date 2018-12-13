
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D {x: x, y: y, z: z}
    }
    pub fn length(&self) -> f64 {
        let l :f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
    pub fn normalize(&mut self) {
        let l :f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
    pub fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
    pub fn dot(&self, other :Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other :Point3D) -> Point3D {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl Add for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl Mul for Point3D {
    type Output = Point3D;
    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}
impl Div for Point3D {
    type Output = Point3D;
    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}
impl Add<f64> for Point3D {
    type Output = Point3D;
    fn add(self, other: f64) -> Point3D {
        Point3D {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_len() {
        let p: Point3D = Point3D::new(1.0, 0.0, 0.0);
        assert_eq!(p.length(), 1.0_f64);
    }

    #[test]
    fn test_point3d_normal() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p.normalize();
        assert_eq!(p.length(), 1.0_f64);
        assert!(p.x == p.y && p.y == p.z);
    }

    #[test]
    fn test_point3d_add_point3d() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        // 自身同士でも大丈夫
        p = p + p;
        assert_eq!(p.x, 20.0);
    }

    #[test]
    fn test_point3d_add_f64() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p = p + 10.0;
        assert_eq!(p.x, 20.0);
    }
}


