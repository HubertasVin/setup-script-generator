mod questions_box;
mod tabbar;

use crate::setup_generator::{configure_fonts, configure_text_styles};

use super::{SetupGenerator, Tab};
use eframe::{
    egui::{self, style::ScrollStyle, CentralPanel, ScrollArea, Vec2},
    App,
};
use questions_box::render_questions_box;

impl App for SetupGenerator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            configure_fonts(ctx);
            configure_text_styles(ctx);
            self.initialize(ctx);
        }
        //get frame width
        CentralPanel::default().show(ctx, |ui| {
            tabbar::render_tabbar(ui, &mut self.current_tab);
            let available_width = ui.available_width();
            scroll_area_style(ctx);
            ScrollArea::vertical().max_width(5.0).show(ui, |ui| {
                ui.set_min_size(Vec2::new(available_width, 0.0)); // Extend the ScrollArea to the full width
                match self.current_tab {
                    Tab::Tab1 => {
                        render_questions_box(self, ui);
                    }
                    Tab::Tab2 => {
                        ui.label("This is Tab 2");
                    }
                    Tab::Tab3 => {
                        ui.label("This is Tab 3");
                    }
                }
            });
        });
    }
}

fn scroll_area_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.scroll = ScrollStyle {
        floating: true,
        bar_width: 5.0,
        handle_min_length: 300.0,
        bar_inner_margin: 2.0,
        bar_outer_margin: 2.0,
        floating_width: 5.0,
        floating_allocated_width: 6.0,
        foreground_color: false,
        dormant_background_opacity: 0.0,
        dormant_handle_opacity: 0.0,
        active_background_opacity: 0.0,
        active_handle_opacity: 1.0,
        interact_background_opacity: 0.0,
        interact_handle_opacity: 2.0,
    };
    ctx.set_style(style);
}
