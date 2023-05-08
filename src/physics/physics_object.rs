use std::fmt::{Debug, Formatter};
use kiss3d::scene::SceneNode;
use crate::physics::forces::Force;
use kiss3d::nalgebra::{Translation3, Vector3};

/// an object that has physics
pub struct PhysicsObject {
    /// uniqueness!
    /// please dont make 2^128-1 physics objects
    uuid: u128,
    /// position in m
    pub position: Vector3<f32>,
    /// velocity in m/s
    pub velocity: Vector3<f32>,
    /// mass in kg
    pub mass: f32,
    /// our scene node
    pub node: SceneNode
}

///TODO: make this multithreadable
static mut UUID: u128 = 0;

impl PhysicsObject {
    pub fn new(position: Vector3<f32>, mass: f32, node: SceneNode) -> PhysicsObject {

        PhysicsObject {
            uuid: unsafe {
                let tmp = UUID;
                UUID += 1;
                tmp
            },
            position,
            velocity: Default::default(),
            mass,
            node,
        }
    }

    /// distance from another object in metres
    pub fn distance_from(&self, other: &PhysicsObject) -> f32 {
        (self.position - other.position).magnitude()
    }

    /// applies a force (newtons) for a time (seconds)
    pub fn apply_force(&mut self, force: Force, time: f32) {
        self.velocity += force.0.scale(time);
    }

    pub fn apply_forces<'a, I: Iterator<Item=&'a Force>>(&mut self, forces: I, time: f32) {
        let total = forces.fold(Force::zeroes(), |x, y| x + *y);
        self.apply_force(total, time);
    }

    pub fn tick(&mut self, time: f32) {
        self.position += self.velocity.scale(time);
        self.node.set_local_translation({
            let [x, y, z] = self.position.data.0[0];
            Translation3::new(x, y, z)
        });
    }
}

impl PartialEq for PhysicsObject {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Debug for PhysicsObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhysicsObject")
            .field("uuid", &self.uuid)
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .field("mass", &self.mass)
            .finish()
    }
}