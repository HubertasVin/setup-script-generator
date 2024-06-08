use super::Tab;
use eframe::egui::{Color32, Pos2, Rounding, SelectableLabel, Stroke, Ui};

/// Renders the tab bar with selectable tabs and draws lines to separate them.
pub fn render_tabbar(ui: &mut Ui, current_tab: &mut Tab) {
    let button_width = ui.available_width() / 3.0;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0; // Set spacing between buttons

        // Save the current visuals
        let original_visuals = ui.visuals().clone();

        // Customize visuals for the tab bar
        customize_visuals(ui.visuals_mut());

        // Draw the SelectableLabels and vertical lines
        for (i, tab) in [Tab::Tab1, Tab::Tab2, Tab::Tab3].iter().enumerate() {
            let widget = ui.add_sized(
                [button_width, 20.0],
                SelectableLabel::new(*current_tab == *tab, format!("Tab {}", i + 1)),
            );

            if widget.clicked() {
                *current_tab = *tab;
            }

            // Draw a horizontal line at the bottom of the selected tab
            if *current_tab == *tab {
                draw_horizontal_line(ui, &widget.rect);
            }

            // Draw vertical lines between tabs
            if i < 2 {
                draw_vertical_line(ui, &widget.rect);
            }
        }

        // Restore the original visuals
        *ui.visuals_mut() = original_visuals;
    });
}

/// Customizes the visuals for the tab bar.
fn customize_visuals(visuals: &mut eframe::egui::Visuals) {
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
}

/// Draws a horizontal line at the bottom of the selected tab.
fn draw_horizontal_line(ui: &mut Ui, rect: &eframe::egui::Rect) {
    ui.painter().line_segment(
        [
            Pos2::new(rect.left(), rect.bottom()),
            Pos2::new(rect.right(), rect.bottom()),
        ],
        Stroke::new(1.0, Color32::GRAY),
    );
}

/// Draws a vertical line between tabs.
fn draw_vertical_line(ui: &mut Ui, rect: &eframe::egui::Rect) {
    ui.painter().line_segment(
        [
            Pos2::new(rect.right(), rect.top()),
            Pos2::new(rect.right(), rect.bottom()),
        ],
        Stroke::new(1.0, Color32::WHITE),
    );
}
