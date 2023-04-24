use std::{fmt::Display, ops};

use crate::{random, random_range};

#[derive(Default, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    // returns <0, 0, 0>
    pub fn zero() -> Self {
        Vec3::default()
    }

    // returns <1, 1, 1>
    pub fn one() -> Self {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn random() -> Self {
        Vec3(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random();
            if p.length_sqaured() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_sqaured())
    }

    pub fn length_sqaured(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

// |a||b|*costA where A is the angle between vector
pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

// resulting in a vector that is perpendicular to both vectors
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - b.1 * a.2,
        b.0 * a.2 - a.0 * b.2,
        a.0 * b.1 - b.0 * a.1,
    )
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::random_in_unit_sphere().unit_vector()
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (rhs * -1.0)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}
