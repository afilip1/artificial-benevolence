use super::{components, resources};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        cgmath::{Matrix4, Vector3},
        transform::{GlobalTransform, Transform},
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, Sprite, SpriteRender, SpriteSheet,
        Texture, TextureCoordinates,
    },
};
use itertools::iproduct;

const ARENA_WIDTH: f32 = 329.0;
const ARENA_HEIGHT: f32 = 329.0;

pub struct Game {
    pub map: resources::Map,
}

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.register::<components::Tile>();
        data.world.register::<components::Cursor>();

        self.init_tiles(data.world);
        self.init_cursor(data.world);
        self.init_camera(data.world);

        data.world.add_resource(self.map.clone());
    }
}

impl Game {
    fn init_cursor(&self, world: &mut World) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "cursor.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        let texture_id = 0;
        {
            let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
            material_texture_set.insert(texture_id, texture_handle);
        }

        let tex_coords = TextureCoordinates {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        };

        let cursor_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords,
        };

        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites: vec![cursor_sprite],
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
        };

        let sprite_render_cursor = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
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

    fn init_tiles(&mut self, world: &mut World) {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "terrain.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        let texture_id = 1;
        {
            let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
            material_texture_set.insert(texture_id, texture_handle);
        }

        let ground_tex = TextureCoordinates {
            left: 0.0,
            right: 0.5,
            bottom: 0.0,
            top: 1.0,
        };

        let ground_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: ground_tex,
        };

        let river_tex = TextureCoordinates {
            left: 0.5,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        };

        let river_sprite = Sprite {
            width: 32.0,
            height: 32.0,
            offsets: [0.0, 0.0],
            tex_coords: river_tex,
        };

        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites: vec![ground_sprite, river_sprite],
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
        };

        let sprite_render_ground = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let sprite_render_river = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 1,
            flip_horizontal: false,
            flip_vertical: false,
        };

        for (y, x) in iproduct!(0..self.map.height, 0..self.map.width) {
            let mut transform = Transform::default();
            transform.translation = Vector3::new(x as f32 * 33.0, y as f32 * 33.0, 0.0);

            if rand::random::<f32>() > 0.9 {
                self.map.tiles.push(
                    world
                        .create_entity()
                        .with(sprite_render_river.clone())
                        .with(components::Tile {
                            terrain: components::Terrain::Water,
                        }).with(GlobalTransform::default())
                        .with(transform)
                        .build(),
                );
            } else {
                self.map.tiles.push(
                    world
                        .create_entity()
                        .with(sprite_render_ground.clone())
                        .with(components::Tile {
                            terrain: components::Terrain::Ground,
                        }).with(GlobalTransform::default())
                        .with(transform)
                        .build(),
                );
            };
        }
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
