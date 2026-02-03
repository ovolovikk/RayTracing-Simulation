use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

macro_rules! impl_vec_op {
    ($trait:ident, $method:ident) => {
        impl $trait<Vec3> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: Vec3) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }
    };
}

macro_rules! impl_vec_op_scalar {
    ($trait:ident, $method:ident) => {
        impl $trait<f32> for Vec3 {
            type Output = Vec3;
            fn $method(self, rhs: f32) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs),
                    y: self.y.$method(rhs),
                    z: self.z.$method(rhs),
                }
            }
        }

        impl $trait<Vec3> for f32 {
            type Output = Vec3;
            fn $method(self, rhs: Vec3) -> Self::Output {
                Vec3 {
                    x: self.$method(rhs.x),
                    y: self.$method(rhs.y),
                    z: self.$method(rhs.z),
                }
            }
        }
    };
}

impl_vec_op!(Add, add);
impl_vec_op!(Sub, sub);
impl_vec_op!(Mul, mul);
impl_vec_op!(Div, div);
impl_vec_op_scalar!(Add, add);
impl_vec_op_scalar!(Sub, sub);
impl_vec_op_scalar!(Mul, mul);
impl_vec_op_scalar!(Div, div);

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 { *self } else { *self / len }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

pub fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    let a = rng.gen_range(-1.0..1.0);
    let b = rng.gen_range(-1.0..1.0);
    let c = rng.gen_range(-1.0..1.0);
    let v = Vec3 { x: a, y: b, z: c };
    if v.near_zero() {
        random_unit_vector(rng)
    } else {
        v.normalize()
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}
