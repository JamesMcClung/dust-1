use bevy::prelude::*;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

// from https://bevy-cheatbook.github.io/cookbook/print-framerate.html

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Startup, setup_fps_counter);
        app.add_systems(Update, (
            update_fps_display,
            fps_counter_showhide,
        ));
    }
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

const MISSING_VALUE: &'static str = " N/a"; // the space is for alignment
const FPS_INDEX: usize = 1;

const DEFAULT_COLOR: Color = Color::WHITE;

fn setup_fps_counter(
    mut commands: Commands,
) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands.spawn((
        FpsRoot,
        NodeBundle {
            // give it a dark background for readability
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            // make it "always on top" by setting the Z index to maximum
            // we want it to be displayed over all other UI
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                // position it at the top-right corner
                // 1% away from the top window edge
                right: Val::Percent(1.),
                top: Val::Percent(1.),
                // set bottom/left to Auto, so it can be
                // automatically sized depending on the text
                bottom: Val::Auto,
                left: Val::Auto,
                // give it some padding for readability
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )).id();

    let style = TextStyle {
        font_size: 16.0,
        color: DEFAULT_COLOR,
        ..default()
    };

    let text_fps = commands.spawn((
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
            ]),
            ..Default::default()
        },
    )).id();
    commands.entity(root).push_children(&[text_fps]);
}

fn update_fps_display(
    diagnostics: Res<DiagnosticsStore>,
    mut text: Query<&mut Text, With<FpsText>>,
) {
    let mut text = text.single_mut();
    
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        text.sections[FPS_INDEX].value = format!("{value:>4.0}");
        text.sections[FPS_INDEX].style.color = interpolate_color(value as f32, 120.0, 60.0, 30.0);
    } else {
        text.sections[FPS_INDEX].value = MISSING_VALUE.into();
        text.sections[FPS_INDEX].style.color = DEFAULT_COLOR;
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

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}