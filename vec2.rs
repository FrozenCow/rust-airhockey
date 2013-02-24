use core::float::*;
use core::num::*;
use core::ptr::ref_eq;
use core::cmp::Eq;
use core::vec::*;

pub struct Vec2 { x:float, y:float }

pub fn Vec2(x:float, y:float) -> Vec2 { Vec2 { x: x, y: y } }

const Zero:Vec2 = Vec2 { x: 0., y: 0. };
const One:Vec2 = Vec2 { x: 1., y: 1. };
const XAxis:Vec2 = Vec2 { x: 1., y: 0. };
const YAxis:Vec2 = Vec2 { x: 0., y: 1. };

impl Add<Vec2,Vec2> for Vec2 {
    pure fn add(&self, rhs: &Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vec2,Vec2> for Vec2 {
    pure fn sub(&self, rhs: &Vec2) -> Vec2 {
        Vec2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<float,Vec2> for Vec2 {
    pure fn mul(&self, rhs: &float) -> Vec2 {
        Vec2 {x: self.x * (*rhs), y: self.y * (*rhs)}
    }
}

impl Div<float,Vec2> for Vec2 {
    pure fn div(&self, rhs: &float) -> Vec2 {
        Vec2 {x: self.x / (*rhs), y: self.y / (*rhs)}
    }
}

impl Neg<Vec2> for Vec2 {
    pure fn neg(&self) -> Vec2 {
        Vec2 {x: -self.x, y: -self.y}
    }
}

impl Vec2 {
    pure fn length(&self) -> float {
        float::sqrt(self.length2())
    }
    pure fn length2(&self) -> float {
        self.x * self.x + self.y * self.y
    }
    pure fn normalize(&self) -> Vec2 {
        let len = self.length();
        assert len != 0.;
        Vec2 {x: self.x / len, y: self.y / len}
    }
    pure fn normalizeOr(&self, b: Vec2) -> Vec2 {
        let len = self.length();
        if (len == 0.) { b }
        else { Vec2 {x: self.x / len, y: self.y / len} }
    }
    pure fn normalizeOrZero(&self) -> Vec2 {
        self.normalizeOr(Zero)
    }
    pure fn dot(&self, b: Vec2) -> float {
        self.x * b.x + self.y * b.y
    }
    pure fn normalRight(&self) -> Vec2 {
        Vec2 { x: -self.y, y: self.x }
    }
    pure fn normalLeft(&self) -> Vec2 {
        Vec2 { x: self.y, y: -self.x }
    }
}
