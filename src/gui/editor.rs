use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::Arc;

use crate::params::{CorrosionParams, ExciterFamily, ExciterType, Object};

const BASE_EDITOR_WIDTH: u32 = 1440;
const BASE_EDITOR_HEIGHT: u32 = 1024;

struct EditorUiState {
    last_ui_scale: i32,
}

pub fn create_editor(
    params: Arc<CorrosionParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    let initial_scale = params.ui_scale.value();
    create_egui_editor(
        editor_state,
        EditorUiState {
            last_ui_scale: initial_scale,
        },
        |_, _| {},
        move |egui_ctx, setter, state| {
            let ui_scale = params.ui_scale.value();
            let scale = ui_scale_factor(ui_scale);
            if state.last_ui_scale != ui_scale {
                state.last_ui_scale = ui_scale;
                let (width, height) = scaled_editor_size(scale);
                egui_ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    width as f32,
                    height as f32,
                )));
            }

            let mut style = (*egui_ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(10.0 * scale, 8.0 * scale);
            style.spacing.window_margin = egui::Margin::same((16.0 * scale) as i8);
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::proportional(22.0 * scale),
            );
            style.text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(14.0 * scale),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(14.0 * scale),
            );
            egui_ctx.set_style(style);
            egui_ctx.set_visuals(egui::Visuals::dark());

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    render_ui(ui, &params, setter);
                });
            });
        },
    )
}

fn ui_scale_factor(value: i32) -> f32 {
    match value {
        0 => 0.50,
        1 => 0.75,
        2 => 1.00,
        3 => 1.25,
        4 => 1.50,
        _ => 1.00,
    }
}

fn scaled_editor_size(scale: f32) -> (u32, u32) {
    (
        (BASE_EDITOR_WIDTH as f32 * scale).round() as u32,
        (BASE_EDITOR_HEIGHT as f32 * scale).round() as u32,
    )
}

fn render_ui(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Corrosion");
    ui.label("Simple control surface. All changes are applied directly to plugin parameters.");
    ui.separator();

    render_global(ui, params, setter);
    ui.separator();

    ui.columns(3, |columns| {
        render_exciter(&mut columns[0], params, setter);
        render_resonator(&mut columns[1], params, setter);
        render_processing(&mut columns[2], params, setter);
    });
}

fn render_global(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.horizontal(|ui| {
        combo_i32(
            ui,
            "UI Scale",
            &params.ui_scale,
            &[(0, "50%"), (1, "75%"), (2, "100%"), (3, "125%"), (4, "150%")],
            setter,
        );
        slider(ui, "Output", &params.output, 0.0, 10.0, setter);
        slider(ui, "Width", &params.width, -2.0, 3.0, setter);
        slider(ui, "Body", &params.body, 0.0, 5.0, setter);
    });
}

fn render_exciter(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Exciter");

    let exciter = ExciterType::from_int(params.exciter.value());
    let exciter_items: Vec<(i32, &'static str)> = (0..=16)
        .map(|value| (value, ExciterType::from_int(value).name()))
        .collect();
    combo_i32(ui, "Model", &params.exciter, &exciter_items, setter);

    ui.collapsing("Model settings", |ui| {
        let (a, b, c) = exciter_labels(exciter);
        slider(ui, a, &params.exciter_pressure, 0.0, 1.0, setter);
        slider(ui, b, &params.exciter_speed, 0.0, 1.0, setter);
        slider(ui, c, &params.exciter_roughness, 0.0, 1.0, setter);
    });

    ui.collapsing("Envelope", |ui| match exciter.family() {
        ExciterFamily::Hit => {
            ui.label("AR envelope for strike force.");
            slider(ui, "Attack", &params.env_attack, 0.001, 2.0, setter);
            slider(ui, "Release", &params.env_release, 0.01, 5.0, setter);
        }
        ExciterFamily::Scrape => {
            ui.label("6-stage MSEG for continuous friction gestures.");
            slider(ui, "Onset", &params.mseg_onset, 0.001, 1.0, setter);
            slider(ui, "Attack", &params.mseg_attack, 0.001, 2.0, setter);
            slider(ui, "Hold", &params.mseg_hold, 0.0, 2.0, setter);
            slider(ui, "Decay", &params.mseg_decay, 0.01, 5.0, setter);
            slider(ui, "Sustain", &params.mseg_sustain, 0.0, 1.0, setter);
            slider(ui, "Release", &params.mseg_release, 0.01, 5.0, setter);
            combo_i32(
                ui,
                "Loop",
                &params.loop_mode,
                &[(0, "Off"), (1, "Forward"), (2, "Ping-Pong")],
                setter,
            );
            combo_i32(
                ui,
                "Loop Start",
                &params.loop_start_stage,
                &[(0, "0"), (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5")],
                setter,
            );
            combo_i32(
                ui,
                "Loop End",
                &params.loop_end_stage,
                &[(0, "0"), (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5")],
                setter,
            );
        }
        ExciterFamily::Specialty => {
            ui.label("Simple envelope for specialty sources.");
            slider(ui, "Attack", &params.env_attack, 0.001, 2.0, setter);
            slider(ui, "Decay", &params.env_decay, 0.01, 5.0, setter);
            slider(ui, "Sustain", &params.env_sustain, 0.0, 1.0, setter);
            slider(ui, "Release", &params.env_release, 0.01, 5.0, setter);
        }
    });

    ui.collapsing("Envelope modulation", |ui| {
        slider(ui, "Env Amount", &params.env_amount, 0.0, 1.0, setter);
        slider(ui, "Velocity To Peak", &params.velocity_to_peak, 0.0, 1.0, setter);
        slider(ui, "Global Time", &params.global_time_scale, 0.1, 10.0, setter);
        slider(ui, "Velocity To Level", &params.velocity_to_level, 0.0, 1.0, setter);
        slider(ui, "Velocity To Time", &params.velocity_to_time, 0.0, 1.0, setter);
        slider(ui, "Curve", &params.curve_tension, -1.0, 1.0, setter);
    });

    ui.collapsing("Interaction", |ui| {
        slider(ui, "Strike Position", &params.strike_position, 0.0, 1.0, setter);
        slider(ui, "Coupling", &params.coupling_stiffness, 0.0, 1.0, setter);
        slider(ui, "Position Wander", &params.position_wander, 0.0, 1.0, setter);
        slider(ui, "Position Envelope", &params.position_envelope, 0.0, 1.0, setter);
        slider(ui, "Fundamental Anchor", &params.fundamental_anchor, 0.0, 1.0, setter);
    });
}

fn render_resonator(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Resonator");

    let object = Object::from_int(params.object.value());
    let object_items: Vec<(i32, &'static str)> = (0..=8)
        .map(|value| (value, Object::from_int(value).name()))
        .collect();
    combo_i32(ui, "Model", &params.object, &object_items, setter);

    ui.collapsing("Model settings", |ui| {
        let (size, damping, brightness, thickness) = object_labels(object);
        slider(ui, size, &params.size, 0.05, 10.0, setter);
        slider(ui, damping, &params.res_damping, 0.0, 1.0, setter);
        slider(ui, brightness, &params.res_brightness, 0.0, 1.0, setter);
        slider(ui, thickness, &params.thickness, 0.0, 1.0, setter);
    });

    ui.collapsing("Damage and material", |ui| {
        slider(ui, "Rust", &params.rust, 0.0, 5.0, setter);
        slider(ui, "Damage", &params.damage, 0.0, 10.0, setter);
        slider(ui, "Heat", &params.heat, 0.0, 1.0, setter);
        slider(ui, "Sludge", &params.sludge, 0.0, 1.0, setter);
    });
}

fn render_processing(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Processing");

    ui.collapsing("Filter", |ui| {
        slider(ui, "Cutoff", &params.filter_cutoff, 20.0, 20_000.0, setter);
        slider(ui, "Resonance", &params.filter_resonance, 0.0, 1.0, setter);
        slider(ui, "Tolerance", &params.component_tolerance, 0.0, 1.0, setter);
    });

    ui.collapsing("Drive", |ui| {
        slider(ui, "Drive Amount", &params.drive_amount, 0.0, 5.0, setter);
        slider(ui, "Bias Starvation", &params.bias_starvation, 0.0, 1.0, setter);
        slider(ui, "Chaos", &params.chaos_depth, 0.0, 1.0, setter);
        slider(ui, "Legacy Drive", &params.drive, 0.0, 5.0, setter);
    });

    ui.collapsing("Body and spread", |ui| {
        slider(ui, "Chassis Material", &params.chassis_material, 0.0, 1.0, setter);
        slider(ui, "Chassis Volume", &params.chassis_volume, 0.0, 1.0, setter);
        slider(ui, "Spread", &params.spread_width, 0.0, 1.0, setter);
        slider(ui, "Listener Proximity", &params.listener_proximity, 0.0, 1.0, setter);
    });

    ui.collapsing("Space", |ui| {
        combo_i32(
            ui,
            "Mode",
            &params.space_mode,
            &[(0, "Off"), (1, "Factory"), (2, "Spring"), (3, "Echo")],
            setter,
        );
        slider(ui, "Amount", &params.space_amount, 0.0, 1.0, setter);
        match params.space_mode.value() {
            1 => {
                slider(ui, "Factory Size", &params.factory_size, 0.0, 1.0, setter);
                slider(ui, "Machinery Clutter", &params.machinery_clutter, 0.0, 1.0, setter);
                slider(ui, "Wall Impedance", &params.wall_impedance, 0.0, 1.0, setter);
            }
            2 => {
                slider(ui, "Spring Tension", &params.spring_tension, 0.0, 1.0, setter);
                slider(ui, "Wire Stiffness", &params.wire_stiffness, 0.0, 1.0, setter);
                slider(ui, "Spring Tank Size", &params.spring_tank_size, 0.0, 1.0, setter);
            }
            3 => {
                slider(ui, "Delay Time", &params.delay_time, 0.0, 1.0, setter);
                slider(ui, "Machinery Movement", &params.machinery_movement, 0.0, 1.0, setter);
                slider(ui, "High Frequency Damping", &params.high_frequency_damping, 0.0, 1.0, setter);
            }
            _ => {}
        }
    });

    ui.collapsing("Limiter", |ui| {
        slider(ui, "Analog Ceiling", &params.analog_ceiling, 0.5, 1.0, setter);
        slider(ui, "Diode Softness", &params.diode_softness, 0.0, 1.0, setter);
    });
}

fn slider(
    ui: &mut egui::Ui,
    label: &str,
    param: &FloatParam,
    min: f32,
    max: f32,
    setter: &ParamSetter,
) {
    let mut value = param.value();
    let response = ui.add(egui::Slider::new(&mut value, min..=max).text(label));
    if response.changed() {
        setter.set_parameter(param, value);
    }
}

fn combo_i32(
    ui: &mut egui::Ui,
    label: &str,
    param: &IntParam,
    items: &[(i32, &'static str)],
    setter: &ParamSetter,
) {
    let current = param.value();
    let selected = items
        .iter()
        .find(|(value, _)| *value == current)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown");

    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_salt(label)
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (value, name) in items {
                    if ui.selectable_label(current == *value, *name).clicked() {
                        setter.set_parameter(param, *value);
                    }
                }
            });
    });
}

fn exciter_labels(exciter: ExciterType) -> (&'static str, &'static str, &'static str) {
    match exciter {
        ExciterType::Hit => ("Level", "Speed", "Tone"),
        ExciterType::HandStrike => ("Hand Mass", "Palm Stiffness", "Skin Damping"),
        ExciterType::FeltMallet => ("Mallet Mass", "Hardness", "Soft Curve"),
        ExciterType::HardMallet => ("Mallet Mass", "Stiffness", "Damping"),
        ExciterType::Drumstick => ("Stick Mass", "Stiffness", "Rebound"),
        ExciterType::WireBrush => ("Wires", "Sweep", "Spread"),
        ExciterType::MetalPipe => ("Pipe Mass", "Stiffness", "Pitch"),
        ExciterType::MetalChain => ("Links", "Speed", "Rattle"),
        ExciterType::Scrape => ("Pressure", "Speed", "Roughness"),
        ExciterType::StiffPoint => ("Speed", "Pressure", "Chatter"),
        ExciterType::HeavyGrinding => ("Speed", "Pressure", "Grit"),
        ExciterType::CorrugatedDrag => ("Speed", "Spacing", "Depth"),
        ExciterType::TensionRise => ("Pull Speed", "Threshold", "Stochasticity"),
        ExciterType::PneumaticJet => ("Pressure", "Nozzle Width", "Chaos"),
        ExciterType::ElectromagneticHum => ("Mains Frequency", "Field", "Voltage Sag"),
        ExciterType::TensionSnap => ("Pull Distance", "Hook Stiffness", "Snap Force"),
        ExciterType::ParticleRain => ("Flow", "Particle Mass", "Mass Variance"),
    }
}

fn object_labels(object: Object) -> (&'static str, &'static str, &'static str, &'static str) {
    match object {
        Object::Pipe => ("Pipe Length", "Wall Loss", "Tube Ring", "Wall Thickness"),
        Object::Plate => ("Plate Size", "Edge Loss", "Metal Brightness", "Plate Thickness"),
        Object::Tank => ("Tank Volume", "Cavity Loss", "Shell Ring", "Wall Thickness"),
        Object::Chain => ("Link Mass", "Friction Decay", "Link Brightness", "Link Gauge"),
        Object::IBeam => ("Beam Mass", "Rigidity Damping", "Shear Brightness", "Beam Mass"),
        Object::TautCable => ("Cable Tension", "Tension Loss", "Braid Brightness", "Wire Gauge"),
        Object::CoilSpring => ("Coil Length", "Friction", "Spring Slosh", "Wire Gauge"),
        Object::SheetMetal => ("Sheet Size", "Edge Damping", "Sheet Brightness", "Thinness"),
        Object::IndustrialCog => ("Blade Radius", "Friction Decay", "Tooth Ring", "Blade Thickness"),
    }
}

#[cfg(test)]
mod tests {
    use super::{scaled_editor_size, ui_scale_factor};

    #[test]
    fn ui_scale_resizes_window_from_base_dimensions() {
        assert_eq!(scaled_editor_size(ui_scale_factor(0)), (720, 512));
        assert_eq!(scaled_editor_size(ui_scale_factor(1)), (1080, 768));
        assert_eq!(scaled_editor_size(ui_scale_factor(2)), (1440, 1024));
        assert_eq!(scaled_editor_size(ui_scale_factor(3)), (1800, 1280));
        assert_eq!(scaled_editor_size(ui_scale_factor(4)), (2160, 1536));
    }
}
