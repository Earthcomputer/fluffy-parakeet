use glam::IVec3;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize};
use std::mem;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    pub const HORIZONTAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    pub const VERTICAL: [Direction; 2] = [Direction::Up, Direction::Down];

    #[inline]
    pub fn axis(&self) -> Axis {
        match self {
            Direction::Down => Axis::Y,
            Direction::Up => Axis::Y,
            Direction::North => Axis::Z,
            Direction::South => Axis::Z,
            Direction::West => Axis::X,
            Direction::East => Axis::X,
        }
    }

    #[inline]
    pub fn axis_direction(&self) -> AxisDirection {
        match self {
            Direction::Down => AxisDirection::Negative,
            Direction::Up => AxisDirection::Positive,
            Direction::North => AxisDirection::Negative,
            Direction::South => AxisDirection::Positive,
            Direction::West => AxisDirection::Negative,
            Direction::East => AxisDirection::Positive,
        }
    }

    #[inline]
    pub fn from_axis_and_direction(axis: Axis, axis_direction: AxisDirection) -> Direction {
        match (axis, axis_direction) {
            (Axis::X, AxisDirection::Negative) => Direction::West,
            (Axis::X, AxisDirection::Positive) => Direction::East,
            (Axis::Y, AxisDirection::Negative) => Direction::Down,
            (Axis::Y, AxisDirection::Positive) => Direction::Up,
            (Axis::Z, AxisDirection::Negative) => Direction::North,
            (Axis::Z, AxisDirection::Positive) => Direction::South,
        }
    }

    #[inline]
    pub fn plane(&self) -> Plane {
        match self {
            Direction::Down => Plane::Vertical,
            Direction::Up => Plane::Vertical,
            Direction::North => Plane::Horizontal,
            Direction::South => Plane::Horizontal,
            Direction::West => Plane::Horizontal,
            Direction::East => Plane::Horizontal,
        }
    }

    #[inline]
    pub fn offset(&self) -> IVec3 {
        match self {
            Direction::Down => IVec3::NEG_Y,
            Direction::Up => IVec3::Y,
            Direction::North => IVec3::NEG_Z,
            Direction::South => IVec3::Z,
            Direction::West => IVec3::NEG_X,
            Direction::East => IVec3::X,
        }
    }

    #[inline]
    pub fn opposite(&self) -> Direction {
        let ordinal = *self as u8;
        let opposite_ordinal = ordinal ^ 1;
        // SAFETY: there are 6 directions, xoring with 1 will never escape the range 0-5
        unsafe { mem::transmute::<u8, Direction>(opposite_ordinal) }
    }

    #[inline]
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Down => Direction::Down,
            Direction::Up => Direction::Up,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    #[inline]
    pub fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Down => Direction::Down,
            Direction::Up => Direction::Up,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    pub fn deserialize_horizontal<'de, D>(deserializer: D) -> Result<Direction, D::Error>
    where
        D: Deserializer<'de>,
    {
        let direction = Direction::deserialize(deserializer)?;
        if direction.plane() != Plane::Horizontal {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("vertical direction"),
                &"a horizontal direction",
            ));
        }
        Ok(direction)
    }

    pub fn deserialize_vertical<'de, D>(deserializer: D) -> Result<Direction, D::Error>
    where
        D: Deserializer<'de>,
    {
        let direction = Direction::deserialize(deserializer)?;
        if direction.plane() != Plane::Vertical {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("horizontal direction"),
                &"a vertical direction",
            ));
        }
        Ok(direction)
    }
}

impl Add<Direction> for IVec3 {
    type Output = IVec3;

    fn add(self, rhs: Direction) -> IVec3 {
        self + rhs.offset()
    }
}

impl Add<IVec3> for Direction {
    type Output = IVec3;

    fn add(self, rhs: IVec3) -> IVec3 {
        self.offset() + rhs
    }
}

impl AddAssign<Direction> for IVec3 {
    fn add_assign(&mut self, rhs: Direction) {
        AddAssign::add_assign(self, rhs.offset())
    }
}

impl Sub<Direction> for IVec3 {
    type Output = IVec3;

    fn sub(self, rhs: Direction) -> Self::Output {
        self - rhs.offset()
    }
}

impl SubAssign<Direction> for IVec3 {
    fn sub_assign(&mut self, rhs: Direction) {
        SubAssign::sub_assign(self, rhs.offset())
    }
}

impl Mul<i32> for Direction {
    type Output = IVec3;

    fn mul(self, rhs: i32) -> IVec3 {
        self.offset() * rhs
    }
}

impl Mul<Direction> for i32 {
    type Output = IVec3;

    fn mul(self, rhs: Direction) -> IVec3 {
        self * rhs.offset()
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        self.opposite()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum AxisDirection {
    Positive,
    Negative,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Plane {
    Horizontal,
    Vertical,
}
