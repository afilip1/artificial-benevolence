use amethyst::ecs::prelude::*;

pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Cursor(pub u8, pub u8);

impl Cursor {
    pub fn move_left(&mut self) {
        self.0 = self.0.checked_sub(1).unwrap_or(0);
    }

    pub fn move_right(&mut self, upper_bound: u8) {
        self.0 = self
            .0
            .checked_add(1)
            .unwrap_or(std::u8::MAX)
            .min(upper_bound);
    }

    pub fn move_up(&mut self, upper_bound: u8) {
        self.1 = self
            .1
            .checked_add(1)
            .unwrap_or(std::u8::MAX)
            .min(upper_bound);
    }

    pub fn move_down(&mut self) {
        self.1 = self.1.checked_sub(1).unwrap_or(0);
    }
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}
