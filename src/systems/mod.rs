use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use crate::{components::Cursor, states::Map};
use itertools::izip;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOVES: [Box<dyn Fn(&mut Cursor, &Map) + Sync>; 4] = [
        Box::new(|c, _| c.move_left()),
        Box::new(|c, m| c.move_right(m.width - 1)),
        Box::new(|c, m| c.move_up(m.height - 1)),
        Box::new(|c, _| c.move_down()),
    ];
}

#[derive(Default)]
pub struct CursorSystem {
    is_moving: [bool; 4],
}

impl<'a> System<'a> for CursorSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Cursor>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, Map>,
    );

    fn run(&mut self, (mut transforms, mut cursors, input, map): Self::SystemData) {
        let actions_down: Vec<_> = ["left", "right", "up", "down"]
            .iter()
            .map(|&a| input.action_is_down(a).unwrap_or(false))
            .collect();

        for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            for (&is_down, is_moving, mov) in izip!(&actions_down, &mut self.is_moving, &MOVES[..])
            {
                if !is_down {
                    *is_moving = false;
                } else if !*is_moving {
                    *is_moving = true;
                    mov(cursor, &map);

                    transform.translation[0] = cursor.0 as f32 * 33.0;
                    transform.translation[1] = cursor.1 as f32 * 33.0;
                }
            }
        }
    }
}
