mod setup_generator;
use eframe::{
    egui::{
        CentralPanel, Color32, Pos2, Rounding, ScrollArea, SelectableLabel, Stroke, Ui,
        ViewportBuilder,
    },
    run_native, App, CreationContext, NativeOptions,
};
use setup_generator::{SetupGenerator, Tab};

impl App for SetupGenerator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            self.configure_fonts(ctx);
            self.initialize(ctx);
        }

        CentralPanel::default().show(ctx, |ui| {
            render_tabbar(ui, &mut self.current_tab);
            ScrollArea::vertical().show(ui, |ui| match self.current_tab {
                setup_generator::Tab::Tab1 => {
                    ui.label("Hello, world!");
                    for elem in &self.sections {
                        ui.label(elem);
                    }
                }
                setup_generator::Tab::Tab2 => {
                    ui.label("This is Tab 2");
                }
                setup_generator::Tab::Tab3 => {
                    ui.label("This is Tab 3");
                }
            });
        });
    }
}

fn render_tabbar(ui: &mut Ui, current_tab: &mut Tab) {
    let available_width = ui.available_width();
    let button_width = available_width / 3.0;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;

        // Customize visuals
        customize_visuals(ui);

        // Draw the SelectableLabels and vertical lines
        for (i, tab) in [Tab::Tab1, Tab::Tab2, Tab::Tab3].iter().enumerate() {
            let response = ui.add_sized(
                [button_width, 20.0],
                SelectableLabel::new(*current_tab == *tab, format!("Tab {}", i + 1)),
            );

            if response.clicked() {
                *current_tab = *tab;
            }

            if *current_tab == *tab {
                draw_horizontal_line(ui, &response.rect);
            }

            if i < 2 {
                draw_vertical_line(ui, &response.rect);
            }
        }
    });
}

fn customize_visuals(ui: &mut Ui) {
    let visuals = ui.visuals_mut();

    visuals.widgets.active.rounding = Rounding::ZERO;
    visuals.widgets.inactive.rounding = Rounding::ZERO;
    visuals.widgets.hovered.rounding = Rounding::ZERO;
    visuals.widgets.noninteractive.rounding = Rounding::ZERO;

    visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::GRAY);

    visuals.selection.stroke = Stroke::new(1.0, Color32::GRAY);
    visuals.selection.bg_fill = Color32::TRANSPARENT;

    ui.visuals_mut().selection = visuals.selection.clone();
}

fn draw_horizontal_line(ui: &mut Ui, rect: &eframe::egui::Rect) {
    ui.painter().line_segment(
        [
            Pos2::new(rect.left(), rect.bottom()),
            Pos2::new(rect.right(), rect.bottom()),
        ],
        Stroke::new(1.0, Color32::GRAY),
    );
}

fn draw_vertical_line(ui: &mut Ui, rect: &eframe::egui::Rect) {
    let line_x = rect.right();
    ui.painter().line_segment(
        [Pos2::new(line_x, rect.top()), Pos2::new(line_x, rect.bottom())],
        Stroke::new(1.0, Color32::WHITE),
    );
}

fn main() {
    let win_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(eframe::egui::Vec2::from([600.0, 800.0])),
        ..Default::default()
    };

    match run_native(
        "Setup script generator",
        win_options,
        Box::new(|_cc: &CreationContext| Box::new(SetupGenerator::new())),
    ) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
