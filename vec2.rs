use core::vec::*;

pub struct Vec2 { x:float, y:float }

pub fn Vec2(x:float, y:float) -> Vec2 { Vec2 { x: x, y: y } }

pub static Zero:Vec2 = Vec2 { x: 0., y: 0. };
pub static One:Vec2 = Vec2 { x: 1., y: 1. };
pub static XAxis:Vec2 = Vec2 { x: 1., y: 0. };
pub static YAxis:Vec2 = Vec2 { x: 0., y: 1. };

impl Add<Vec2,Vec2> for Vec2 {
    fn add(&self, rhs: &Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vec2,Vec2> for Vec2 {
    fn sub(&self, rhs: &Vec2) -> Vec2 {
        Vec2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<float,Vec2> for Vec2 {
    fn mul(&self, rhs: &float) -> Vec2 {
        Vec2 {x: self.x * (*rhs), y: self.y * (*rhs)}
    }
}

impl Div<float,Vec2> for Vec2 {
    fn div(&self, rhs: &float) -> Vec2 {
        Vec2 {x: self.x / (*rhs), y: self.y / (*rhs)}
    }
}

impl Neg<Vec2> for Vec2 {
    fn neg(&self) -> Vec2 {
        Vec2 {x: -self.x, y: -self.y}
    }
}

pub impl Vec2 {
    fn length(&self) -> float {
        float::sqrt(self.length2())
    }
    fn length2(&self) -> float {
        self.x * self.x + self.y * self.y
    }
    fn normalize(&self) -> Vec2 {
        let len = self.length();
        assert!(len != 0.);
        Vec2 {x: self.x / len, y: self.y / len}
    }
    fn normalizeOr(&self, b: Vec2) -> Vec2 {
        let len = self.length();
        if (len == 0.) { b }
        else { Vec2 {x: self.x / len, y: self.y / len} }
    }
    fn normalizeOrZero(&self) -> Vec2 {
        self.normalizeOr(Zero)
    }
    fn dot(&self, b: Vec2) -> float {
        self.x * b.x + self.y * b.y
    }
    fn normalRight(&self) -> Vec2 {
        Vec2 { x: -self.y, y: self.x }
    }
    fn normalLeft(&self) -> Vec2 {
        Vec2 { x: self.y, y: -self.x }
    }
}
