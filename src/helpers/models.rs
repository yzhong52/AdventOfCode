#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct _Point<T> {
    pub x: T,
    pub y: T,
}

impl _Point<i32> {
    pub fn origin() -> _Point<i32> {
        _Point { x: 0, y: 0}
    }
}

impl _Point<i128> {
    pub fn origin() -> _Point<i128> {
        _Point { x: 0, y: 0}
    }
}

pub type Point = _Point<i32>;
pub type BigPoint = _Point<i128>;

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
