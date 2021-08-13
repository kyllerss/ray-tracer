use crate::domain::{Point, Vector};

#[derive(Debug)]
pub struct Tick {
    pub gravity: Vector,
    pub wind: Vector,
    pub projectile: Point,
    pub projectile_velocity: Vector,
}

pub fn apply_tick(tick: &mut Tick) {
    tick.gravity = tick.gravity + tick.gravity; // gravity accelerates every tick
    let env_forces = tick.gravity + tick.wind;
    tick.projectile = tick.projectile + (tick.projectile_velocity + env_forces);
}

pub fn run() {
    println!("Running ch1...");

    let mut tick = Tick {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
        projectile: Point::new(0.0, 1.0, 0.0),
        projectile_velocity: Vector::new(1.0, 1.0, 0.0),
    };
    for i in 0..10 {
        apply_tick(&mut tick);
        println!("Iteration {}: {:?}", i, &tick);
    }
}
