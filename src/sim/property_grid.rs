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

    pub fn get_checked(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            None
        } else {
            self.arr.get(x as usize)?.get(y as usize)
        }
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
