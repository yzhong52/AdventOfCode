#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
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
