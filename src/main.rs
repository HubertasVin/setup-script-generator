mod question_type;
mod setup_generator;
mod ui;
use eframe::{egui::ViewportBuilder, run_native, CreationContext, NativeOptions};
use setup_generator::{SetupGenerator, Tab};

fn main() {
    let window_dimensions = eframe::egui::Vec2::from([600.0, 800.0]);
    let win_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(window_dimensions.clone()),
        ..Default::default()
    };

    run_native(
        "Setup script generator",
        win_options,
        Box::new(move |_cc: &CreationContext| {
            Box::new(SetupGenerator::new(window_dimensions.clone()))
        }),
    )
    .unwrap_or_else(|err| eprintln!("Error: {:?}", err));
}
