use amethyst::ecs::prelude::*;

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
