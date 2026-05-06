use nih_plug::params::persist::PersistentField;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, resizable_window::ResizableWindow, EguiState};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::params::{exciter_model_items, CorrosionParams, ExciterFamily, ExciterType, Object};
use crate::Preset;

/// Logical editor width for the 100% UI scale.
///
/// The first custom editor pass used 1440x1024 as its 100% anchor, which made
/// the 75% option the practical size for normal plugin use. The product
/// baseline is now that former 75% footprint, so every scale option is derived
/// from 1080x768 instead of keeping a separate "design size" and "window size".
const BASE_EDITOR_WIDTH: u32 = 1080;
/// Logical editor height for the 100% UI scale.
const BASE_EDITOR_HEIGHT: u32 = 768;

struct EditorUiState {
    last_ui_scale: i32,
    /// Last logical window size requested through egui's viewport command.
    ///
    /// This is separate from `last_ui_scale` because hosts can restore an old
    /// persisted editor size before the first frame. Tracking the requested
    /// size lets the editor correct that stale outer window even when the scale
    /// parameter itself did not change during this open editor session.
    last_requested_size: (u32, u32),
    factory_presets: Vec<FactoryPresetEntry>,
    selected_factory_preset: usize,
    last_preset_status: Option<String>,
}

#[derive(Clone)]
struct FactoryPresetEntry {
    name: String,
    path: PathBuf,
}

#[derive(Clone, Copy)]
enum ExciterControlId {
    HandMass,
    FleshStiffness,
    FleshDamping,
    MuteDecay,
    MalletMass,
    FeltSoftness,
    CoreHardness,
    CompressionCurve,
    MaterialStiffness,
    ImpactDamping,
    StickMass,
    TipStiffness,
    RestitutionBounciness,
    MicroBounceLimit,
    WireDensity,
    SpreadDuration,
    BrushWireStiffness,
    AmplitudeRandomization,
    PipeMass,
    MetalStiffness,
    PipePitch,
    PipeRingDecay,
    LinkCount,
    ChainMass,
    DropEnvelopeSpread,
    InternalRattle,
    RattleColor,
    BowPressure,
    BowSpeed,
    RosinGrip,
    SlipCurve,
    ScrapeSpeed,
    PointPressure,
    ChatterPitch,
    ChatterDamping,
    GrindSpeed,
    GrindPressure,
    SurfaceGrit,
    GritColor,
    DragSpeed,
    RidgeSpacing,
    RidgeDepth,
    DragExciterMass,
    PullSpeed,
    BreakThreshold,
    SlipStochasticity,
    CreakSharpness,
    AirPressure,
    NozzleWidth,
    TurbulenceChaos,
    MainsFrequency,
    CoilProximity,
    VoltageSag,
    PullDistance,
    HookStiffness,
    SnapForce,
    FlowRate,
    ParticleMass,
    MassVariance,
}

#[derive(Clone, Copy)]
enum ResonatorControlId {
    Size,
    Damping,
    Brightness,
    Thickness,
}

#[derive(Clone, Copy)]
struct ExciterControlSpec {
    id: ExciterControlId,
    label: &'static str,
    min: f32,
    max: f32,
}

#[derive(Clone, Copy)]
struct ResonatorControlSpec {
    id: ResonatorControlId,
    label: &'static str,
    min: f32,
    max: f32,
}

struct ExciterPanelSpec {
    title: &'static str,
    description: &'static str,
    controls: &'static [ExciterControlSpec],
}

struct ResonatorPanelSpec {
    title: &'static str,
    description: &'static str,
    controls: &'static [ResonatorControlSpec],
}

const HAND_STRIKE_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::HandMass,
        label: "Hand Mass",
        min: 0.4,
        max: 3.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::FleshStiffness,
        label: "Flesh Stiffness",
        min: 0.05,
        max: 0.8,
    },
    ExciterControlSpec {
        id: ExciterControlId::FleshDamping,
        label: "Flesh Damping",
        min: 0.3,
        max: 1.8,
    },
    ExciterControlSpec {
        id: ExciterControlId::MuteDecay,
        label: "Mute Decay",
        min: 0.85,
        max: 0.999,
    },
];
const FELT_MALLET_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::MalletMass,
        label: "Mallet Mass",
        min: 0.4,
        max: 3.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::FeltSoftness,
        label: "Felt Softness",
        min: 0.1,
        max: 1.3,
    },
    ExciterControlSpec {
        id: ExciterControlId::CoreHardness,
        label: "Core Hardness",
        min: 0.5,
        max: 4.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::CompressionCurve,
        label: "Compression Curve",
        min: 2.0,
        max: 5.0,
    },
];
const HARD_MALLET_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::MalletMass,
        label: "Mallet Mass",
        min: 0.4,
        max: 3.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::MaterialStiffness,
        label: "Material Stiffness",
        min: 0.5,
        max: 5.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::ImpactDamping,
        label: "Impact Damping",
        min: 0.1,
        max: 1.3,
    },
];
const DRUMSTICK_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::StickMass,
        label: "Stick Mass",
        min: 0.05,
        max: 1.25,
    },
    ExciterControlSpec {
        id: ExciterControlId::TipStiffness,
        label: "Tip Stiffness",
        min: 0.8,
        max: 6.8,
    },
    ExciterControlSpec {
        id: ExciterControlId::RestitutionBounciness,
        label: "Restitution Bounciness",
        min: 0.2,
        max: 0.9,
    },
    ExciterControlSpec {
        id: ExciterControlId::MicroBounceLimit,
        label: "Micro Bounce Limit",
        min: 2.0,
        max: 8.0,
    },
];
const WIRE_BRUSH_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::WireDensity,
        label: "Wire Density",
        min: 10.0,
        max: 130.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::SpreadDuration,
        label: "Spread Duration",
        min: 10.0,
        max: 250.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::BrushWireStiffness,
        label: "Wire Stiffness",
        min: 0.0,
        max: 1.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::AmplitudeRandomization,
        label: "Amplitude Randomization",
        min: 0.0,
        max: 1.0,
    },
];
const METAL_PIPE_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::PipeMass,
        label: "Pipe Mass",
        min: 0.4,
        max: 2.6,
    },
    ExciterControlSpec {
        id: ExciterControlId::MetalStiffness,
        label: "Metal Stiffness",
        min: 0.5,
        max: 5.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::PipePitch,
        label: "Pipe Pitch",
        min: 0.5,
        max: 2.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::PipeRingDecay,
        label: "Pipe Ring Decay",
        min: 0.96,
        max: 0.999,
    },
];
const METAL_CHAIN_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::LinkCount,
        label: "Link Count",
        min: 3.0,
        max: 15.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::ChainMass,
        label: "Chain Mass",
        min: 0.2,
        max: 1.4,
    },
    ExciterControlSpec {
        id: ExciterControlId::DropEnvelopeSpread,
        label: "Drop Envelope Spread",
        min: 40.0,
        max: 400.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::InternalRattle,
        label: "Internal Rattle",
        min: 0.0,
        max: 1.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::RattleColor,
        label: "Rattle Color",
        min: 0.0,
        max: 1.0,
    },
];
const BOW_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::BowPressure,
        label: "Bow Pressure",
        min: 0.2,
        max: 2.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::BowSpeed,
        label: "Bow Speed",
        min: 0.1,
        max: 2.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::RosinGrip,
        label: "Rosin Grip",
        min: 0.05,
        max: 1.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::SlipCurve,
        label: "Slip Curve",
        min: 0.05,
        max: 1.5,
    },
];
const STIFF_POINT_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::ScrapeSpeed,
        label: "Scrape Speed",
        min: 0.1,
        max: 2.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::PointPressure,
        label: "Point Pressure",
        min: 0.1,
        max: 1.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::ChatterPitch,
        label: "Chatter Pitch",
        min: 0.1,
        max: 1.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::ChatterDamping,
        label: "Chatter Damping",
        min: 0.1,
        max: 0.9,
    },
];
const HEAVY_GRINDING_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::GrindSpeed,
        label: "Grind Speed",
        min: 0.1,
        max: 2.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::GrindPressure,
        label: "Grind Pressure",
        min: 0.1,
        max: 1.9,
    },
    ExciterControlSpec {
        id: ExciterControlId::SurfaceGrit,
        label: "Surface Grit",
        min: 0.0,
        max: 1.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::GritColor,
        label: "Grit Color",
        min: 0.0,
        max: 1.0,
    },
];
const CORRUGATED_DRAG_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::DragSpeed,
        label: "Drag Speed",
        min: 0.1,
        max: 2.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::RidgeSpacing,
        label: "Ridge Spacing",
        min: 0.01,
        max: 0.2,
    },
    ExciterControlSpec {
        id: ExciterControlId::RidgeDepth,
        label: "Ridge Depth",
        min: 0.0,
        max: 2.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::DragExciterMass,
        label: "Exciter Mass",
        min: 0.2,
        max: 2.0,
    },
];
const TENSION_RISE_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::PullSpeed,
        label: "Pull Speed",
        min: 0.05,
        max: 1.55,
    },
    ExciterControlSpec {
        id: ExciterControlId::BreakThreshold,
        label: "Break Threshold",
        min: 0.1,
        max: 1.6,
    },
    ExciterControlSpec {
        id: ExciterControlId::SlipStochasticity,
        label: "Slip Stochasticity",
        min: 0.0,
        max: 1.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::CreakSharpness,
        label: "Creak Sharpness",
        min: 0.2,
        max: 1.4,
    },
];
const PNEUMATIC_JET_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::AirPressure,
        label: "Air Pressure",
        min: 0.1,
        max: 2.1,
    },
    ExciterControlSpec {
        id: ExciterControlId::NozzleWidth,
        label: "Nozzle Width",
        min: 0.1,
        max: 1.6,
    },
    ExciterControlSpec {
        id: ExciterControlId::TurbulenceChaos,
        label: "Turbulence Chaos",
        min: 0.0,
        max: 2.0,
    },
];
const ELECTROMAGNETIC_HUM_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::MainsFrequency,
        label: "Mains Frequency",
        min: 40.0,
        max: 120.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::CoilProximity,
        label: "Coil Proximity",
        min: 0.0,
        max: 2.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::VoltageSag,
        label: "Voltage Sag",
        min: 0.0,
        max: 2.0,
    },
];
const TENSION_SNAP_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::PullDistance,
        label: "Pull Distance",
        min: 0.1,
        max: 1.5,
    },
    ExciterControlSpec {
        id: ExciterControlId::HookStiffness,
        label: "Hook Stiffness",
        min: 0.2,
        max: 2.2,
    },
    ExciterControlSpec {
        id: ExciterControlId::SnapForce,
        label: "Snap Force",
        min: 0.1,
        max: 2.0,
    },
];
const PARTICLE_RAIN_CONTROLS: &[ExciterControlSpec] = &[
    ExciterControlSpec {
        id: ExciterControlId::FlowRate,
        label: "Flow Rate",
        min: 0.1,
        max: 3.1,
    },
    ExciterControlSpec {
        id: ExciterControlId::ParticleMass,
        label: "Particle Mass",
        min: 0.05,
        max: 1.0,
    },
    ExciterControlSpec {
        id: ExciterControlId::MassVariance,
        label: "Mass Variance",
        min: 0.0,
        max: 2.0,
    },
];

const PIPE_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Pipe Length",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Tube Diameter",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Sustain Time",
        min: 0.0,
        max: 1.0,
    },
];
const PLATE_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Plate Size",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Aspect Ratio",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Metal Stiffness",
        min: 0.0,
        max: 1.0,
    },
];
const TANK_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Tank Volume",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Thickness,
        label: "Wall Thickness",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Cavity Mix",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Shell Decay",
        min: 0.0,
        max: 1.0,
    },
];
const CHAIN_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Link Mass",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Thickness,
        label: "Chain Length",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Instability",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Friction Decay",
        min: 0.0,
        max: 1.0,
    },
];
const IBEAM_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Beam Mass",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Shear Density",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Rigidity Damping",
        min: 0.0,
        max: 1.0,
    },
];
const TAUT_CABLE_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Cable Tension",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Braid Stiffness",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Tension Drop",
        min: 0.0,
        max: 1.0,
    },
];
const COIL_SPRING_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Coil Length",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Dispersion Chirp",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Spring Slosh",
        min: 0.0,
        max: 1.0,
    },
];
const SHEET_METAL_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Sheet Size",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Thickness,
        label: "Metal Thinness",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Damping,
        label: "Edge Damping",
        min: 0.0,
        max: 1.0,
    },
];
const INDUSTRIAL_COG_RESONATOR_CONTROLS: &[ResonatorControlSpec] = &[
    ResonatorControlSpec {
        id: ResonatorControlId::Size,
        label: "Blade Radius",
        min: 0.05,
        max: 10.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Brightness,
        label: "Tooth Dissonance",
        min: 0.0,
        max: 1.0,
    },
    ResonatorControlSpec {
        id: ResonatorControlId::Thickness,
        label: "Blade Thickness",
        min: 0.0,
        max: 1.0,
    },
];

pub fn create_editor(
    params: Arc<CorrosionParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    let initial_scale = params.ui_scale.value();
    let initial_size = scaled_editor_size(ui_scale_factor(initial_scale));
    persist_editor_size(&editor_state, initial_size);
    let resize_state = editor_state.clone();
    create_egui_editor(
        editor_state,
        EditorUiState {
            last_ui_scale: initial_scale,
            last_requested_size: initial_size,
            factory_presets: load_factory_presets(),
            selected_factory_preset: 0,
            last_preset_status: None,
        },
        |_, _| {},
        move |egui_ctx, setter, state| {
            let ui_scale = params.ui_scale.value();
            let scale = ui_scale_factor(ui_scale);
            let desired_size = scaled_editor_size(scale);

            // NIH-plug's public resize hook for egui lives behind
            // `ResizableWindow`, whose drag handle can call the wrapper's
            // private `EguiState::set_requested_size()` queue. Programmatic UI
            // scale changes cannot call that private queue directly, so keep the
            // persisted `EguiState::size()` and egui viewport synchronized as a
            // best-effort scale command while the wrapper below provides the
            // official host-approved resize path.
            if state.last_ui_scale != ui_scale
                || state.last_requested_size != desired_size
                || viewport_size_mismatch(egui_ctx, desired_size)
            {
                state.last_ui_scale = ui_scale;
                state.last_requested_size = desired_size;
                persist_editor_size(&resize_state, desired_size);
                let (width, height) = desired_size;
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

            ResizableWindow::new("corrosion-editor-window")
                .min_size(egui::vec2(
                    (BASE_EDITOR_WIDTH as f32 * 0.5).round(),
                    (BASE_EDITOR_HEIGHT as f32 * 0.5).round(),
                ))
                .show(egui_ctx, &resize_state, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        render_ui(ui, &params, setter, state);
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

fn persist_editor_size(editor_state: &Arc<EguiState>, size: (u32, u32)) {
    let serialized = serde_json::json!({ "size": size });
    if let Ok(state) = serde_json::from_value::<EguiState>(serialized) {
        editor_state.set(state);
    }
}

fn viewport_size_mismatch(egui_ctx: &egui::Context, desired_size: (u32, u32)) -> bool {
    egui_ctx.input(|input| {
        input
            .viewport()
            .inner_rect
            .map(|rect| {
                let current_size = (rect.width().round() as u32, rect.height().round() as u32);
                !sizes_match(current_size, desired_size)
            })
            .unwrap_or(false)
    })
}

fn sizes_match(current_size: (u32, u32), desired_size: (u32, u32)) -> bool {
    current_size.0.abs_diff(desired_size.0) <= 1 && current_size.1.abs_diff(desired_size.1) <= 1
}

fn render_ui(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
    state: &mut EditorUiState,
) {
    ui.heading("Corrosion");
    ui.label("Selection-specific control surface. The editor only shows the controls that belong to the active exciter and resonator.");
    ui.separator();

    render_global(ui, params, setter, state);
    ui.separator();

    ui.columns(3, |columns| {
        render_exciter(&mut columns[0], params, setter);
        render_resonator(&mut columns[1], params, setter);
        render_processing(&mut columns[2], params, setter);
    });
}

fn render_global(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
    state: &mut EditorUiState,
) {
    render_preset_loader(ui, params, setter, state);

    ui.horizontal(|ui| {
        combo_i32(
            ui,
            "ui-scale",
            "UI Scale",
            &params.ui_scale,
            &[
                (0, "50%"),
                (1, "75%"),
                (2, "100%"),
                (3, "125%"),
                (4, "150%"),
            ],
            setter,
        );
        slider(ui, "Output", &params.output, 0.0, 10.0, setter);
        slider(ui, "Width", &params.width, -2.0, 3.0, setter);
        slider(ui, "Body", &params.body, 0.0, 5.0, setter);
    });
}

fn render_preset_loader(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
    state: &mut EditorUiState,
) {
    ui.horizontal(|ui| {
        ui.label("Preset");
        if state.factory_presets.is_empty() {
            ui.label("No factory presets found");
            return;
        }

        state.selected_factory_preset = state
            .selected_factory_preset
            .min(state.factory_presets.len().saturating_sub(1));
        let selected_name = state.factory_presets[state.selected_factory_preset]
            .name
            .as_str();

        egui::ComboBox::from_id_salt("factory-preset-loader")
            .selected_text(selected_name)
            .show_ui(ui, |ui| {
                for (index, preset) in state.factory_presets.iter().enumerate() {
                    if ui
                        .selectable_label(index == state.selected_factory_preset, &preset.name)
                        .clicked()
                    {
                        state.selected_factory_preset = index;
                    }
                }
            });

        if ui.button("Load").clicked() {
            let entry = state.factory_presets[state.selected_factory_preset].clone();
            match Preset::load(&entry.path) {
                Ok(preset) => {
                    apply_preset(params, setter, &preset);
                    state.last_preset_status = Some(format!("Loaded {}", preset.name));
                }
                Err(error) => {
                    state.last_preset_status =
                        Some(format!("Failed to load {}: {error}", entry.name));
                }
            }
        }
    });

    if let Some(status) = &state.last_preset_status {
        ui.label(status);
    }
}

fn apply_preset(params: &CorrosionParams, setter: &ParamSetter, preset: &Preset) {
    setter.set_parameter(&params.object, preset.object.to_int());
    setter.set_parameter(&params.exciter, preset.exciter);
    setter.set_parameter(&params.size, preset.size);
    setter.set_parameter(&params.rust, preset.rust);
    setter.set_parameter(&params.damage, preset.damage);
    setter.set_parameter(&params.drive, preset.drive);
    setter.set_parameter(&params.output, preset.output);
    setter.set_parameter(&params.width, preset.width);
    setter.set_parameter(&params.body, preset.body);
}

fn load_factory_presets() -> Vec<FactoryPresetEntry> {
    let preset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("presets/factory");
    let Ok(entries) = fs::read_dir(preset_dir) else {
        return Vec::new();
    };

    let mut presets: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension == "corrosion-preset")
        })
        .filter_map(|path| {
            Preset::load(&path).ok().map(|preset| FactoryPresetEntry {
                name: preset.name,
                path,
            })
        })
        .collect();

    presets.sort_by(|left, right| left.name.cmp(&right.name));
    presets
}

fn render_exciter(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Exciter");

    let exciter = ExciterType::from_int(params.exciter.value());
    combo_i32(
        ui,
        "exciter-model",
        "Model",
        &params.exciter,
        exciter_model_items(),
        setter,
    );

    let panel = exciter_panel(exciter);
    ui.collapsing(panel.title, |ui| {
        if !panel.description.is_empty() {
            ui.label(panel.description);
        }
        if !panel.controls.is_empty() {
            render_exciter_controls(ui, params, setter, panel.controls);
        }
    });

    ui.collapsing("Exciter envelope", |ui| match exciter.family() {
        ExciterFamily::Hit => {
            ui.label("One-shot force envelope for impact-style exciters.");
            slider(ui, "Attack", &params.env_attack, 0.001, 2.0, setter);
            slider(ui, "Release", &params.env_release, 0.01, 5.0, setter);
        }
        ExciterFamily::Friction => {
            ui.label("6-stage gesture envelope for continuous friction models.");
            slider(ui, "Onset", &params.mseg_onset, 0.001, 1.0, setter);
            slider(ui, "Attack", &params.mseg_attack, 0.001, 2.0, setter);
            slider(ui, "Hold", &params.mseg_hold, 0.0, 2.0, setter);
            slider(ui, "Decay", &params.mseg_decay, 0.01, 5.0, setter);
            slider(ui, "Sustain", &params.mseg_sustain, 0.0, 1.0, setter);
            slider(ui, "Release", &params.mseg_release, 0.01, 5.0, setter);
            combo_i32(
                ui,
                "loop-mode",
                "Loop",
                &params.loop_mode,
                &[(0, "Off"), (1, "Forward"), (2, "Ping-Pong")],
                setter,
            );
            combo_i32(
                ui,
                "loop-start-stage",
                "Loop Start",
                &params.loop_start_stage,
                &[(0, "0"), (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5")],
                setter,
            );
            combo_i32(
                ui,
                "loop-end-stage",
                "Loop End",
                &params.loop_end_stage,
                &[(0, "0"), (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5")],
                setter,
            );
        }
        ExciterFamily::Specialty => {
            ui.label("ADSR-style force shaping for continuous specialty sources.");
            slider(ui, "Attack", &params.env_attack, 0.001, 2.0, setter);
            slider(ui, "Decay", &params.env_decay, 0.01, 5.0, setter);
            slider(ui, "Sustain", &params.env_sustain, 0.0, 1.0, setter);
            slider(ui, "Release", &params.env_release, 0.01, 5.0, setter);
        }
    });

    ui.collapsing("Gesture modifiers", |ui| {
        slider(ui, "Env Amount", &params.env_amount, 0.0, 1.0, setter);
        slider(
            ui,
            "Velocity To Peak",
            &params.velocity_to_peak,
            0.0,
            1.0,
            setter,
        );
        slider(
            ui,
            "Global Time",
            &params.global_time_scale,
            0.1,
            10.0,
            setter,
        );
        slider(
            ui,
            "Velocity To Level",
            &params.velocity_to_level,
            0.0,
            1.0,
            setter,
        );
        slider(
            ui,
            "Velocity To Time",
            &params.velocity_to_time,
            0.0,
            1.0,
            setter,
        );
        slider(ui, "Curve", &params.curve_tension, -1.0, 1.0, setter);
    });
}

fn render_resonator(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Resonator");

    let object = Object::from_int(params.object.value());
    let object_items: Vec<(i32, &'static str)> = (0..=8)
        .map(|value| (value, Object::from_int(value).name()))
        .collect();
    combo_i32(
        ui,
        "resonator-model",
        "Model",
        &params.object,
        &object_items,
        setter,
    );

    let panel = resonator_panel(object);
    ui.collapsing(panel.title, |ui| {
        ui.label(panel.description);
        render_resonator_controls(ui, params, setter, panel.controls);
    });

    ui.collapsing("Material wear", |ui| {
        slider(ui, "Rust", &params.rust, 0.0, 5.0, setter);
        slider(ui, "Damage", &params.damage, 0.0, 10.0, setter);
        slider(ui, "Heat", &params.heat, 0.0, 1.0, setter);
        slider(ui, "Sludge", &params.sludge, 0.0, 1.0, setter);
    });

    ui.collapsing("Interaction bus", |ui| {
        slider(
            ui,
            "Strike Position",
            &params.strike_position,
            0.0,
            1.0,
            setter,
        );
        slider(ui, "Coupling", &params.coupling_stiffness, 0.0, 1.0, setter);
        slider(
            ui,
            "Position Wander",
            &params.position_wander,
            0.0,
            1.0,
            setter,
        );
        slider(
            ui,
            "Position Envelope",
            &params.position_envelope,
            0.0,
            1.0,
            setter,
        );
        slider(
            ui,
            "Fundamental Anchor",
            &params.fundamental_anchor,
            0.0,
            1.0,
            setter,
        );
    });
}

fn render_processing(ui: &mut egui::Ui, params: &CorrosionParams, setter: &ParamSetter) {
    ui.heading("Processing");

    ui.collapsing("Filter", |ui| {
        slider(ui, "Cutoff", &params.filter_cutoff, 20.0, 20_000.0, setter);
        slider(ui, "Resonance", &params.filter_resonance, 0.0, 1.0, setter);
        slider(
            ui,
            "Tolerance",
            &params.component_tolerance,
            0.0,
            1.0,
            setter,
        );
    });

    ui.collapsing("Drive", |ui| {
        slider(ui, "Drive Amount", &params.drive_amount, 0.0, 5.0, setter);
        slider(
            ui,
            "Bias Starvation",
            &params.bias_starvation,
            0.0,
            1.0,
            setter,
        );
        slider(ui, "Chaos", &params.chaos_depth, 0.0, 1.0, setter);
        slider(ui, "Legacy Drive", &params.drive, 0.0, 5.0, setter);
    });

    ui.collapsing("Body and spread", |ui| {
        slider(
            ui,
            "Chassis Material",
            &params.chassis_material,
            0.0,
            1.0,
            setter,
        );
        slider(
            ui,
            "Chassis Volume",
            &params.chassis_volume,
            0.0,
            1.0,
            setter,
        );
        slider(ui, "Spread", &params.spread_width, 0.0, 1.0, setter);
        slider(
            ui,
            "Listener Proximity",
            &params.listener_proximity,
            0.0,
            1.0,
            setter,
        );
    });

    ui.collapsing("Space", |ui| {
        combo_i32(
            ui,
            "space-mode",
            "Mode",
            &params.space_mode,
            &[(0, "Off"), (1, "Factory"), (2, "Spring"), (3, "Echo")],
            setter,
        );
        slider(ui, "Amount", &params.space_amount, 0.0, 1.0, setter);
        match params.space_mode.value() {
            1 => {
                slider(ui, "Factory Size", &params.factory_size, 0.0, 1.0, setter);
                slider(
                    ui,
                    "Machinery Clutter",
                    &params.machinery_clutter,
                    0.0,
                    1.0,
                    setter,
                );
                slider(
                    ui,
                    "Wall Impedance",
                    &params.wall_impedance,
                    0.0,
                    1.0,
                    setter,
                );
            }
            2 => {
                slider(
                    ui,
                    "Spring Tension",
                    &params.spring_tension,
                    0.0,
                    1.0,
                    setter,
                );
                slider(
                    ui,
                    "Wire Stiffness",
                    &params.wire_stiffness,
                    0.0,
                    1.0,
                    setter,
                );
                slider(
                    ui,
                    "Spring Tank Size",
                    &params.spring_tank_size,
                    0.0,
                    1.0,
                    setter,
                );
            }
            3 => {
                slider(ui, "Delay Time", &params.delay_time, 0.0, 1.0, setter);
                slider(
                    ui,
                    "Machinery Movement",
                    &params.machinery_movement,
                    0.0,
                    1.0,
                    setter,
                );
                slider(
                    ui,
                    "High Frequency Damping",
                    &params.high_frequency_damping,
                    0.0,
                    1.0,
                    setter,
                );
            }
            _ => {}
        }
    });

    ui.collapsing("Limiter", |ui| {
        slider(
            ui,
            "Analog Ceiling",
            &params.analog_ceiling,
            0.5,
            1.0,
            setter,
        );
        slider(
            ui,
            "Diode Softness",
            &params.diode_softness,
            0.0,
            1.0,
            setter,
        );
    });
}

fn exciter_panel(exciter: ExciterType) -> ExciterPanelSpec {
    match exciter {
        ExciterType::Bow => ExciterPanelSpec {
            title: "Bow",
            description: "Smooth stick-slip bow friction with pressure, speed, grip, and slip controls from the documented bow model.",
            controls: BOW_CONTROLS,
        },
        ExciterType::HandStrike => ExciterPanelSpec {
            title: "Hand strike physics",
            description: "Fleshy Kelvin-Voigt impact with heavy damping and an explicit mute tail.",
            controls: HAND_STRIKE_CONTROLS,
        },
        ExciterType::FeltMallet => ExciterPanelSpec {
            title: "Felt mallet contact",
            description: "Soft non-linear mallet with a compressive felt layer over a harder core.",
            controls: FELT_MALLET_CONTROLS,
        },
        ExciterType::HardMallet => ExciterPanelSpec {
            title: "Hard mallet contact",
            description: "Rigid Hertzian strike tuned around mass, brightness, and bounce suppression.",
            controls: HARD_MALLET_CONTROLS,
        },
        ExciterType::Drumstick => ExciterPanelSpec {
            title: "Drumstick bounce",
            description: "Light rigid impact with explicit rebound energy and micro-bounce count.",
            controls: DRUMSTICK_CONTROLS,
        },
        ExciterType::WireBrush => ExciterPanelSpec {
            title: "Wire brush cluster",
            description: "Impulse-cloud brush model with density, spread, stiffness, and stochastic variance.",
            controls: WIRE_BRUSH_CONTROLS,
        },
        ExciterType::MetalPipe => ExciterPanelSpec {
            title: "Metal pipe coupling",
            description: "Bi-directional pipe impact with a ringing exciter body feeding back into the collision.",
            controls: METAL_PIPE_CONTROLS,
        },
        ExciterType::MetalChain => ExciterPanelSpec {
            title: "Metal chain cascade",
            description: "Clustered heavy-link impacts with separate timing spread and high-frequency rattle coloration.",
            controls: METAL_CHAIN_CONTROLS,
        },
        ExciterType::StiffPoint => ExciterPanelSpec {
            title: "Stiff point chatter",
            description: "Rigid point scrape where chatter pitch and damping define the squeal character.",
            controls: STIFF_POINT_CONTROLS,
        },
        ExciterType::HeavyGrinding => ExciterPanelSpec {
            title: "Grinding friction",
            description: "Heavy rough drag with separate baseline force, tearing grit, and noise color.",
            controls: HEAVY_GRINDING_CONTROLS,
        },
        ExciterType::CorrugatedDrag => ExciterPanelSpec {
            title: "Corrugated drag topology",
            description: "Drag motion across ridges with explicit spacing, depth, and moving contact mass.",
            controls: CORRUGATED_DRAG_CONTROLS,
        },
        ExciterType::TensionRise => ExciterPanelSpec {
            title: "Tension-rise slip",
            description: "Integrate-and-fire creak model driven by pull speed, threshold, randomness, and crack sharpness.",
            controls: TENSION_RISE_CONTROLS,
        },
        ExciterType::PneumaticJet => ExciterPanelSpec {
            title: "Pneumatic jet drive",
            description: "Band-limited turbulence exciter shaped by nozzle focus and overload chaos.",
            controls: PNEUMATIC_JET_CONTROLS,
        },
        ExciterType::ElectromagneticHum => ExciterPanelSpec {
            title: "Electromagnetic drive",
            description: "Continuous Lorentz-force injection with direct control of mains pitch, proximity, and sag.",
            controls: ELECTROMAGNETIC_HUM_CONTROLS,
        },
        ExciterType::TensionSnap => ExciterPanelSpec {
            title: "Tension snap release",
            description: "Hook-and-release mechanism with explicit pull distance, stiffness, and break force.",
            controls: TENSION_SNAP_CONTROLS,
        },
        ExciterType::ParticleRain => ExciterPanelSpec {
            title: "Particle rain stream",
            description: "Continuous debris cloud controlled by flow density, particle weight, and variance.",
            controls: PARTICLE_RAIN_CONTROLS,
        },
    }
}

fn resonator_panel(object: Object) -> ResonatorPanelSpec {
    match object {
        Object::Pipe => ResonatorPanelSpec {
            title: "Pipe geometry",
            description: "Longitudinal pipe response with dedicated length, diameter, and sustain controls.",
            controls: PIPE_RESONATOR_CONTROLS,
        },
        Object::Plate => ResonatorPanelSpec {
            title: "Plate surface",
            description: "2D plate response balancing overall span, aspect ratio, and material stiffness.",
            controls: PLATE_RESONATOR_CONTROLS,
        },
        Object::Tank => ResonatorPanelSpec {
            title: "Tank cavity",
            description: "Shell-and-cavity behavior with volume, wall thickness, cavity balance, and shell decay.",
            controls: TANK_RESONATOR_CONTROLS,
        },
        Object::Chain => ResonatorPanelSpec {
            title: "Chain instability",
            description: "Linked resonator behavior shaped by link weight, chain length, instability, and friction loss.",
            controls: CHAIN_RESONATOR_CONTROLS,
        },
        Object::IBeam => ResonatorPanelSpec {
            title: "I-beam structure",
            description: "Beam-like modes with explicit mass, shear density, and rigidity damping emphasis.",
            controls: IBEAM_RESONATOR_CONTROLS,
        },
        Object::TautCable => ResonatorPanelSpec {
            title: "Cable tension",
            description: "Stiff string response with tension, braid stiffness, and dynamic pitch-drop control.",
            controls: TAUT_CABLE_RESONATOR_CONTROLS,
        },
        Object::CoilSpring => ResonatorPanelSpec {
            title: "Coil spring dispersion",
            description: "Dispersive spring response controlled by coil length, chirp density, and slosh instability.",
            controls: COIL_SPRING_RESONATOR_CONTROLS,
        },
        Object::SheetMetal => ResonatorPanelSpec {
            title: "Sheet metal buckling",
            description: "Thin-sheet response with size, thinness, and edge damping rather than generic brightness knobs.",
            controls: SHEET_METAL_RESONATOR_CONTROLS,
        },
        Object::IndustrialCog => ResonatorPanelSpec {
            title: "Cog and blade split",
            description: "Circular blade behavior with radius, tooth dissonance, and blade thickness control.",
            controls: INDUSTRIAL_COG_RESONATOR_CONTROLS,
        },
    }
}

fn render_exciter_controls(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
    controls: &[ExciterControlSpec],
) {
    for spec in controls {
        slider(
            ui,
            spec.label,
            exciter_param_ref(params, spec.id),
            spec.min,
            spec.max,
            setter,
        );
    }
}

fn render_resonator_controls(
    ui: &mut egui::Ui,
    params: &CorrosionParams,
    setter: &ParamSetter,
    controls: &[ResonatorControlSpec],
) {
    for spec in controls {
        slider(
            ui,
            spec.label,
            resonator_param_ref(params, spec.id),
            spec.min,
            spec.max,
            setter,
        );
    }
}

fn exciter_param_ref(params: &CorrosionParams, id: ExciterControlId) -> &FloatParam {
    match id {
        ExciterControlId::HandMass => &params.hand_mass,
        ExciterControlId::FleshStiffness => &params.flesh_stiffness,
        ExciterControlId::FleshDamping => &params.flesh_damping,
        ExciterControlId::MuteDecay => &params.mute_decay,
        ExciterControlId::MalletMass => &params.mallet_mass,
        ExciterControlId::FeltSoftness => &params.felt_softness,
        ExciterControlId::CoreHardness => &params.core_hardness,
        ExciterControlId::CompressionCurve => &params.compression_curve,
        ExciterControlId::MaterialStiffness => &params.material_stiffness,
        ExciterControlId::ImpactDamping => &params.impact_damping,
        ExciterControlId::StickMass => &params.stick_mass,
        ExciterControlId::TipStiffness => &params.tip_stiffness,
        ExciterControlId::RestitutionBounciness => &params.restitution_bounciness,
        ExciterControlId::MicroBounceLimit => &params.micro_bounce_limit,
        ExciterControlId::WireDensity => &params.wire_density,
        ExciterControlId::SpreadDuration => &params.spread_duration,
        ExciterControlId::BrushWireStiffness => &params.brush_wire_stiffness,
        ExciterControlId::AmplitudeRandomization => &params.amplitude_randomization,
        ExciterControlId::PipeMass => &params.pipe_mass,
        ExciterControlId::MetalStiffness => &params.metal_stiffness,
        ExciterControlId::PipePitch => &params.pipe_pitch,
        ExciterControlId::PipeRingDecay => &params.pipe_ring_decay,
        ExciterControlId::LinkCount => &params.link_count,
        ExciterControlId::ChainMass => &params.chain_mass,
        ExciterControlId::DropEnvelopeSpread => &params.drop_envelope_spread,
        ExciterControlId::InternalRattle => &params.internal_rattle,
        ExciterControlId::RattleColor => &params.rattle_color,
        ExciterControlId::BowPressure => &params.bow_pressure,
        ExciterControlId::BowSpeed => &params.bow_speed,
        ExciterControlId::RosinGrip => &params.rosin_grip,
        ExciterControlId::SlipCurve => &params.slip_curve,
        ExciterControlId::ScrapeSpeed => &params.scrape_speed,
        ExciterControlId::PointPressure => &params.point_pressure,
        ExciterControlId::ChatterPitch => &params.chatter_pitch,
        ExciterControlId::ChatterDamping => &params.chatter_damping,
        ExciterControlId::GrindSpeed => &params.grind_speed,
        ExciterControlId::GrindPressure => &params.grind_pressure,
        ExciterControlId::SurfaceGrit => &params.surface_grit,
        ExciterControlId::GritColor => &params.grit_color,
        ExciterControlId::DragSpeed => &params.drag_speed,
        ExciterControlId::RidgeSpacing => &params.ridge_spacing,
        ExciterControlId::RidgeDepth => &params.ridge_depth,
        ExciterControlId::DragExciterMass => &params.drag_exciter_mass,
        ExciterControlId::PullSpeed => &params.pull_speed,
        ExciterControlId::BreakThreshold => &params.break_threshold,
        ExciterControlId::SlipStochasticity => &params.slip_stochasticity,
        ExciterControlId::CreakSharpness => &params.creak_sharpness,
        ExciterControlId::AirPressure => &params.air_pressure,
        ExciterControlId::NozzleWidth => &params.nozzle_width,
        ExciterControlId::TurbulenceChaos => &params.turbulence_chaos,
        ExciterControlId::MainsFrequency => &params.mains_frequency,
        ExciterControlId::CoilProximity => &params.coil_proximity,
        ExciterControlId::VoltageSag => &params.voltage_sag,
        ExciterControlId::PullDistance => &params.pull_distance,
        ExciterControlId::HookStiffness => &params.hook_stiffness,
        ExciterControlId::SnapForce => &params.snap_force,
        ExciterControlId::FlowRate => &params.flow_rate,
        ExciterControlId::ParticleMass => &params.particle_mass,
        ExciterControlId::MassVariance => &params.mass_variance,
    }
}

fn resonator_param_ref(params: &CorrosionParams, id: ResonatorControlId) -> &FloatParam {
    match id {
        ResonatorControlId::Size => &params.size,
        ResonatorControlId::Damping => &params.res_damping,
        ResonatorControlId::Brightness => &params.res_brightness,
        ResonatorControlId::Thickness => &params.thickness,
    }
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
    id: &'static str,
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
        egui::ComboBox::from_id_salt(id)
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

#[cfg(test)]
mod tests {
    use super::{
        exciter_panel, load_factory_presets, persist_editor_size, resonator_panel,
        scaled_editor_size, sizes_match, ui_scale_factor,
    };
    use crate::params::{exciter_model_items, ExciterType, Object};
    use nih_plug_egui::EguiState;

    #[test]
    fn ui_scale_resizes_window_from_base_dimensions() {
        assert_eq!(scaled_editor_size(ui_scale_factor(0)), (540, 384));
        assert_eq!(scaled_editor_size(ui_scale_factor(1)), (810, 576));
        assert_eq!(scaled_editor_size(ui_scale_factor(2)), (1080, 768));
        assert_eq!(scaled_editor_size(ui_scale_factor(3)), (1350, 960));
        assert_eq!(scaled_editor_size(ui_scale_factor(4)), (1620, 1152));
    }

    #[test]
    fn viewport_resize_tolerates_rounding_noise_only() {
        assert!(sizes_match((1079, 768), (1080, 768)));
        assert!(sizes_match((1081, 769), (1080, 768)));
        assert!(!sizes_match((1440, 1024), (1080, 768)));
    }

    #[test]
    fn ui_scale_persists_host_facing_editor_size() {
        let editor_state = EguiState::from_size(1080, 768);

        persist_editor_size(&editor_state, scaled_editor_size(ui_scale_factor(1)));

        assert_eq!(editor_state.size(), (810, 576));
    }

    #[test]
    fn exciter_panel_resolves_algorithm_specific_control_counts() {
        assert_eq!(exciter_panel(ExciterType::HandStrike).controls.len(), 4);
        assert_eq!(exciter_panel(ExciterType::HardMallet).controls.len(), 3);
        assert_eq!(exciter_panel(ExciterType::MetalChain).controls.len(), 5);
        assert_eq!(exciter_panel(ExciterType::Bow).controls.len(), 4);
    }

    #[test]
    fn exciter_dropdown_items_are_spec_models_not_categories() {
        let names: Vec<_> = exciter_model_items()
            .iter()
            .map(|(_, name)| *name)
            .collect();

        assert_eq!(names[0], "Hand Strike");
        assert!(names.contains(&"Bow"));
        assert!(!names.contains(&"Hit"));
        assert!(!names.contains(&"Scrape"));
        assert!(!names.contains(&"Other"));
        for (value, name) in exciter_model_items() {
            assert_eq!(ExciterType::from_int(*value).name(), *name);
        }
    }

    #[test]
    fn resonator_panel_resolves_object_specific_control_counts() {
        assert_eq!(resonator_panel(Object::Pipe).controls.len(), 3);
        assert_eq!(resonator_panel(Object::Tank).controls.len(), 4);
        assert_eq!(resonator_panel(Object::IndustrialCog).controls.len(), 3);
    }

    #[test]
    fn factory_preset_loader_finds_factory_bank() {
        let presets = load_factory_presets();

        assert_eq!(presets.len(), 60);
        assert_eq!(presets[0].name, "Anchored Tank Moan");
        assert_eq!(presets[59].name, "Worn Chain Hail");
    }
}
