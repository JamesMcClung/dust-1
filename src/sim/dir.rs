use super::RelCoords;

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
    Zero,
}

#[derive(Clone)]
pub struct Steps(Vec<Dir>);

impl Dir {
    pub fn get(&self) -> RelCoords {
        match self {
            Self::Up => RelCoords::new(0, 1),
            Self::Down => RelCoords::new(0, -1),
            Self::Left => RelCoords::new(-1, 0),
            Self::Right => RelCoords::new(1, 0),
            Self::Zero => RelCoords::new(0, 0),
        }
    }
}

impl From<RelCoords> for Dir {
    fn from(value: RelCoords) -> Self {
        match (value.x, value.y) {
            (0, 1) => Self::Up,
            (0, -1) => Self::Down,
            (1, 0) => Self::Right,
            (-1, 0) => Self::Left,
            _ => panic!(),
        }
    }
}

impl std::ops::Index<usize> for Steps {
    type Output = Dir;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Steps {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<Dir>> for Steps {
    fn from(value: Vec<Dir>) -> Self {
        Self(value)
    }
}

impl Steps {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}