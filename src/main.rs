#![feature(const_fn_floating_point_arithmetic)]
mod physics;

use std::{thread, time};
use kiss3d::window::Window;
use crate::physics::simulation::GravitySimulation;

fn main() {
    let mut window = Window::new("physics");
    let mut simulation = GravitySimulation::new(20, 60., &mut window);

    while window.render() {
        simulation.tick();
        thread::sleep(time::Duration::from_secs_f32(1./60.));
    }
}