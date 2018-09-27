use amethyst::ecs::prelude::*;

pub struct Unit;

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}