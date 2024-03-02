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
struct PaletteText;

#[derive(Component)]
pub struct ParticleToDraw(pub Particle);

const INITIAL_PARTICLE_TO_DRAW: Particle = Particle::Air { gas_properties: GasProperties::DEFAULT };

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

fn setup_palette(mut commands: Commands) {
    commands.spawn((
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
    )).with_children(|parent| {
        parent.spawn((
            PaletteText,
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
    });
}

fn setup_particle_to_draw(mut commands: Commands) {
    commands.spawn(ParticleToDraw(INITIAL_PARTICLE_TO_DRAW));
}