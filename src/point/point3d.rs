
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
    pub fn dot(&self, other :&Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other :&Point3D) -> Point3D {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}



