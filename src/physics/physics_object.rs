use std::cell::Cell;
use std::ops::Add;
use crate::physics::forces::Force;
use crate::physics::vectors::Vector3;

/// an object that has physics
pub struct PhysicsObject {
    /// uniqueness!
    /// please dont make 2^128-1 physics objects
    uuid: u128,
    /// position in m
    pub position: Vector3,
    /// velocity in m/s
    pub velocity: Vector3,
    /// mass in kg
    pub mass: f64,
}

static UUID: Cell<u128> = Cell::new(0);

impl PhysicsObject {
    pub fn new(position: Vector3, mass: f64) -> PhysicsObject {
        PhysicsObject {
            uuid: UUID.update(|x| x+1),
            position,
            velocity: Default::default(),
            mass
        }
    }

    /// distance from another object in metres
    pub fn distance_from(&self, other: &PhysicsObject) -> f64 {
        (self.position - other.position).magnitude()
    }

    /// applies a force (newtons) for a time (seconds)
    pub fn apply_force(&mut self, f: Force, t: f64) {
        self.velocity += f.0 * t;
    }

    pub fn apply_forces<I: Iterator<Item=&Force>>(&mut self, forces: I, time: f64) {
        let total = forces.fold(Force::ZERO, Force::add);
        self.apply_force(total, time);
    }
}
