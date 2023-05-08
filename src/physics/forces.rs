use std::ops::{Add, Neg, Sub};
use crate::physics::physics_object::PhysicsObject;
use kiss3d::nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Force(pub(crate) Vector3<f32>);

pub const GRAVITATIONAL_CONSTANT: f32 = 6.6743e-11;

impl Force {
    pub fn zeroes() -> Self {
        Self(Vector3::zeros())
    }

    pub fn gravity(o1: &PhysicsObject, o2: &PhysicsObject) -> Self {
        let diff = o1.position - o2.position;

        let distance = diff.magnitude();

        // Gm1m2/d^2
        let mag = GRAVITATIONAL_CONSTANT
            * o1.mass * o2.mass
            / (distance * distance);

        Self(-diff.normalize().scale(mag))
    }
}

impl Add for Force {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Neg for Force {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sub for Force {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}