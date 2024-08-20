use std::ops::Deref;

use crate::math::{Mtx4x4, Vec3};
use nalgebra::{Rotation3, Scale3, Translation3};

#[derive(Debug)]
pub struct Rotation(Rotation3<f64>);

impl Deref for Rotation {
    type Target = Rotation3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Rotation> for Rotation3<f64> {
    fn from(value: Rotation) -> Self {
        value.0
    }
}

impl From<Rotation3<f64>> for Rotation {
    fn from(value: Rotation3<f64>) -> Self {
        Self(value)
    }
}

impl From<Rotation> for Vec3 {
    fn from(value: Rotation) -> Self {
        let (x, y, z) = value.into();
        Vec3::new(x, y, z)
    }
}

impl From<Rotation> for (f64, f64, f64) {
    fn from(value: Rotation) -> Self {
        let tup = value.into();
        tup
    }
}

#[derive(Debug)]
pub struct Scale(Scale3<f64>);

impl Deref for Scale {
    type Target = Scale3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Scale> for Scale3<f64> {
    fn from(value: Scale) -> Self {
        value.0
    }
}

impl From<Scale> for Vec3 {
    fn from(value: Scale) -> Self {
        let (x, y, z) = value.into();
        Vec3::new(x, y, z)
    }
}

impl From<Scale> for (f64, f64, f64) {
    fn from(value: Scale) -> Self {
        let tup = value.into();
        tup
    }
}

impl From<Scale3<f64>> for Scale {
    fn from(value: Scale3<f64>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Translation(Translation3<f64>);

impl Deref for Translation {
    type Target = Translation3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Translation> for Translation3<f64> {
    fn from(value: Translation) -> Self {
        value.0
    }
}

impl From<Translation3<f64>> for Translation {
    fn from(value: Translation3<f64>) -> Self {
        Self(value)
    }
}

impl From<Translation> for Vec3 {
    fn from(value: Translation) -> Self {
        let (x, y, z) = value.into();
        Vec3::new(x, y, z)
    }
}

impl From<Translation> for (f64, f64, f64) {
    fn from(value: Translation) -> Self {
        let tup = value.into();
        tup
    }
}

#[derive(Debug)]
pub struct Transform {
    rotation: Rotation,
    scale: Scale,
    translation: Translation,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Rotation3::default().into(),
            scale: Scale3::new(1.0, 1.0, 1.0).into(),
            translation: Translation3::default().into(),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Scale) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_translation(mut self, translation: Translation) -> Self {
        self.translation = translation;
        self
    }

    pub fn get_transformation(&self) -> Mtx4x4 {
        let mtx = Mtx4x4::identity();
        let mtx = mtx.translate(self.translation.vector.into());
        // let mut mtx = mtx.rotate(self.translation.vector.into());
        mtx.scale(self.scale.vector.into())
    }
}
