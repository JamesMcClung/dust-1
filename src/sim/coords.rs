use bevy::prelude::Component;

use super::types::Vector;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RelCoords {
    pub x: isize,
    pub y: isize,
}

// Coord impls

impl Coords {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn to(self, upper: Coords) -> impl Iterator<Item = Coords> {
        CoordsRange::new(self, upper)
    }
}

impl From<(usize, usize)> for Coords {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl TryFrom<(isize, isize)> for Coords {
    type Error = ();

    fn try_from((x, y): (isize, isize)) -> Result<Self, Self::Error> {
        if x < 0 || y < 0 {
            Err(())
        } else {
            Ok(Self::new(x as usize, y as usize))
        }
    }
}

impl TryFrom<RelCoords> for Coords {
    type Error = ();
    
    fn try_from(value: RelCoords) -> Result<Self, Self::Error> {
        Self::try_from((value.x, value.y))
    }
}

impl std::ops::Sub<Coords> for Coords {
    type Output = RelCoords;

    fn sub(self, rhs: Coords) -> Self::Output {
        RelCoords::from(self) - RelCoords::from(rhs)
    }
}

// RelCoord impls

impl RelCoords {
    pub const ONE: Self = Self::new(1, 1);

    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn abs(mut self) -> Self {
        self.x = self.x.abs();
        self.y = self.y.abs();
        self
    }

    pub const fn as_tuple(self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl From<Coords> for RelCoords {
    fn from(value: Coords) -> Self {
        Self::new(value.x as isize, value.y as isize)
    }
}

impl From<Vector> for RelCoords {
    fn from(value: Vector) -> Self {
        Self::new(value.x.floor() as isize, value.y.floor() as isize)
    }
}

impl std::ops::Mul<RelCoords> for RelCoords {
    type Output = Self;

    fn mul(self, rhs: RelCoords) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Add<RelCoords> for RelCoords {
    type Output = Self;

    fn add(self, rhs: RelCoords) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for RelCoords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::MulAssign<RelCoords> for RelCoords {
    fn mul_assign(&mut self, rhs: RelCoords) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

// Coords/RelCoords

impl std::ops::Add<Coords> for RelCoords {
    type Output = RelCoords;

    fn add(self, rhs: Coords) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl std::ops::Add<RelCoords> for Coords {
    type Output = RelCoords;

    fn add(self, rhs: RelCoords) -> Self::Output {
        rhs + RelCoords::from(self)
    }
}

impl std::ops::Sub<RelCoords> for Coords {
    type Output = RelCoords;

    fn sub(self, rhs: RelCoords) -> Self::Output {
        RelCoords::from(self) - rhs
    }
}

// Vector/RelCoords

impl std::ops::Mul<Vector> for RelCoords {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(rhs.x * self.x as f32, rhs.y * self.y as f32)
    }
}

impl std::ops::Mul<RelCoords> for Vector {
    type Output = Vector;

    fn mul(self, rhs: RelCoords) -> Self::Output {
        Vector::new(self.x * rhs.x as f32, self.y * rhs.y as f32)
    }
}

impl std::ops::MulAssign<RelCoords> for Vector {
    fn mul_assign(&mut self, rhs: RelCoords) {
        self.x *= rhs.x as f32;
        self.y *= rhs.y as f32;
    }
}

impl std::ops::Add<Vector> for RelCoords {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(rhs.x + self.x as f32, rhs.y + self.y as f32)
    }
}

impl std::ops::Add<RelCoords> for Vector {
    type Output = Vector;

    fn add(self, rhs: RelCoords) -> Self::Output {
        Vector::new(self.x + rhs.x as f32, self.y + rhs.y as f32)
    }
}

impl From<RelCoords> for Vector {
    fn from(value: RelCoords) -> Self {
        Self::new(value.x as f32, value.y as f32)
    }
}

// i32/RelCoords

impl std::ops::Mul<isize> for RelCoords {
    type Output = RelCoords;
    
    fn mul(mut self, rhs: isize) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl std::ops::Mul<RelCoords> for isize {
    type Output = RelCoords;
    
    fn mul(self, mut rhs: RelCoords) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs
    }
}

// Coord Ranges

struct CoordsRange {
    lower: Coords,
    current: Option<Coords>,
    upper: Coords,
}

impl CoordsRange {
    pub fn new(lower: Coords, upper: Coords) -> Self {
        Self { lower, upper, current: Some(lower) }
    }
}

impl Iterator for CoordsRange {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(ref mut current) => {
                let res = current.clone();
                
                current.y += 1;
                if current.y >= self.upper.y {
                    current.y = self.lower.y;
                    current.x += 1;
                    if current.x >= self.upper.x {
                        self.current = None;
                    }
                }
                
                Some(res)
            }
        }
    }
}

// RelCoords Iteration

struct RelCoordsRange {
    lower: RelCoords,
    current: Option<RelCoords>,
    upper: RelCoords,
}

impl Iterator for RelCoordsRange {
    type Item = RelCoords;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(ref mut current) => {
                let res = current.clone();
                
                current.y += 1;
                if current.y >= self.upper.y {
                    current.y = self.lower.y;
                    current.x += 1;
                    if current.x >= self.upper.x {
                        self.current = None;
                    }
                }
                
                Some(res)
            }
        }
    }
}
