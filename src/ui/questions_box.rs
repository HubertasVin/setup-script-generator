use eframe::egui::{self, Color32, Pos2, Rounding, Stroke, Ui, Visuals};

use crate::{
    question_type::{OptionType, QuestionType},
    setup_generator::SetupGenerator,
};

const ITEM_WIDTH: f32 = 20.0;
const ITEM_HEIGHT: f32 = 20.0;
const OPTION_WIDTH: f32 = (ITEM_WIDTH * 2.0) + OPTION_TEXT_EDIT_WIDTH + (GAP_WIDTH * 2.0);
const OPTION_TEXT_EDIT_WIDTH: f32 = 150.0;
const GAP_WIDTH: f32 = 10.0;

pub fn render_questions_box(setup_generator: &mut SetupGenerator, ui: &mut Ui) {
    let title_edit_width: f32 = ui.available_width() - 10.0;
    let original_visuals = ui.visuals().clone();

    ui.add_space(5.0);

    for i in 0..setup_generator.sections.len() {
        let window_width = ui.available_width();
        match &mut setup_generator.sections[i] {
            QuestionType::Options {
                title,
                questions: options,
            } => {
                let x_capacity = (window_width / OPTION_WIDTH).floor() as usize;

                let mut to_remove: Vec<usize> = Vec::new();
                let mut add_new_option = false;

                customize_visuals(ui.visuals_mut());
                ui.add_sized(
                    [title_edit_width, ITEM_HEIGHT],
                    egui::TextEdit::singleline(title),
                );
                *ui.visuals_mut() = original_visuals.clone();

                let grid_layout = egui::Grid::new(format!("options_grid_{}", i)).show(ui, |ui| {
                    for (index, option) in options.iter_mut().enumerate() {
                        render_option(ui, option, &original_visuals, &mut to_remove, index);

                        if x_capacity != 0 && (index + 1) % x_capacity == 0 {
                            ui.end_row();
                        }
                    }

                    if options.len() % x_capacity != 0 {
                        ui.end_row();
                    }

                    if ui
                        .add_sized([ITEM_WIDTH, ITEM_HEIGHT], egui::Button::new("➕"))
                        .clicked()
                    {
                        add_new_option = true;
                    }
                });

                for index in to_remove.iter().rev() {
                    options.remove(*index);
                }

                if add_new_option {
                    options.push(OptionType::new("".to_string(), false));
                }
                ui.add_space(5.0);
                draw_horizontal_line(ui, &grid_layout.response.rect);
            }
            QuestionType::ManualInputArray {
                title,
                questions: values,
            } => {
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
            Pos2::new(rect.left(), rect.bottom() + 6.0),
            Pos2::new(ui.available_width(), rect.bottom() + 6.0),
        ],
        Stroke::new(1.0, Color32::GRAY),
    );
}

fn render_option(
    ui: &mut Ui,
    option: &mut OptionType,
    original_visuals: &Visuals,
    to_remove: &mut Vec<usize>,
    index: usize,
) {
    ui.horizontal(|ui| {
        if ui
            .add_sized([ITEM_WIDTH, ITEM_HEIGHT], egui::Button::new("🗑"))
            .clicked()
        {
            to_remove.push(index);
        };

        ui.spacing_mut().item_spacing.x = GAP_WIDTH;
        ui.add_sized(
            [ITEM_WIDTH, ITEM_HEIGHT],
            egui::Checkbox::new(&mut option.is_checked, ""),
        );

        customize_visuals(&mut ui.visuals_mut());
        ui.add_sized(
            [OPTION_TEXT_EDIT_WIDTH, ITEM_HEIGHT],
            egui::TextEdit::singleline(&mut option.value).hint_text("package-name"),
        );
        ui.spacing_mut().item_spacing.x = GAP_WIDTH;

        *ui.visuals_mut() = original_visuals.clone();
    });
}
