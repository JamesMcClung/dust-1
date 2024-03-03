use bevy::prelude::*;

use crate::sim::liquid::LiquidProperties;
use crate::sim::{gas::GasProperties, Particle, particle};
use crate::color;

pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_palette, setup_particle_to_draw, select_initial_particle).chain())
            .add_systems(Update, (handle_buttons, update_palette).chain())
        ;
    }
}

#[derive(Component)]
struct PaletteRoot;

#[derive(Component)]
struct PaletteTitle;

#[derive(Component)]
struct PaletteDetails;

#[derive(Component)]
pub struct ParticleToDraw(pub Option<Particle>);

const INITIAL_PARTICLE_TO_DRAW: &'static str = particle::names::AIR;

fn get_style() -> TextStyle {
    TextStyle {
        font_size: 16.0,
        color: Color::WHITE,
        ..default()
    }
}

fn get_section(text: impl Into<String>) -> TextSection {
    TextSection {
        value: text.into(),
        style: get_style(),
    }
}

fn setup_palette(mut commands: Commands) {
    commands.spawn((
        PaletteRoot,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(5.0),
                bottom: Val::Percent(30.0),
                width: Val::Px(300.0),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            PaletteTitle,
            TextBundle {
                text: Text::from_sections([
                    get_section("DRAWING\n"),
                    get_section("particle name goes here"),
                ]),
                ..default()
            }
        ));
        parent.spawn((
            PaletteDetails,
            TextBundle {
                text: Text::from_section("particle details go here", get_style()),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            }
        ));
        parent.spawn(
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                    min_width: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            }
        ).with_children(|grid| {
            grid.spawn(TextBundle {
                text: Text::from_section("ELEMENTS", get_style()),
                style: Style { grid_column: GridPlacement::span(4), ..default() },
                ..default()
            });
            
            let elements = [
                Particle::Vacuum,
                Particle::Air { gas_properties: default() },
                Particle::Water { liquid_properties: default() },
            ];

            for element in elements {
                grid.spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        style: Style {
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        ..default()
                    },
                    element,
                )).with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(element.name(), TextStyle { color: get_text_color(&element), ..get_style() }),
                        ..default()
                    });
                });
            }
        });
    });
}

fn handle_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Particle),
        (Changed<Interaction>, With<Button>),
    >,
    mut particle_to_draw: Query<&mut ParticleToDraw>,
) {
    for (interaction, mut background_color, particle) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                background_color.0 = Color::DARK_GRAY;
            },
            Interaction::Hovered => {
                background_color.0 = Color::GRAY;
            },
            Interaction::Pressed => {
                particle_to_draw.single_mut().0 = Some(*particle);
            }
        }
    }
}

fn setup_particle_to_draw(mut commands: Commands) {
    commands.spawn(ParticleToDraw(None));
}

fn select_initial_particle(
    mut particle_to_draw: Query<&mut ParticleToDraw>,
    mut buttons: Query<&Particle, With<Button>>,
) {
    for particle in &mut buttons {
        if particle.name() == INITIAL_PARTICLE_TO_DRAW {
            particle_to_draw.single_mut().0 = Some(*particle);
            return;
        }
    }
}

fn update_palette(
    particle_to_draw: Query<&ParticleToDraw, Changed<ParticleToDraw>>,
    mut buttons: Query<(&mut BorderColor, &Particle), With<Button>>,
    mut palette_title: Query<&mut Text, (With<PaletteTitle>, Without<PaletteDetails>)>,
    mut palette_details: Query<&mut Text, With<PaletteDetails>>,
) {
    let Ok(ParticleToDraw(Some(particle_to_draw))) = particle_to_draw.get_single() else {
        return;
    };

    for (mut border_color, particle) in &mut buttons {
        border_color.0 = if particle.name() == particle_to_draw.name() {
            Color::GRAY
        } else {
            Color::DARK_GRAY
        };
    }

    let mut palette_title = palette_title.single_mut();
    let mut palette_details = palette_details.single_mut();

    palette_title.sections[1].value = particle_to_draw.name().into();
    palette_title.sections[1].style.color = get_text_color(particle_to_draw);
    palette_details.sections[0].value = get_details(particle_to_draw);
}

fn get_text_color(particle: &Particle) -> Color {
    color::get_color(particle).with_a(1.0)
}

fn get_details(particle: &Particle) -> String {
    match particle {
        Particle::Vacuum => "".into(),
        Particle::Air { gas_properties } => gas_property_details(gas_properties),
        Particle::Water { liquid_properties } => liquid_property_details(liquid_properties),
    }
}

fn gas_property_details(gas_properties: &GasProperties) -> String {
    let mass = gas_properties.mass;
    let velocity = gas_properties.velocity();
    let vx = velocity.x;
    let vy = velocity.y;
    let temperature = gas_properties.temperature();
    format!("\
* Gas Properties
  - mass:        {mass:5.1} kg
  - velocity:    ({vx:4.2}, {vy:4.2}) m/s
  - temperature: {temperature:5.1} K\
")
}

fn liquid_property_details(liquid_properties: &LiquidProperties) -> String {
    format!("\
* Liquid Properties\\
")
}
