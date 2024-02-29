use bevy::prelude::*;

use super::types::{Scalar, Vector};
use super::{Coords, N_PIXELS};

#[derive(Component)]
pub struct PropertyGrid<T> {
    arr: Box<[[T; N_PIXELS.y]; N_PIXELS.x]>,
}

impl<T> PropertyGrid<T> {
    pub fn get(&self, coords: impl Into<Coords>) -> &T {
        let coords = coords.into();
        &self.arr[coords.x][coords.y]
    }

    pub fn get_mut(&mut self, coords: impl Into<Coords>) -> &mut T {
        let coords = coords.into();
        &mut self.arr[coords.x][coords.y]
    }

    pub fn try_get(&self, coords: impl TryInto<Coords>) -> Option<&T> {
        let coords: Coords = coords.try_into().ok()?;
        self.arr.get(coords.x)?.get(coords.y)
    }

    pub fn try_get_mut(&mut self, coords: impl TryInto<Coords>) -> Option<&mut T> {
        Some(self.get_mut(coords.try_into().ok()?))
    }

    pub fn dims(&self) -> Coords {
        Coords::new(self.arr.len(), self.arr[0].len())
    }

    pub fn coords(&self) -> impl Iterator<Item = Coords> {
        Coords::ZERO.to(self.dims())
    }
}

// Special impls

impl<T: Default> Default for PropertyGrid<T> {
    fn default() -> Self {
        Self {
            arr: Box::new(std::array::from_fn(|_| std::array::from_fn(|_| T::default()))),
        }
    }
}

impl PropertyGrid<Scalar> {
    pub fn zero() -> Self {
        Self {
            arr: Box::new([[0.0; N_PIXELS.y]; N_PIXELS.x])
        }
    }
}

impl PropertyGrid<Vector> {
    pub fn zero() -> Self {
        Self {
            arr: Box::new([[Vector::ZERO; N_PIXELS.y]; N_PIXELS.x])
        }
    }
}
