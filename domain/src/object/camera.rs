use std::f32::consts::FRAC_PI_2;

use egui::{Pos2, Rect, Vec2};
use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::math::Transform;
use crate::visitor::{Visitable, Visitor};

/// Camera controller and parameters
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub proj: Perspective,
    pub view: ArcBall,
    pub control: ArcBallController,
}

impl Visitable for Camera {
    fn accept(&self, _visitor: &mut impl Visitor) {
        todo!()
    }
}

impl Camera {
    pub fn pos(&self) -> Vec3 {
        self.view.eye()
    }

    pub fn dir(&self) -> Vec3 {
        (self.view.pivot - self.view.eye()).normalize()
    }

    /// Return the projection matrix of this camera
    pub fn projection(&self, width: f32, height: f32) -> Mat4 {
        self.proj.matrix(width, height)
    }

    /// Return the view matrix of this camera
    pub fn view(&self) -> Mat4 {
        self.view.matrix()
    }

    /// Pivot the camera by the given mouse pointer delta
    pub fn pivot(&mut self, delta_x: f32, delta_y: f32) {
        self.control.pivot(&mut self.view, delta_x, delta_y)
    }

    /// Pan the camera by the given mouse pointer delta
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        self.control.pan(&mut self.view, delta_x, delta_y)
    }

    /// Zoom the camera by the given mouse scroll delta
    pub fn zoom(&mut self, delta: f32) {
        self.control.zoom(&mut self.view, delta)
    }

    pub fn egui_to_world(&self, i: usize, j: usize, width: usize, height: usize) -> Vec3 {
        let t = Transform::new(
            self.projection(width as f32, height as f32) * self.view(),
            Rect::from_min_size(Pos2::ZERO, (width as f32, height as f32).into()),
        );

        t.egui_to_world(Vec2::new(j as f32, i as f32), -1.)
    }
}

/// Perspective projection parameters
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Perspective {
    pub fov: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

/// Arcball camera parameters
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArcBall {
    pub pivot: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

/// Arcball camera controller parameters
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArcBallController {
    pub pan_sensitivity: f32,
    pub swivel_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub closest_zoom: f32,
}

impl Perspective {
    pub fn matrix(&self, width: f32, height: f32) -> Mat4 {
        Mat4::perspective_rh(self.fov, width / height, self.clip_near, self.clip_far)
    }
}

impl ArcBall {
    pub fn pivot(&self) -> Vec3 {
        self.pivot
    }

    pub fn matrix(&self) -> Mat4 {
        let eye = self.eye();
        Mat4::look_at_rh(eye, self.pivot, Vec3::new(0.0, 1.0, 0.0))
    }

    pub fn eye(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ) * self.distance
    }
}

impl ArcBallController {
    pub fn pivot(&mut self, arcball: &mut ArcBall, delta_x: f32, delta_y: f32) {
        arcball.yaw += delta_x * self.swivel_sensitivity;
        arcball.pitch += delta_y * self.swivel_sensitivity;

        arcball.pitch = arcball.pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn pan(&mut self, arcball: &mut ArcBall, delta_x: f32, delta_y: f32) {
        let delta = Vec4::new(
            (-delta_x) * arcball.distance,
            (delta_y) * arcball.distance,
            0.0,
            0.0,
        ) * self.pan_sensitivity;

        // TODO: This is dumb, just use the cross product 4head
        let inv = arcball.matrix().inverse();
        let delta = (inv * delta).xyz();
        arcball.pivot += delta;
    }

    pub fn zoom(&mut self, arcball: &mut ArcBall, delta: f32) {
        arcball.distance += delta * self.zoom_sensitivity.powf(2.) * arcball.distance;
        arcball.distance = arcball.distance.max(self.closest_zoom);
    }
}

// Arbitrary
impl Default for ArcBall {
    fn default() -> Self {
        Self {
            pivot: Vec3::new(0.0, 0.5, 0.0),
            pitch: 0.4,
            yaw: 3.7,
            distance: 15.,
        }
    }
}

// Arbitrary
impl Default for Perspective {
    fn default() -> Self {
        Self {
            fov: 45.0f32.to_radians(),
            clip_near: 0.1,
            clip_far: 100.0,
        }
    }
}

// Arbitrary
impl Default for ArcBallController {
    fn default() -> Self {
        Self {
            pan_sensitivity: 0.0015,
            swivel_sensitivity: 0.005,
            zoom_sensitivity: 0.04,
            closest_zoom: 0.01,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_camera_trans() {
            let camera = Camera {
                proj: Perspective {
                    fov: 45.0,
                    clip_near: 0.0,
                    clip_far: 100.0,
                },
                view: ArcBall {
                    pivot: Vec3::ZERO,
                    distance: 10.0,
                    yaw: 0.0,
                    pitch: 0.0,
                },
                control: Default::default(),
            };

            let dir = camera.dir();
            assert_eq!(dir, Vec3::new(-1.0, 0.0, 0.0));
            assert_eq!(dir.length(), 1.0);
        }

        #[test]
        fn test_camera_direction() {
            let camera = Camera {
                proj: Perspective {
                    fov: 45.0,
                    clip_near: 0.0,
                    clip_far: 100.0,
                },
                view: ArcBall {
                    pivot: Vec3::ZERO,
                    distance: 10.0,
                    yaw: 0.0,
                    pitch: 0.0,
                },
                control: Default::default(),
            };

            let dir = camera.dir();
            assert_eq!(dir, Vec3::new(-1.0, 0.0, 0.0));
            assert_eq!(dir.length(), 1.0);
        }

        #[test]
        fn test_camera_position() {
            let camera = Camera {
                proj: Perspective {
                    fov: 45.0,
                    clip_near: 0.0,
                    clip_far: 100.0,
                },
                view: ArcBall {
                    pivot: Vec3::ZERO,
                    distance: 10.0,
                    yaw: 0.0,
                    pitch: 0.0,
                },
                control: Default::default(),
            };
            assert_eq!(camera.pos(), Vec3::new(10.0, 0.0, 0.0));
        }
    }
}
