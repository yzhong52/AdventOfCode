use std::ops;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct _Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> _Point<T> where T: num::Integer, T: Copy {
    pub fn origin() -> _Point<T> {
        _Point { x: T::zero(), y: T::zero() }
    }

    pub fn is_valid(&self, max_x: T, max_y: T) -> bool {
        return T::zero() <= self.x && self.x < max_x
            && T::zero() <= self.y && self.y < max_y;
    }

    pub fn neighbours4(&self, max_x: T, max_y: T) -> Vec<_Point<T>> {
        let mut result: Vec<_Point<T>> = Vec::new();
        if self.x > T::zero() {
            result.push(_Point { x: self.x - T::one(), y: self.y });
        }
        if self.y > T::zero() {
            result.push(_Point { x: self.x, y: self.y - T::one() });
        }
        if self.x + T::one() < max_x {
            result.push(_Point { x: self.x + T::one(), y: self.y });
        }
        if self.y + T::one() < max_y {
            result.push(_Point { x: self.x, y: self.y + T::one() })
        }
        result
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
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Sub<_Point<T>> for _Point<T> where T: num::Integer, T: ops::Sub {
    type Output = _Point<T>;

    fn sub(self, rhs: _Point<T>) -> _Point<T> {
        _Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// TODO: Yuchen - move them into templates
impl _Point<i128> {
    pub fn turn_right(&self) -> _Point<i128> {
        _Point { x: self.y, y: -self.x }
    }

    pub fn turn_left(&self) -> _Point<i128> {
        _Point { x: -self.y, y: self.x }
    }

    pub fn dot_product(self, rhs: _Point<i128>) -> i128 {
        self.x * rhs.x + self.y * rhs.y
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
