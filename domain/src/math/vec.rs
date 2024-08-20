use std::ops::Deref;

use nalgebra::{Vector3, Vector4};

pub struct Vec3(Vector3<f64>);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3::new(x, y, z).into()
    }
}

pub struct Vec4(Vector4<f64>);

impl Deref for Vec3 {
    type Target = Vector3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec3> for Vector3<f64> {
    fn from(value: Vec3) -> Self {
        value.0
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(value: Vec3) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<Vector3<f64>> for Vec3 {
    fn from(value: Vector3<f64>) -> Self {
        Self(value)
    }
}

impl Deref for Vec4 {
    type Target = Vector4<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
