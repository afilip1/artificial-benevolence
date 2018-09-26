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
        SpriteSheetHandle, TextureCoordinates,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use itertools::iproduct;

const ARENA_WIDTH: f32 = 479.0;
const ARENA_HEIGHT: f32 = 329.0;

pub struct Game {
    pub map: resources::Map,
}

impl<'a, 'b> SimpleState<'a, 'b> for Game {
    fn on_start(&mut self, data: StateData<GameData>) {
        let sprite_sheet_handle = self.load_sprite_sheet(data.world);

        self.init_tiles(data.world, sprite_sheet_handle.clone());
        self.init_cursor(data.world, sprite_sheet_handle);
        self.init_ui(data.world);
        self.init_camera(data.world);

        data.world.add_resource(self.map.clone());
    }
}

pub struct Ui(pub Entity);

impl Game {
    fn load_sprite_sheet(&self, world: &mut World) -> SpriteSheetHandle {
        let texture_handle = world.read_resource::<Loader>().load(
            "spritesheet.png",
            PngFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let texture_id = 0;
        let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
        material_texture_set.insert(texture_id, texture_handle);

        // TODO: reduce boilerplate
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

        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites: vec![ground_sprite, water_sprite, cursor_sprite],
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
        };

        sprite_sheet_handle
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
            transform.translation = Vector3::new(x as f32 * 33.0, y as f32 * 33.0, 0.0);

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
            Default::default(),
            (),
            &world.read_resource(),
        );

        let ui_transform = UiTransform::new(
            "ui".to_string(),
            Anchor::TopRight,
            -140.,
            30.,
            1.,
            300.,
            50.,
            0,
        );

        let ui = world
            .create_entity()
            .with(ui_transform)
            .with(UiText::new(
                font.clone(),
                "Terrain: ".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                36.,
            )).build();

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
