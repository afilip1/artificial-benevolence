mod components;
mod states;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage, ALPHA,
    },
    ui::{DrawUi, UiBundle},
};

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let render_bundle = {
        let display_config = {
            let path = format!(
                "{}/resources/display_config.ron",
                env!("CARGO_MANIFEST_DIR")
            );
            DisplayConfig::load(&path)
        };

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawSprite::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    Some(DepthMode::LessEqualWrite),
                )).with_pass(DrawUi::new()),
        );

        RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor()
    };

    let input_bundle = {
        let binding_path = format!(
            "{}/resources/bindings_config.ron",
            env!("CARGO_MANIFEST_DIR")
        );

        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(render_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(
            systems::CursorMovementSystem::default(),
            "cursor_movement_system",
            &["input_system"],
        ).with(
            systems::CursorHoverInfoSystem,
            "cursor_hover_info_system",
            &["cursor_movement_system"],
        );

    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));
    let initial_state = states::MapState {
        dimensions: (10, 10),
    };

    Application::build(assets_dir, initial_state)?
        .build(game_data)?
        .run();

    Ok(())
}
