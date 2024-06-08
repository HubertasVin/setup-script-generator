use eframe::egui::{
    self, Color32, Pos2, Rounding, Stroke, Ui,
};

use crate::{question_type::QuestionType, setup_generator::SetupGenerator};

pub fn render_questions_box(setup_generator: &mut SetupGenerator, ui: &mut Ui) {
    let original_visuals = ui.visuals().clone();

    for i in 0..setup_generator.sections.len() {
        let window_width = ui.available_width();
        match &mut setup_generator.sections[i] {
            QuestionType::Options(options) => {
                // Use a wrapping layout for the main axis
                let grid_layout = egui::Grid::new(format!("options_grid_{}", i)).show(ui, |ui| {
                    let checkbox_width = 20.0;
                    let text_edit_width = 150.0;
                    let gap_width = 10.0;
                    let option_width = checkbox_width + text_edit_width + gap_width;
                    let x_capacity = (window_width / option_width).floor() as usize;
                    let mut index = 0;

                    for _ in 0..options.len() {
                        ui.horizontal(|ui| {
                            let option = &mut options[index];
                            // Add the checkbox
                            ui.add_sized(
                                [checkbox_width, 20.0],
                                egui::Checkbox::new(&mut option.is_checked, ""),
                            );

                            customize_visuals(&mut ui.visuals_mut());

                            // Add the text edit field
                            ui.add_sized(
                                [text_edit_width, 20.0],
                                egui::TextEdit::singleline(&mut option.value),
                            );

                            *ui.visuals_mut() = original_visuals.clone();

                            // Ensure there's space between elements
                            ui.spacing_mut().item_spacing.x = gap_width;
                            index += 1;
                        });
                        if x_capacity != 0 && index % x_capacity == 0 {
                            ui.end_row();
                        }
                    }
                });
                draw_horizontal_line(ui, &grid_layout.response.rect);
            }
            QuestionType::ManualInputArray(values) => {
                for value in values {
                    ui.label(value.clone());
                }
            }
        }
    }
}

fn customize_visuals(visuals: &mut eframe::egui::Visuals) {
    visuals.widgets.active.rounding = Rounding::ZERO;
    visuals.widgets.inactive.rounding = Rounding::ZERO;
    visuals.widgets.hovered.rounding = Rounding::ZERO;
    visuals.widgets.noninteractive.rounding = Rounding::ZERO;

    visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    visuals.widgets.active.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.extreme_bg_color = Color32::TRANSPARENT;

    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::GRAY);

    visuals.selection.stroke = Stroke::new(1.0, Color32::GRAY);
    visuals.selection.bg_fill = Color32::TRANSPARENT;
}

fn draw_horizontal_line(ui: &mut Ui, rect: &eframe::egui::Rect) {
    ui.painter().line_segment(
        [
            Pos2::new(rect.left(), rect.bottom()),
            Pos2::new(ui.available_width(), rect.bottom()),
        ],
        Stroke::new(1.0, Color32::GRAY),
    );
}
