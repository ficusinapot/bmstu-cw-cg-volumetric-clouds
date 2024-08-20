use crate::math::Vec3;
use nalgebra::Matrix4;
use std::ops::Deref;

pub struct Mtx4x4(Matrix4<f64>);

impl Mtx4x4 {
    pub fn identity() -> Self {
        Matrix4::identity().into()
    }

    pub fn rotate(&self, _radians: f64, _vec: Vec3) -> Self {
        unimplemented!()
    }

    pub fn translate(&self, vec: Vec3) -> Self {
        self.append_translation(vec.deref()).into()
    }

    pub fn scale(&self, scaling: Vec3) -> Self {
        self.append_nonuniform_scaling(scaling.deref()).into()
    }
}

impl Deref for Mtx4x4 {
    type Target = Matrix4<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Matrix4<f64>> for Mtx4x4 {
    fn from(value: Matrix4<f64>) -> Self {
        Self(value)
    }
}

impl From<Mtx4x4> for Matrix4<f64> {
    fn from(value: Mtx4x4) -> Self {
        value.0
    }
}
