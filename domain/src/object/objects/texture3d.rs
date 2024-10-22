use glam::Vec3;
use noise::{Fbm as _Fbm, Worley as _Worley};

pub enum Texture3d {
    Fbm(Fbm),
}

impl Texture3d {
    pub fn new() -> Self {
        Texture3d::Fbm(Fbm::new(1))
    }
}

pub struct Fbm(_Fbm<_Worley>);

impl Fbm {
    fn new(sid: u32) -> Self {
        let fbm = noise::Fbm::new(1);
        Self(fbm)
    }

    fn get(&self, vec3: Vec3) -> f32 {
        let p: (f32, f32, f32) = vec3.into();
        let p = (p.0 as f64, p.1 as f64, p.2 as f64);
        noise::NoiseFn::get(&self.0, p.into()) as f32
    }
}
