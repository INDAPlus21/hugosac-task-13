use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let magnitude: f32 = self.magnitude();
        Vector3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }

    pub fn to_string(&self) -> String {
        format!("x: {},\ny: {},\nz: {}", self.x, self.y, self.z)
    }

    pub fn dot(u: Vector3, v: Vector3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vector3, v: Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

    


}

// Addition for vectors u + v
impl ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// Subtraction for vectors u - v
impl ops::Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// Scaling vectors k * v
impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vec: Vector3) -> Vector3 {
        Vector3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z
        }
    }
}

// Negative vector -v
impl ops::Neg<> for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

// Addition assign u += v
impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}   

// Subtraction assign u -= v
impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// Multiplication assign v *= k
// Vector scaling
impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

// Division assign v /= k
impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, divider: f32) {
        *self = Self {
            x: self.x / divider,
            y: self.y / divider,
            z: self.z / divider
        }
    }
} 