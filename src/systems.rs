use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::InputHandler,
    ui::UiText,
};
use crate::{
    components::{Cursor, Tile},
    states::Map,
};
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
pub struct CursorMovementSystem {
    is_moving: [bool; 4],
}

impl<'a> System<'a> for CursorMovementSystem {
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

                    transform.translation[0] = f32::from(cursor.0) * 33.0;
                    transform.translation[1] = f32::from(cursor.1) * 33.0;
                }
            }
        }
    }
}

// TODO: combine stuff for perf optims
pub struct CursorHoverInfoSystem;

impl<'a> System<'a> for CursorHoverInfoSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        ReadStorage<'a, Cursor>,
        ReadStorage<'a, Tile>,
        Read<'a, Map>,
        ReadExpect<'a, crate::states::Ui>,
    );

    fn run(&mut self, (mut ui_text, cursors, tiles, map, ui): Self::SystemData) {
        for cursor in cursors.join() {
            let hovered_tile_entity = map.tiles[(cursor.1 * map.height + cursor.0) as usize];
            let hovered_tile = tiles.get(hovered_tile_entity).unwrap();

            if let Some(text) = ui_text.get_mut(ui.0) {
                text.text = format!("Terrain: {:?}", hovered_tile.terrain);
            }
        }
    }
}
