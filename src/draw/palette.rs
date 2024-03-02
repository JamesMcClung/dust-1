use bevy::prelude::*;

use crate::sim::{gas::GasProperties, Particle};
use crate::color::get_color;

pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_palette, setup_particle_to_draw));
    }
}

#[derive(Component)]
struct PaletteRoot;

#[derive(Component)]
struct PaletteTitle;

#[derive(Component)]
struct PaletteDetails;

#[derive(Component)]
pub struct ParticleToDraw(pub Particle);

const INITIAL_PARTICLE_TO_DRAW: Particle = Particle::Air { gas_properties: GasProperties::DEFAULT };

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
                left: Val::Percent(75.0),
                top: Val::Percent(40.0),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            PaletteTitle,
            TextBundle {
                text: Text::from_sections([
                    get_section("Drawing: "),
                    {
                        let mut sec = get_section(INITIAL_PARTICLE_TO_DRAW.name());
                        sec.style.color = get_color(&INITIAL_PARTICLE_TO_DRAW);
                        sec
                    },
                ]),
                ..default()
            }
        ));
        parent.spawn((
            PaletteDetails,
            TextBundle {
                text: Text::from_section(get_details(&INITIAL_PARTICLE_TO_DRAW), get_style()),
                ..default()
            }
        ));
    });
}

fn setup_particle_to_draw(mut commands: Commands) {
    commands.spawn(ParticleToDraw(INITIAL_PARTICLE_TO_DRAW));
}

fn get_details(particle: &Particle) -> String {
    match particle {
        Particle::Vacuum => "".into(),
        Particle::Air { gas_properties } => gas_property_details(gas_properties),
    }
}

fn gas_property_details(gas_properties: &GasProperties) -> String {
    let mass = gas_properties.mass;
    let velocity = gas_properties.velocity();
    let vx = velocity.x;
    let vy = velocity.y;
    let temperature = gas_properties.temperature();
    format!("\
GAS PROPERTIES
  mass:        {mass:5.1} kg
  velocity:    ({vx:4.2}, {vy:4.2}) m/s
  temperature: {temperature:5.1} K\
")
}
