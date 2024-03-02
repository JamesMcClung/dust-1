use bevy::prelude::*;

use crate::sim::{gas::GasProperties, Particle};

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
struct PaletteText;

#[derive(Component)]
pub struct ParticleToDraw(pub Particle);

const INITIAL_PARTICLE_TO_DRAW: Particle = Particle::Air { gas_properties: GasProperties::DEFAULT };

fn setup_palette(mut commands: Commands) {
    let palette_root = commands.spawn((
        PaletteRoot,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(75.0),
                top: Val::Percent(40.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        },
    )).id();

    fn get_section(text: impl Into<String>) -> TextSection {
        TextSection {
            value: text.into(),
            style: TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        }
    }

    let palette_text = commands.spawn((
        PaletteText,
        TextBundle {
            text: Text::from_sections([
                get_section("Drawing: "),
                get_section(INITIAL_PARTICLE_TO_DRAW.name()),
            ]),
            ..default()
        }
    )).id();

    commands.entity(palette_root).add_child(palette_text);
}

fn setup_particle_to_draw(mut commands: Commands) {
    commands.spawn(ParticleToDraw(INITIAL_PARTICLE_TO_DRAW));
}