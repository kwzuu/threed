use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// get the magnitude
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x +
            self.y * self.y +
            self.z * self.z).sqrt()
    }

    /// normalize the vector, i.e. make its magnitude 1
    pub fn normalize(&self) -> Self {
        self.inverse_scale(self.magnitude())
    }

    /// multiply by scalar
    pub fn scale(&self, n: f64) -> Self {
        Self::new(
            self.x * n,
            self.y * n,
            self.z * n,
        )
    }

    /// divide by scalar
    pub fn inverse_scale(&self, n: f64) -> Self {
        self.scale(1.0 / n)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}
