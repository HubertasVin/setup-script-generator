mod question_type;
mod setup_generator;
mod ui;
use eframe::{egui::ViewportBuilder, run_native, CreationContext, NativeOptions};
use setup_generator::{SetupGenerator, Tab};

fn main() {
    let win_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(eframe::egui::Vec2::from([660.0, 800.0])),
        ..Default::default()
    };

    run_native(
        "Setup script generator",
        win_options,
        Box::new(move |_cc: &CreationContext| {
            Box::new(SetupGenerator::new())
        }),
    )
    .unwrap_or_else(|err| eprintln!("Error: {:?}", err));
}
