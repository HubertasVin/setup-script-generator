use eframe::egui::{self, containers, Label};
use strum::IntoEnumIterator;

use crate::{
    question_type::{self, QuestionType},
    setup_generator::{SetupGenerator, Tab},
};

use super::questions_box::render_questions_box;

pub fn render_tab(setup_generator: &mut SetupGenerator, ui: &mut egui::Ui) {
    match setup_generator.current_tab {
        Tab::Debian => {
            render_questions_box(setup_generator, ui);
            render_add_section_button(setup_generator, ui);
        }
        Tab::Fedora => {
            ui.label("This is Tab Fedora");
        }
        Tab::Arch => {
            ui.label("This is Tab Arch");
        }
        Tab::Any => {
            ui.label("This is Tab Any");
        }
    }
}

fn render_add_section_button(setup_generator: &mut SetupGenerator, ui: &mut egui::Ui) {
    ui.add_space(5.0);

    show_combo_box(ui, &mut setup_generator.selected_new_question_type);

    if ui.button("Add question").clicked() {
        let new_question = match setup_generator.selected_new_question_type {
            QuestionType::Options { .. } => QuestionType::Options {
                title: "Choose a title".to_owned(),
                questions: Vec::new(),
            },
            QuestionType::ManualInputArray { .. } => QuestionType::ManualInputArray {
                title: "Choose a title".to_owned(),
                questions: Vec::new(),
            },
        };
        setup_generator.sections.push(new_question);
    }
}

fn show_combo_box(ui: &mut egui::Ui, current_value: &mut QuestionType) {
    egui::ComboBox::from_label("")
        .selected_text(format!("{:?}", current_value))
        .show_ui(ui, |ui| {
            for question_type in QuestionType::iter() {
                ui.selectable_value(
                    current_value,
                    question_type.clone(),
                    format!("{:?}", question_type),
                );
            }
        });
}
