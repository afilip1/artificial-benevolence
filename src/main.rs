mod components;
mod states;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DrawFlat, PosTex},
};

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let display_config = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let bindings_config = format!(
        "{}/resources/bindings_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::MovementSystem,
            "movement_system",
            &["input_system"],
        ).with_basic_renderer(display_config, DrawFlat::<PosTex>::new(), false)?;

    Application::build(assets_dir, states::Game)?
        .build(game_data)?
        .run();

    Ok(())
}
