use egui;

use crate::Data;




pub fn guide_popup(data:&mut Data, egui_ctx: &egui::Context) {
    let mut guide =  include_str!("guide.md");
    egui::Window::new("Guide")
    .vscroll(true)
    .open(&mut data.guide)
    .show(egui_ctx, |ui| {
        for line in guide.split("\n").collect::<Vec<&str>>() {
            match line.chars().next() {
                Some('#') => {
                    ui.heading(line.replace("#", ""));
                },
                Some('`') => {
                    
                    ui.monospace(line);
                },
                Some('/') => {
                    // italic
                    ui.label(egui::RichText::new(line.replace("/", "")).italics());
                },
                Some('[') => {
                    ui.group(|ui| {
                        ui.monospace(line.replace("[", ""));
                    });
                },
                _ => {
                    ui.label(line);
                }
                
            }
        }
    });
}