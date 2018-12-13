
use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D {x: x, y: y, z: z}
    }
    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn length(self) -> f64 {
        let l :f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
    pub fn negate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }
    pub fn normalize(&mut self) {
        let l :f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
    pub fn distance(self, other :Point3D) -> f64 {
        let p: Point3D = other - self;
        p.length()
    }
    pub fn dot(self, other :Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(self, other :Point3D) -> Point3D {
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
impl Sub<f64> for Point3D {
    type Output = Point3D;
    fn sub(self, other: f64) -> Point3D {
        Point3D {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other
        }
    }
}
impl Mul<f64> for Point3D {
    type Output = Point3D;
    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}
impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}
impl Rem<f64> for Point3D {
    type Output = Point3D;
    fn rem(self, other: f64) -> Point3D {
        let neg_x: f64 = self.x.signum();
        let neg_y: f64 = self.y.signum();
        let neg_z: f64 = self.z.signum();
        Point3D {
            x: (self.x % other) * neg_x,
            y: (self.y % other) * neg_y,
            z: (self.z % other) * neg_z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_print() {
        let p: Point3D = Point3D::new(0.0, 1.0, 2.0);
        println!("Point3D Debug Check {:?}", p);
    }
    #[test]
    fn test_point3d_set() {
        let mut p: Point3D = Point3D::new(0.0, 1.0, 2.0);
        p.set(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0f64);
        assert_eq!(p.y, 2.0f64);
        assert_eq!(p.z, 3.0f64);
    }
    #[test]
    fn test_point3d_len() {
        let p: Point3D = Point3D::new(1.0, 0.0, 0.0);
        assert_eq!(p.length(), 1.0f64);
    }
    #[test]
    fn test_point3d_negate() {
        let mut p: Point3D = Point3D::new(1.0, 0.0, 0.0);
        p.negate();
        assert_eq!(p.x, -1.0f64);
    }
    #[test]
    fn test_point3d_normal() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p.normalize();
        assert_eq!(p.length(), 1.0f64);
        assert!(p.x == p.y && p.y == p.z);
    }
    #[test]
    fn test_point3d_distance() {
        let p: Point3D = Point3D::new(10.0, 0.0, 0.0);
        assert_eq!(p.distance(Point3D::new(0.0, 0.0, 0.0)), 10.0f64);
    }
    #[test]
    fn test_point3d_add_point3d() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p = p + p;
        assert_eq!(p.x, 20.0f64);
        assert!(p.x == p.y && p.y == p.z);
    }
    #[test]
    fn test_point3d_add_f64() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p = p + 10.0;
        assert_eq!(p.x, 20.0);
        assert!(p.x == p.y && p.y == p.z);
    }
    #[test]
    fn test_point3d_rem_f64() {
        let mut p: Point3D = Point3D::new(10.0, 10.0, 10.0);
        p = p % 7.0;
        assert_eq!(p.x, 3.0);
        p.negate();
        p = p % 7.0;
        assert_eq!(p.x, 3.0);
    }
}


