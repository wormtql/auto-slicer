use std::cmp::Ordering;
use std::ops::{Sub, Add, Mul, Div};
use crate::ray::Ray;


#[derive(Copy, Clone, Debug)]
pub struct V3(pub f64, pub f64, pub f64);

impl Sub for V3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        V3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for V3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Div<f64> for V3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        V3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<f64> for V3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        V3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl V3 {
    pub const ORIGIN: V3 = V3(0.0, 0.0, 0.0);

    pub fn normalize(&mut self) {
        let t = self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
        let t = t.sqrt();

        self.0 /= t;
        self.1 /= t;
        self.2 /= t;
    }

    pub fn cross(&self, other: V3) -> V3 {
        V3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }

    pub fn dot(&self, other: V3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn from_vec_f32(v: &[f32]) -> V3 {
        V3(v[0] as f64, v[1] as f64, v[2] as f64)
    }

    pub fn radiative_ray(&self, axis: usize, count: usize) -> RadiativeLine {
        RadiativeLine {
            count,
            axis,
            origin: self.clone(),
            next: 0,
            offset_radius: 0.0
        }
    }
}

pub struct RadiativeLine {
    pub count: usize,
    pub axis: usize,
    pub origin: V3,
    pub next: usize,
    pub offset_radius: f64,
}

impl Iterator for RadiativeLine {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.count {
            return None;
        }

        let radius = self.offset_radius + 2.0 * std::f64::consts::PI / (self.count as f64) * (self.next as f64);
        self.next += 1;

        if self.axis == 0 {
            // perpendicular to x
            let y = radius.cos();
            let z = radius.sin();

            let dir = V3(0.0, y, z);

            return Some(Ray {
                direction: dir,
                start: self.origin
            })
        } else if self.axis == 1 {
            let x = radius.cos();
            let z = radius.sin();

            let dir = V3(x, 0.0, z);

            return Some(Ray {
                direction: dir,
                start: self.origin
            })
        } else {
            let x = radius.cos();
            let y = radius.sin();

            let dir = V3(x, y, 0.0);

            return Some(Ray {
                direction: dir,
                start: self.origin
            })
        }
    }
}