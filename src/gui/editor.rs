use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

use crate::params::CorrosionParams;

pub fn create_editor(
    params: Arc<CorrosionParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Corrosion");
                        ui.label("Industrial Physical Modeling");
                        ui.separator();
                        
                        ui.add_space(16.0);
                        
                        render_macros_section(ui, &params, setter);
                        
                        ui.add_space(24.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        render_exciter_section(ui, &params, setter);
                        
                        ui.add_space(24.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        render_object_section(ui, &params, setter);
                        
                        ui.add_space(24.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        render_damage_section(ui, &params, setter);
                        
                        ui.add_space(24.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        render_space_section(ui, &params, setter);
                        
                        ui.add_space(24.0);
                    });
                });
            });
        },
    )
}

fn render_macros_section(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
) {
    ui.heading("Macros");
    ui.label("High-level sound shaping");
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.label("Mass:");
        ui.add(widgets::ParamSlider::for_param(&params.mass, setter));
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Corrosion:");
        ui.add(widgets::ParamSlider::for_param(&params.corrosion, setter));
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Violence:");
        ui.add(widgets::ParamSlider::for_param(&params.violence, setter));
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Damage:");
        ui.add(widgets::ParamSlider::for_param(&params.damage_macro, setter));
    });
}

fn render_exciter_section(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
) {
    ui.heading("Exciter");
    ui.label("How the metal is activated");
    ui.add_space(12.0);
    
    ui.horizontal(|ui| {
        ui.label("Type:");
        ui.add(widgets::ParamSlider::for_param(&params.exciter, setter));
    });
}

fn render_object_section(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
) {
    ui.heading("Object");
    ui.label("The resonating body");
    ui.add_space(12.0);
    
    ui.horizontal(|ui| {
        ui.label("Material:");
        ui.add(widgets::ParamSlider::for_param(&params.object, setter));
    });
    
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Size:");
        ui.add(widgets::ParamSlider::for_param(&params.size, setter));
    });
}

fn render_damage_section(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
) {
    ui.heading("Damage");
    ui.label("Wear and deterioration");
    ui.add_space(12.0);
    
    ui.horizontal(|ui| {
        ui.label("Rust:");
        ui.add(widgets::ParamSlider::for_param(&params.rust, setter));
    });
    
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Damage:");
        ui.add(widgets::ParamSlider::for_param(&params.damage, setter));
    });
}

fn render_space_section(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
) {
    ui.heading("Space");
    ui.label("Output shaping");
    ui.add_space(12.0);
    
    ui.horizontal(|ui| {
        ui.label("Drive:");
        ui.add(widgets::ParamSlider::for_param(&params.drive, setter));
    });
    
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(widgets::ParamSlider::for_param(&params.width, setter));
    });
    
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Body:");
        ui.add(widgets::ParamSlider::for_param(&params.body, setter));
    });
    
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Output:");
        ui.add(widgets::ParamSlider::for_param(&params.output, setter));
    });
}
