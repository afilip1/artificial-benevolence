use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub enum Terrain {
    Ground,
    Water,
}

pub struct Tile {
    pub terrain: Terrain,
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            terrain: Terrain::Ground,
        }
    }
}
