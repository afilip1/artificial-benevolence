use super::components;
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::*,
    input::InputHandler,
};

const VELOCITY: f32 = 100.0;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, components::Player>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, time, input): Self::SystemData) {
        for (_, transform) in (&players, &mut transforms).join() {
            if let Some(mv_amt) = input.axis_value("horizontal") {
                transform.translation[0] += time.delta_seconds() * VELOCITY * mv_amt as f32;
            }
            if let Some(mv_amt) = input.axis_value("vertical") {
                transform.translation[1] += time.delta_seconds() * VELOCITY * mv_amt as f32;
            }
        }
    }
}
