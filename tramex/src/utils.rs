//! This module contains some utility functions used in the application.
use std::collections::BTreeSet;

use egui::{text::LayoutJob, Color32, TextFormat, Ui};
use tramex_tools::data::Trace;

/// Create an hyperlink open in a new tab
pub fn make_hyperlink(ui: &mut egui::Ui, label: &str, url: &str, new_tab: bool) {
    use egui::widgets::*; // to use ui();
    egui::Hyperlink::from_label_and_url(label, url)
        .open_in_new_tab(new_tab)
        .ui(ui);
}
/// change the BTreeSet according to the boolean value
pub fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

/// Create a label with a background color
pub fn color_label(job: &mut LayoutJob, ui: &Ui, label: &str, need_color: bool) {
    let default_color = if ui.visuals().dark_mode {
        Color32::LIGHT_GRAY
    } else {
        Color32::DARK_GRAY
    };
    let background_color = if need_color { Color32::DARK_BLUE } else { Color32::DARK_RED };
    job.append(
        label,
        0.0,
        TextFormat {
            color: default_color,
            background: background_color,
            ..Default::default()
        },
    );
}

/// Display a Trace type
pub fn display_log(ui: &mut Ui, log: &Trace) {
    let job = LayoutJob::default();
    ui.label(format!("{:?}", &log.trace_type));
    ui.label(format!("{:?}", &log.hexa));
    ui.label(job);
}
