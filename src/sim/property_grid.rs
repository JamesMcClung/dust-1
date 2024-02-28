use bevy::prelude::*;

use super::N_PIXELS;

#[derive(Component)]
pub struct PropertyGrid<T> {
    arr: [[T; N_PIXELS.y]; N_PIXELS.x],
}

impl<T> PropertyGrid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.arr[x][y]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.arr[x][y]
    }
}

impl<T: Default> Default for PropertyGrid<T> {
    fn default() -> Self {
        Self {
            arr: std::array::from_fn(|_| std::array::from_fn(|_| T::default())),
        }
    }
}
