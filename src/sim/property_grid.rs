use bevy::prelude::*;

use crate::zero::Zero;
use super::{Coords, N_PIXELS};

#[derive(Component)]
pub struct PropertyGrid<T> {
    arr: Vec<Vec<T>>,
}

impl<T> PropertyGrid<T> {
    pub fn new(mut callback: impl FnMut(Coords) -> T) -> Self {
        let mut res = Self { arr: Vec::with_capacity(N_PIXELS.x) };
        for x in 0..N_PIXELS.x {
            let mut col = Vec::with_capacity(N_PIXELS.y);
            for y in 0..N_PIXELS.y {
                col.push(callback(Coords::new(x, y)));
            }
            res.arr.push(col);
        }
        res
    }
    
    pub fn get(&self, coords: Coords) -> &T {
        &self.arr[coords.x][coords.y]
    }

    pub fn get_mut(&mut self, coords: Coords) -> &mut T {
        &mut self.arr[coords.x][coords.y]
    }

    pub fn try_get(&self, coords: impl TryInto<Coords>) -> Option<&T> {
        let coords: Coords = coords.try_into().ok()?;
        self.arr.get(coords.x)?.get(coords.y)
    }

    pub fn try_get_mut(&mut self, coords: impl TryInto<Coords>) -> Option<&mut T> {
        let coords: Coords = coords.try_into().ok()?;
        self.arr.get_mut(coords.x)?.get_mut(coords.y)
    }

    pub fn swap(&mut self, coords: Coords, mut val: T) -> T {
        std::mem::swap(self.get_mut(coords), &mut val);
        val
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
        Self::new(|_| T::default())
    }
}

impl<T: Zero> Zero for PropertyGrid<T> {
    fn zero() -> Self {
        Self::new(|_| T::zero())
    }
}