use super::components;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        Camera, Event, PngFormat, Projection, Sprite, Texture, VirtualKeyCode, WithSpriteRender,
    },
};

const ARENA_HEIGHT: f32 = 360.0;
const ARENA_WIDTH: f32 = 640.0;

pub struct Game;

impl<'a, 'b> State<GameData<'a, 'b>> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.register::<components::Player>();

        init_player(data.world);
        init_camera(data.world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn init_player(world: &mut World) {
    let spritesheet = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "player.png",
            PngFormat,
            Default::default(),
            (),
            &texture_storage,
        )
    };

    let sprite = Sprite {
        left: 0.0,
        right: 10.0,
        top: 0.0,
        bottom: 10.0,
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), (10.0, 10.0))
        .expect("sprite failed")
        .with(components::Player)
        .with(GlobalTransform::default())
        .with(transform)
        .build();
}

fn init_camera(world: &mut World) {
    let projection = Projection::orthographic(0.0, ARENA_WIDTH, ARENA_HEIGHT, 0.0);
    let matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0));

    world
        .create_entity()
        .with(Camera::from(projection))
        .with(GlobalTransform(matrix))
        .build();
}
