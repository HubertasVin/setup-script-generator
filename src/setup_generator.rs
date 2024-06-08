use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, TextStyle};

use crate::question_type::{OptionType, QuestionType};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Tab1,
    Tab2,
    Tab3,
}

pub struct SetupGenerator {
    pub sections: Vec<QuestionType>,
    pub initialized: bool,
    pub current_tab: Tab,
    pub window_dimensions: egui::Vec2,
}

impl SetupGenerator {
    /// Creates a new instance of SetupGenerator.
    pub fn new(window_dimensions: egui::Vec2) -> Self {
        let mut sections = vec![];
        sections.push(QuestionType::Options(vec![
            OptionType::new("Option 1".to_string(), false),
            OptionType::new("Option 2".to_string(), false),
            OptionType::new("Option 3".to_string(), false),
            OptionType::new("Option 3".to_string(), false),
            OptionType::new("Option 3".to_string(), false),
            OptionType::new("Option 3".to_string(), false),
            OptionType::new("Option 3".to_string(), false),
        ]));


        SetupGenerator {
            sections,
            initialized: false,
            current_tab: Tab::Tab1,
            window_dimensions: window_dimensions,
        }
    }

    /// Initializes the SetupGenerator, setting it as initialized.
    pub fn initialize(&mut self, _ctx: &eframe::egui::Context) {
        self.initialized = true; // Set initialized to true after setup
    }
}

/// Configures custom fonts for the application.
pub fn configure_fonts(ctx: &eframe::egui::Context) {
    // Load custom font data
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "MesloLGS".to_owned(),
        FontData::from_static(include_bytes!("../fonts/MesloLGS NF Regular.ttf")),
    );

    // Set custom font family for each font size
    for i in 0..=4 {
        font_def
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(i, "MesloLGS".to_owned());
    }

    ctx.set_fonts(font_def);

    // Set custom font style and size for body text
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(25.0, FontFamily::Proportional),
    );

    ctx.set_style(style);
}

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;
    use TextStyle::*;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (heading2(), FontId::new(25.0, Proportional)),
        (heading3(), FontId::new(23.0, Proportional)),
        (Body, FontId::new(15.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
