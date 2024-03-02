use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(SystemInformationDiagnosticsPlugin);
        app.add_systems(Startup, setup_fps_display);
        app.add_systems(Update, (
            update_fps_display,
            toggle_fps_display_visibility,
        ));
    }
}

#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct LastCpuUsage(Option<f64>);

const MISSING_VALUE: &'static str = "N/a";
const FPS_INDEX: usize = 1;
const CPU_INDEX: usize = 3;
const MEM_INDEX: usize = 5;

const DEFAULT_COLOR: Color = Color::WHITE;

fn setup_fps_display(
    mut commands: Commands,
) {
    commands.spawn(LastCpuUsage(None));
    
    let fps_root = commands.spawn((
        FpsRoot,
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.0),
                top: Val::Percent(1.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        },
    )).id();

    let style = TextStyle {
        font_size: 16.0,
        color: DEFAULT_COLOR,
        ..default()
    };

    let fps_text = commands.spawn((
        FpsText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: style.clone(),
                },
                TextSection {
                    value: MISSING_VALUE.into(),
                    style: style.clone(),
                },
                TextSection {
                    value: "\nCPU: ".into(),
                    style: style.clone(),
                },
                TextSection {
                    value: MISSING_VALUE.into(),
                    style: style.clone(),
                },
                TextSection {
                    value: "\nMEM: ".into(),
                    style: style.clone(),
                },
                TextSection {
                    value: MISSING_VALUE.into(),
                    style: style.clone(),
                },
            ]),
            ..default()
        },
    )).id();

    commands.entity(fps_root).push_children(&[fps_text]);
}

fn update_fps_display(
    diagnostics: Res<DiagnosticsStore>,
    mut last_cpu_usage: Query<&mut LastCpuUsage>,
    mut text: Query<&mut Text, With<FpsText>>,
) {
    let mut last_cpu_usage = last_cpu_usage.single_mut();
    let mut text = text.single_mut();
    
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        text.sections[FPS_INDEX].value = format!("{fps:>4.0}");
        text.sections[FPS_INDEX].style.color = interpolate_color(fps as f32, 120.0, 60.0, 30.0);
    } else {
        text.sections[FPS_INDEX].value = MISSING_VALUE.into();
        text.sections[FPS_INDEX].style.color = DEFAULT_COLOR;
    }

    if let Some(cpu) = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
        .and_then(|cpu| cpu.value())
        .filter(|x| x.is_finite())
        .or(last_cpu_usage.0)
    {
        text.sections[CPU_INDEX].value = format!("{cpu:>4.0}%");
        text.sections[CPU_INDEX].style.color = interpolate_color(-cpu as f32, -30.0, -60.0, -100.0);
        last_cpu_usage.0 = Some(cpu);
    } else {
        text.sections[CPU_INDEX].value = MISSING_VALUE.into();
        text.sections[CPU_INDEX].style.color = DEFAULT_COLOR;
    }

    if let Some(mem) = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
        .and_then(|mem| mem.value())
    {
        text.sections[MEM_INDEX].value = format!("{mem:>4.1}%");
        text.sections[MEM_INDEX].style.color = interpolate_color(-mem as f32, -30.0, -60.0, -100.0);
    } else {
        text.sections[MEM_INDEX].value = MISSING_VALUE.into();
        text.sections[MEM_INDEX].style.color = DEFAULT_COLOR;
    }
}

fn interpolate_color(
    value: f32,
    g_threshold: f32,
    y_threshold: f32,
    r_threshold: f32,
) -> Color {
    if value >= g_threshold {
        Color::rgb(0.0, 1.0, 0.0)
    } else if value >= y_threshold {
        Color::rgb(
            (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
            1.0,
            0.0,
        )
    } else if value >= r_threshold {
        Color::rgb(
            1.0,
            ((value - 30.0) / (60.0 - 30.0)) as f32,
            0.0,
        )
    } else {
        Color::rgb(1.0, 0.0, 0.0)
    }
}

fn toggle_fps_display_visibility(
    mut visibility: Query<&mut Visibility, With<FpsRoot>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F12) {
        let mut visibility = visibility.single_mut();
        *visibility = match *visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}