use amethyst::{
    assets::Loader,
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
        SpriteSheetHandle, TextureCoordinates,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use crate::components;
use itertools::iproduct;

const ARENA_WIDTH: f32 = 479.0;
const ARENA_HEIGHT: f32 = 329.0;

#[derive(Default, Clone)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Entity>,
}

pub struct Ui(pub Entity);

pub struct MapState {
    pub dimensions: (u8, u8),
}

impl<'a, 'b> SimpleState<'a, 'b> for MapState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.register::<components::Unit>();

        let sprite_sheet_handle = self.load_sprite_sheet(data.world);

        self.init_tiles(data.world, sprite_sheet_handle.clone());
        self.init_units(data.world, sprite_sheet_handle.clone());
        self.init_cursor(data.world, sprite_sheet_handle);
        self.init_ui(data.world);
        self.init_camera(data.world);
    }
}

impl MapState {
    fn load_sprite_sheet(&self, world: &mut World) -> SpriteSheetHandle {
        let texture_handle = world.read_resource::<Loader>().load(
            "spritesheet.png",
            PngFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let texture_id = 0;
        world
            .write_resource::<MaterialTextureSet>()
            .insert(texture_id, texture_handle);

        // TODO: reduce boilerplate (macros?)
        let ground_tex = TextureCoordinates {
            left: 0.0,
            right: 0.5,
            bottom: 0.5,
            top: 1.0,
        };

        let ground_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: ground_tex,
        };

        let water_tex = TextureCoordinates {
            left: 0.5,
            right: 1.0,
            bottom: 0.5,
            top: 1.0,
        };

        let water_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: water_tex,
        };

        let cursor_tex = TextureCoordinates {
            left: 0.0,
            right: 0.5,
            bottom: 0.0,
            top: 0.5,
        };

        let cursor_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: cursor_tex,
        };

        let tank_tex = TextureCoordinates {
            left: 0.5,
            right: 1.0,
            bottom: 0.0,
            top: 0.5,
        };

        let tank_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: tank_tex,
        };

        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites: vec![ground_sprite, water_sprite, cursor_sprite, tank_sprite],
        };

        world
            .read_resource::<Loader>()
            .load_from_data(sprite_sheet, (), &world.read_resource())
    }

    fn init_tiles(&mut self, world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
        let sprite_render_ground = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let sprite_render_water = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 1,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let (width, height) = self.dimensions;
        let mut tiles = Vec::with_capacity((width * height) as usize);

        for (y, x) in iproduct!(0..height, 0..width) {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(f32::from(x) * 33.0, f32::from(y) * 33.0, 0.0);

            let mut tile = world
                .create_entity()
                .with(GlobalTransform::default())
                .with(transform);

            if rand::random::<f32>() > 0.86 {
                tile = tile
                    .with(sprite_render_water.clone())
                    .with(components::Tile {
                        terrain: components::Terrain::Water,
                    });
            } else {
                tile = tile
                    .with(sprite_render_ground.clone())
                    .with(components::Tile {
                        terrain: components::Terrain::Ground,
                    });
            };

            tiles.push(tile.build());
        }

        world.add_resource(Map {
            width,
            height,
            tiles,
        });
    }

    fn init_units(&mut self, world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
        let sprite_render_tank = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 3,
            flip_horizontal: false,
            flip_vertical: false,
        };
        
        world.create_entity()
            .with(GlobalTransform::default())
            .with(Transform::default())
            .with(components::Unit)
            .with(sprite_render_tank)
            .build();
    }

    fn init_cursor(&self, world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
        let sprite_render_cursor = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 2,
            flip_horizontal: false,
            flip_vertical: false,
        };

        world
            .create_entity()
            .with(sprite_render_cursor)
            .with(components::Cursor::default())
            .with(GlobalTransform::default())
            .with(Transform::default())
            .build();
    }

    fn init_ui(&mut self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "square.ttf",
            TtfFormat,
            (),
            (),
            &world.read_resource(),
        );

        let ui_transform = UiTransform::new(
            "ui".to_string(),
            Anchor::TopRight,
            -140.0,
            30.0,
            1.0,
            300.0,
            50.0,
            0,
        );

        let ui = world
            .create_entity()
            .with(ui_transform)
            .with(UiText::new(font, String::new(), [1.0, 1.0, 1.0, 1.0], 36.0))
            .build();

        world.add_resource(Ui(ui));
    }

    fn init_camera(&self, world: &mut World) {
        let projection = Projection::orthographic(0.0, ARENA_WIDTH, ARENA_HEIGHT, 0.0);
        let matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0));

        world
            .create_entity()
            .with(Camera::from(projection))
            .with(GlobalTransform(matrix))
            .build();
    }
}
