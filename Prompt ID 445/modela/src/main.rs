#![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]

use modela::*;

use std::time::{Duration, Instant};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn winit_loop() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut last_update = Instant::now();
    let desired_timestep = Duration::from_millis(16);

    event_loop.run(move |event, _, control_flow| {
        // Handle events
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
        // Update logic
        let current_time = Instant::now();
        let elapsed_time = current_time.elapsed();

        if elapsed_time >= desired_timestep {
            // Update the game state here
            // ...
            last_update = current_time;
        }
        // ... (Rendering logic)
    });
}
fn main() {
    // render_variable();
    render();
}
