use eframe::egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Tab1,
    Tab2,
    Tab3,
}

pub struct SetupGenerator {
    pub sections: Vec<String>,
    pub initialized: bool,
    pub current_tab: Tab,
}

impl SetupGenerator {
    pub fn new() -> Self {
        let iter = (0..50).map(|i| format!("Text {}", i));
        SetupGenerator {
            sections: Vec::from_iter(iter),
            initialized: false,
            current_tab: Tab::Tab1,
        }
    }

    pub fn initialize(&mut self, _ctx: &eframe::egui::Context) {
        self.initialized = true; // Set initialized to true after setup
    }

    pub fn configure_fonts(&self, ctx: &eframe::egui::Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_owned(),
            FontData::from_static(include_bytes!("../fonts/MesloLGS NF Regular.ttf")),
        );

        (0..=4).for_each(|i| {
            font_def
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(i, "MesloLGS".to_owned());
        });

        ctx.set_fonts(font_def);

        let mut style = (*ctx.style()).clone();
        style
            .text_styles
            .insert(TextStyle::Body, FontId::new(20.0, FontFamily::Proportional));
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(35.0, FontFamily::Proportional),
        );

        ctx.set_style(style);
    }
}
