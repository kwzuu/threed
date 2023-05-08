use crate::physics::physics_object::PhysicsObject;
use rand::random;
use crate::physics::forces::Force;
use kiss3d::nalgebra::{Translation3, Vector3};
use kiss3d::window::Window;

/// a simulation of many-body gravity to test stuff out
pub struct GravitySimulation {
    objects: Vec<PhysicsObject>,
    /// ticks per second
    tickrate: f32,
}

fn sphere_radius_from_volume(volume: f32) -> f32 {
    (volume * 0.75).cbrt()
}

impl GravitySimulation {
    pub fn new(count: usize, tickrate: f32, window: &mut Window) -> Self {
        let mut objects = Vec::with_capacity(count);

        for _ in 0..count {
            let mass = random::<f32>() * 200000.;
            let radius = sphere_radius_from_volume(mass / 20000.);
            let position = Vector3::<f32>::new(random(), random(), random());

            let mut node = window.add_sphere(radius);
            node.set_local_translation({
                let [x, y, z] = position.data.0[0];
                Translation3::new(x, y, z)
            });

            objects.push(PhysicsObject::new(
                position,
                mass,
                node
            ))
        }

        Self { objects, tickrate }
    }

    /// processes a tick in the simulation
    /// in the long-term, this could be improved to use half the cpu time by knowing the
    /// fact that gravity is the same both ways for two objects
    pub fn tick(&mut self) {
        let frametime = 1./self.tickrate;

        let mut gravities: Vec<Vec<Force>> = Vec::with_capacity(self.objects.len());

        for (index, o1) in self.objects.iter().enumerate() {
            gravities.push(vec![]);

            for o2 in self.objects.iter() {
                // same index -> same object, meaningless gravity
                if o1 == o2 {
                    continue;
                }

                gravities[index].push(Force::gravity(o1, o2));
            }
        }

        for (index, forces) in gravities.iter().enumerate() {
            let object = self.objects.get_mut(index).unwrap();
            object.apply_forces(forces.iter(), frametime);
        }

        for object in self.objects.iter_mut() {
            object.tick(frametime)
        }
    }
}
