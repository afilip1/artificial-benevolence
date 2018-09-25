use amethyst::ecs::Entity;

#[derive(Default, Clone)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Entity>,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Map {
        Map {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
        }
    }
}
