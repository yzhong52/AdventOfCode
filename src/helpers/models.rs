use std::ops;
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct _Point<T> {
    pub x: T,
    pub y: T,
}

impl _Point<i32> {
    pub fn origin() -> _Point<i32> {
        _Point { x: 0, y: 0 }
    }
}

impl<T> ops::AddAssign<_Point<T>> for _Point<T> where T: num::Integer, T: ops::AddAssign {
    fn add_assign(&mut self, rhs: _Point<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::Add<_Point<T>> for _Point<T> where T: num::Integer, T: ops::Add {
    type Output = _Point<T>;

    fn add(self, rhs: _Point<T>) -> _Point<T> {
        _Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}


impl _Point<i128> {
    pub fn origin() -> _Point<i128> {
        _Point { x: 0, y: 0 }
    }
}

pub type Point = _Point<i32>;
pub type BigPoint = _Point<i128>;

pub struct Rect<P> {
    pub lower: P,
    pub upper: P,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn origin() -> Point3D {
        Point3D { x: 0, y: 0, z: 0 }
    }
}
