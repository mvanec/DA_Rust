#![allow(dead_code)]
#![allow(unused)]

use std::time::{Duration, Instant};

struct Particle {
    position: f32,
    velocity: f32,
}

impl Particle {
    fn new(position: f32, velocity: f32) -> Self {
        Self { position, velocity }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

pub fn render_particle() {
    let mut particle = Particle::new(0.0, 1.0);
    let mut _last_update = Instant::now();
    let desired_timestep = Duration::from_millis(16); // 60 FPS

    loop {
        let current_time = Instant::now();
        let elapsed_time = current_time.elapsed();

        if elapsed_time < desired_timestep {
            std::thread::sleep(desired_timestep - elapsed_time);
        }

        let dt = elapsed_time.as_secs_f32();
        particle.update(dt);

        let difference = Instant::now() - current_time;
        println!("Particle Position after {:?}: {}", difference, particle.position);

        _last_update = current_time;
    }
}

pub fn render_variable() {
    let mut _last_update = Instant::now();
    let mut max_dt = Duration::from_millis(16); // 60 FPS
    let desired_timestep = Duration::from_millis(16);

    loop {
        let current_time = Instant::now();
        let elapsed_time = current_time.elapsed();

        if elapsed_time > max_dt {
            // Handle the "missed" updates (e.g., interpolate)
            // ...
            max_dt = elapsed_time;
        }

        if elapsed_time < desired_timestep {
            std::thread::sleep(desired_timestep - elapsed_time);
        }

        let difference = Instant::now() - current_time;
        println!("Difference is {:?}", difference);

        // ...
        _last_update = current_time;
    }
}

pub fn render() {
    let mut last_update = Instant::now();
    let desired_timestep = Duration::from_millis(16); // 60 FPS

    loop {
        let current_time = Instant::now();
        let elapsed_time = current_time.elapsed();

        if elapsed_time < desired_timestep {
            std::thread::sleep(desired_timestep - elapsed_time);
        }

        // Update your simulation here
        // ...
        let difference = Instant::now() - current_time;
        println!("Difference is {:?}", difference);

        last_update = current_time;
    }

}