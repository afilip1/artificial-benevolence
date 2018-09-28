#[macro_use]
mod common;

use amethyst::{
    assets::Loader,
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetHandle,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use crate::components;
use itertools::iproduct;
use std::iter;


#[derive(Clone)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Entity>,
    pub units: Vec<Option<Entity>>,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Map {
        Map {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
            units: iter::repeat(None).take((width * height) as usize).collect(),
        }
    }
}

pub struct Ui {
    pub terrain: Entity,
    pub unit: Entity,
    pub selected_unit: Entity,
}

pub struct MapState {
    pub map: Map,
}

impl<'a, 'b> SimpleState<'a, 'b> for MapState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let sprite_sheet_handle = self.load_sprite_sheet(data.world);

        self.init_tiles(data.world, sprite_sheet_handle.clone());
        self.init_units(data.world, sprite_sheet_handle.clone());
        self.init_cursor(data.world, sprite_sheet_handle);
        self.init_ui(data.world);
        self.init_camera(data.world);

        data.world.add_resource(self.map.clone());
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

        let sprites = sprites! {
            (64.0, 64.0),
            ground => 0.0, 0.0, 32.0, 32.0,
            water => 32.0, 0.0, 32.0, 32.0,
            cursor => 0.0, 32.0, 32.0, 32.0,
            tank => 32.0, 32.0, 32.0, 32.0
        };

        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites,
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

        for (y, x) in iproduct!(0..self.map.height, 0..self.map.width) {
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

            self.map.tiles.push(tile.build());
        }
    }

    fn init_units(&mut self, world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
        let sprite_render_tank = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 3,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let unit = world
            .create_entity()
            .with(GlobalTransform::default())
            .with(Transform::default())
            .with(components::Unit {
                kind: components::UnitKind::Tank,
            }).with(sprite_render_tank)
            .build();

        self.map.units[0] = Some(unit);
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

    fn init_ui(&self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "square.ttf",
            TtfFormat,
            (),
            (),
            &world.read_resource(),
        );

        let terrain = {
            let transform = UiTransform::new(
                "ui_terrain".to_string(),
                Anchor::TopRight,
                -140.0,
                30.0,
                1.0,
                300.0,
                50.0,
                0,
            );

            world
                .create_entity()
                .with(transform)
                .with(UiText::new(
                    font.clone(),
                    String::new(),
                    [1.0, 1.0, 1.0, 1.0],
                    36.0,
                )).build()
        };

        let unit = {
            let transform = UiTransform::new(
                "ui_unit".to_string(),
                Anchor::TopRight,
                -140.0,
                80.0,
                1.0,
                300.0,
                50.0,
                0,
            );

            world
                .create_entity()
                .with(transform)
                .with(UiText::new(
                    font.clone(),
                    String::new(),
                    [1.0, 1.0, 1.0, 1.0],
                    36.0,
                )).build()
        };

        let selected_unit = {
            let transform = UiTransform::new(
                "ui_selected_unit".to_string(),
                Anchor::TopRight,
                -140.0,
                130.0,
                1.0,
                300.0,
                50.0,
                0,
            );

            world
                .create_entity()
                .with(transform)
                .with(UiText::new(
                    font,
                    "Selected: none".to_string(),
                    [1.0, 1.0, 1.0, 1.0],
                    36.0,
                )).build()
        };

        world.add_resource(Ui {
            terrain,
            unit,
            selected_unit,
        });
    }

    fn init_camera(&self, world: &mut World) {
        let projection = Projection::orthographic(0.0, 479.0, 329.0, 0.0);
        let matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0));

        world
            .create_entity()
            .with(Camera::from(projection))
            .with(GlobalTransform(matrix))
            .build();
    }
}
