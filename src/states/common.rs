#[macro_export]
macro_rules! sprites {
    ($dimensions:expr, $($sprite:ident => $left:expr, $top:expr, $width:expr, $height:expr),*) => ({
        use amethyst::renderer::{TextureCoordinates, Sprite};
        let mut sprites = vec![];
        $(
            let tex_coords = TextureCoordinates {
                left: $left / $dimensions.0,
                right: ($left + $width) / $dimensions.0,
                bottom: 1.0 - (($top + $height) / $dimensions.1),
                top: 1.0 - ($top / $dimensions.1),
            };

            let sprite = Sprite {
                width: $width,
                height: $height,
                offsets: [0.0, 0.0],
                tex_coords,
            };

            sprites.push(sprite);
        )*;
        sprites
    });
}

use amethyst::ecs::prelude::*;
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
