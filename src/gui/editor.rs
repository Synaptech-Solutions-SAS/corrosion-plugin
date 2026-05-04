use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

use crate::params::CorrosionParams;

fn list_factory_presets() -> Vec<(String, String)> {
    let mut presets = vec![
        ("Bass".to_string(), "bass_cannon".to_string()),
        ("Bass".to_string(), "bass_depth_charge".to_string()),
        ("Bass".to_string(), "bass_low_rider".to_string()),
        ("Bass".to_string(), "bass_subterranean".to_string()),
        ("Boom".to_string(), "boom_deep_boom".to_string()),
        ("Boom".to_string(), "boom_low_thud".to_string()),
        ("Boom".to_string(), "boom_resonant".to_string()),
        ("Boom".to_string(), "boom_sub_kick".to_string()),
        ("Clang".to_string(), "clang_alloy_hit".to_string()),
        ("Clang".to_string(), "clang_iron_clang".to_string()),
        ("Clang".to_string(), "clang_metal_strike".to_string()),
        ("Clang".to_string(), "clang_steel_impact".to_string()),
        ("Long".to_string(), "long_ambient_hit".to_string()),
        ("Long".to_string(), "long_eternal_ring".to_string()),
        ("Long".to_string(), "long_long_decay".to_string()),
        ("Long".to_string(), "long_sustained_tone".to_string()),
        ("Short".to_string(), "short_quick_hit".to_string()),
        ("Short".to_string(), "short_rim_shot".to_string()),
        ("Short".to_string(), "short_short_strike".to_string()),
        ("Short".to_string(), "short_tight_snap".to_string()),
        ("Scrape".to_string(), "scrape_bowed_steel".to_string()),
        ("Scrape".to_string(), "scrape_brake_squeal".to_string()),
        ("Scrape".to_string(), "scrape_metal".to_string()),
        ("Scrape".to_string(), "scrape_tension_rise".to_string()),
        ("Chain".to_string(), "chain_gang".to_string()),
        ("Chain".to_string(), "chain_rattle".to_string()),
        ("Chain".to_string(), "industrial_chain".to_string()),
        ("Chain".to_string(), "link_clank".to_string()),
        ("Drone".to_string(), "deep_hum".to_string()),
        ("Drone".to_string(), "drone_pipe".to_string()),
        ("Drone".to_string(), "eternal_ring".to_string()),
        ("Drone".to_string(), "void_resonance".to_string()),
    ];
    presets.sort_by(|a, b| a.1.cmp(&b.1));
    presets
}

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
                        ui.separator();
                        ui.add_space(8.0);
                        
                        render_preset_browser_section(ui);
                        
                        ui.add_space(24.0);
                    });
                });
            });
        },
    )
}

fn render_preset_browser_section(ui: &mut egui::Ui) {
    ui.heading("Presets");
    ui.label("Factory presets by category");
    ui.add_space(12.0);
    
    let presets = list_factory_presets();
    let categories = ["All", "Bass", "Boom", "Clang", "Chain", "Scrape", "Drone", "Long", "Short"];
    
    ui.horizontal_wrapped(|ui| {
        for cat in &categories {
            let _ = ui.selectable_label(false, *cat);
        }
    });
    
    ui.add_space(8.0);
    
    egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
        ui.vertical(|ui| {
            for (category, name) in presets.iter().take(20) {
                ui.horizontal(|ui| {
                    ui.label(format!("[{}]", category));
                    if ui.button(name.replace('_', " ")).clicked() {
                    }
                });
            }
            if presets.len() > 20 {
                ui.label(format!("... and {} more", presets.len() - 20));
            }
        });
    });
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

    ui.add_space(12.0);
    ui.label("Modal energy distribution:");
    render_modal_visualization(ui, params.object.value());
}

fn render_modal_visualization(ui: &mut egui::Ui, object_value: i32) {
    let mode_count = match object_value {
        0 => 6,
        1 => 8,
        2 => 8,
        _ => 10,
    };

    let bar_width = 12.0;
    let spacing = 4.0;
    let total_width = mode_count as f32 * (bar_width + spacing);

    ui.allocate_ui_with_layout(
        egui::vec2(total_width, 60.0),
        egui::Layout::left_to_right(egui::Align::BOTTOM),
        |ui| {
            for i in 0..mode_count {
                let height_pct = match object_value {
                    0 => {
                        let base = 0.4 + 0.6 * (1.0 - (i as f32 / mode_count as f32));
                        base * (1.0 + 0.2 * (i as f32).sin())
                    }
                    1 => {
                        let base = 0.3 + 0.5 * (1.0 - (i as f32 / mode_count as f32).powi(2));
                        base * (1.0 + 0.3 * ((i * 2) as f32).sin())
                    }
                    2 => {
                        let base = 0.5 + 0.4 * (1.0 - (i as f32 / mode_count as f32));
                        base * (1.0 + 0.15 * (i as f32).cos())
                    }
                    _ => {
                        let base = 0.25 + 0.5 * (1.0 - (i as f32 / mode_count as f32));
                        base * (1.0 + 0.4 * ((i * 3) as f32).sin())
                    }
                }
                .clamp(0.1, 1.0);

                let height = 60.0 * height_pct;
                let color = egui::Color32::from_rgb(
                    (180.0 + 40.0 * height_pct) as u8,
                    (100.0 + 60.0 * height_pct) as u8,
                    (80.0 + 30.0 * height_pct) as u8,
                );

                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(bar_width, height),
                    egui::Sense::hover(),
                );

                ui.painter().rect_filled(rect, 2.0, color);
            }
        },
    );
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
