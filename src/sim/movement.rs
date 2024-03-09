use bevy::prelude::*;

use crate::schedule::SimSet;
use crate::sim::{Coords, Particle, PropertyGrid, RelCoords};
use crate::sim::path;
use crate::sim::types::Vector;
use crate::sim::dir::{Steps, Dir};


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, liquid_bulk_flow.in_set(SimSet::Liquid))
        ;
    }
}

fn liquid_bulk_flow(mut particles: Query<&mut PropertyGrid<Particle>>) {
    let mut particles = particles.single_mut();
    let mut moving_particles_next = PropertyGrid::new(|_| MovingParticle::None);
    let mut moving_coords_next = Vec::<Coords>::new();

    // 1. Lift particles that will move to a different cell
    for coords in particles.coords() {
        let particle = particles.get_mut(coords);
        if !matches!(particle, Particle::Water { .. }) {
            continue;
        }
        if let Some(physical_properties) = particle.get_physical_properties_mut() {
            let new_pos = physical_properties.internal_position + physical_properties.velocity();
            if is_in_cell(&new_pos) {
                physical_properties.internal_position = new_pos;
                continue;
            } 
            
            physical_properties.internal_position = new_pos.fract();

            let steps = path::get_path_deltas(physical_properties.internal_position, new_pos)
                .into_iter()
                .map(Dir::from)
                .collect::<Vec<_>>();

            *moving_particles_next.get_mut(coords) = MovingParticle::Some((steps.into(), std::mem::replace(particle, Particle::Vacuum)));
            moving_coords_next.push(coords);
        }
    }

    // 2. Push each lifted particle 1 cell at a time
    for i in 0.. {
        let moving_coords_this = std::mem::replace(&mut moving_coords_next, Vec::new());
        let mut moving_particles_this = std::mem::replace(&mut moving_particles_next, PropertyGrid::new(|_| MovingParticle::None));

        // Only stop when no moving coords remain
        if moving_coords_this.is_empty() {
            break;
        }

        let mut conflict_coords = Vec::<Coords>::new();

        let mut move_into = |next_coords: Coords, coords: Coords, steps: Steps, particle: Particle| {
            match moving_particles_next.get_mut(next_coords) {

                // If nothing has tried to move into the spot, well, now something has
                free_space @ MovingParticle::None => {
                    *free_space = MovingParticle::Some((steps, particle));
                    moving_coords_next.push(next_coords);
                },

                // If another particle already tried to move into the spot, start a conflict
                conflict @ MovingParticle::Some(_) => {
                    let (conflict_steps, conflict_particle) = conflict.to_conflict();
                    conflict.push_to_conflict(if next_coords == coords { Dir::Zero } else { steps[i] }, particle);
                    conflict.push_to_conflict(conflict_steps[i], conflict_particle);
                    conflict_coords.push(next_coords);
                },

                // If there's already a conflict, just add to it
                MovingParticle::Conflict(v) => v.push((if next_coords == coords { Dir::Zero } else { steps[i] }, particle)),
            }
        };

        // 2a. Move each lifted particle to the next cell
        for coords in moving_coords_this {
            if let MovingParticle::Some((mut steps, mut particle)) = moving_particles_this.swap(coords, MovingParticle::None) {

                // If lifted particle has no steps left, put it down
                if steps.len() <= i {
                    *particles.get_mut(coords) = particle;
                    continue;
                }
                
                let next_coords = coords + steps[i].get();
                match particles.try_get_mut(next_coords) {

                    // If unlifted particle is vacuum, try to move into it
                    Some(Particle::Vacuum) => {
                        move_into(next_coords.try_into().unwrap(), coords, steps, particle);
                    },

                    // If unlifted particle is not vacuum, hit it and don't move
                    Some(obstacle) => {
                        particle.collide(obstacle, steps[i].get().into());
                        steps[i] = Dir::Zero;
                        move_into(coords, coords, steps, particle);
                    },
                    
                    // If unlifted particle would go over the edge of the grid, stop moving
                    None => {
                        particle.get_physical_properties_mut().unwrap().momentum *= RelCoords::ONE - steps[i].get().abs(); // zero out the bad momentum
                        steps[i] = Dir::Zero;
                        move_into(coords, coords, steps, particle);
                    },
                }
            }
        }

        // 2b. Resolve conflicts
        while let Some(coords) = conflict_coords.pop() {
            let MovingParticle::Conflict(mut v) = moving_particles_next.swap(coords, MovingParticle::None) else { 
                dbg!(coords);
                panic!();
             };

            // Collide each pair of particles (Rust makes this difficult)
            let mut v2 = Vec::with_capacity(v.len());
            while let Some((dir, mut particle)) = v.pop() {
                for (other_dir, p) in v.iter_mut() {
                    particle.collide(p, (dir.get() - other_dir.get()).into());
                }
                v2.push((dir, particle));
            }
            
            // Determine where particles go
            for (dir, mut particle) in v2 {
                match dir {

                    // If the particle started here, plop it down in the real grid
                    Dir::Zero => *particles.get_mut(coords) = particle,

                    // If the particle came from somewhere else, send it back
                    dir => {
                        let prev_coords = Coords::try_from(coords - dir.get()).unwrap();

                        // If another particle tried to move into its old spot, start a conflict
                        if let conflict @ MovingParticle::Some(_) = moving_particles_next.get_mut(prev_coords) {
                            let (conflict_steps, mut conflict_particle) = conflict.to_conflict();
                            particle.collide(&mut conflict_particle, dir.get().into());
                            conflict.push_to_conflict(conflict_steps[i], conflict_particle);
                            conflict_coords.push(prev_coords);
                        }

                        // If more than one particle tried to move into its old spot, they are already going to
                        // hit each other and send each other back, so nothing to handle in this case.
                        
                        // No matter what, it will end up at its old spot, so do that
                        if matches!(particles.get(prev_coords), Particle::Water {..}) {
                            dbg!(prev_coords);
                            dbg!(coords);
                            panic!();
                        }
                        *particles.get_mut(prev_coords) = particle;
                    }
                }
            }
        }
    }
}

enum MovingParticle {
    None,
    Some((Steps, Particle)),
    Conflict(Vec<(Dir, Particle)>),
}

impl MovingParticle {
    pub fn to_conflict(&mut self) -> (Steps, Particle) {
        match self {
            Self::None | Self::Conflict(..) => panic!(),
            Self::Some((steps, particle)) => {
                let steps = std::mem::replace(steps, Steps::from(Vec::with_capacity(0)));
                let particle = std::mem::replace(particle, Particle::Vacuum);
                *self = Self::Conflict(vec![]);
                (steps, particle)
            },
        }
    }
    pub fn push_to_conflict(&mut self, dir: Dir, particle: Particle) {
        match self {
            Self::None | Self::Some(_) => panic!(),
            Self::Conflict(v) => v.push((dir, particle)),
        }
    }
}

fn is_in_cell(internal_position: &Vector) -> bool {
    0.0 <= internal_position.x && internal_position.x < 1.0 && 0.0 <= internal_position.y && internal_position.y < 1.0
}