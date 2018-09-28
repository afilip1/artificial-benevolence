use amethyst::ecs::prelude::*;

#[derive(Debug)]
pub enum UnitKind {
    Tank,
}

#[derive(Debug)]
pub struct Unit {
    pub kind: UnitKind,
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}
