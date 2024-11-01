use crate::visitor::{Visitable, Visitor};
use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use std::f32::consts::FRAC_PI_2;

/// Camera controller and parameters
#[derive(Default, Copy, Clone, Debug)]
pub struct Camera {
    pub proj: Perspective,
    pub view: ArcBall,
    pub control: ArcBallController,
}

impl Visitable for Camera {
    fn accept(&self, _visitor: &impl Visitor) {
        todo!()
    }
}

impl Camera {
    pub fn get_pixel_screen_position(&self, world_pos: Vec3, width: usize, height: usize) -> Option<(usize, usize)> {
        let view_matrix = self.view();
        let proj_matrix = self.projection(width as f32, height as f32);
        
        let camera_pos = view_matrix.transform_point3(world_pos);
        let clip_space_pos = proj_matrix * camera_pos.extend(1.0);

        if clip_space_pos.w == 0.0 {
            return None; 
        }

        let ndc_pos = clip_space_pos.xyz() / clip_space_pos.w;
        
        let pixel_x = ((ndc_pos.x + 1.0) * 0.5 * width as f32) as isize;
        let pixel_y = ((1.0 - ndc_pos.y) * 0.5 * height as f32) as isize;

        if pixel_x >= 0 && pixel_x < width as isize && pixel_y >= 0 && pixel_y < height as isize {
            Some((pixel_x as usize, pixel_y as usize))
        } else {
            None
        }
    }
    
    pub fn get_pixel_world_position(&self, i: usize, j: usize, width: usize, height: usize) -> Vec3 {
        let aspect_ratio = width as f32 / height as f32;
        let fov_adjustment = (self.proj.fov / 2.0).tan();

        let pixel_ndc_x = (j as f32) / width as f32;
        let pixel_ndc_y = (i as f32) / height as f32;

        let pixel_screen_x = 2.0 * pixel_ndc_x - 1.0;
        let pixel_screen_y = 1.0 - 2.0 * pixel_ndc_y;

        let pixel_camera_x = pixel_screen_x * aspect_ratio * fov_adjustment;
        let pixel_camera_y = pixel_screen_y * fov_adjustment;

        let pixel_camera_position = Vec3::new(pixel_camera_x, pixel_camera_y, -1.0);
        let view_matrix = self.view();
        let pixel_world_position = view_matrix.inverse().transform_point3(pixel_camera_position);

        pixel_world_position
    }

    pub fn get_position(&self) -> Vec3 {
        self.view.eye()
    }

    pub fn get_pivot(&self) -> Vec3 {
        self.view.pivot
    }

    pub fn get_direction(&self) -> Vec3 {
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
}

/// Perspective projection parameters
#[derive(Copy, Clone, Debug)]
pub struct Perspective {
    pub fov: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

/// Arcball camera parameters
#[derive(Copy, Clone, Debug)]
pub struct ArcBall {
    pub pivot: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

/// Arcball camera controller parameters
#[derive(Copy, Clone, Debug)]
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
        Mat4::look_at_rh(
            eye,
            self.pivot - eye,
            Vec3::new(0.0, 1.0, 0.0),
        )
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
            (-delta_x as f32) * arcball.distance,
            (delta_y as f32) * arcball.distance,
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
            pitch: 0.6,
            yaw: 0.0,
            distance: 5.,
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

    #[test]
    fn test_camera_direction() {
        let camera = Camera::default();
        let dir = camera.get_direction();
        assert_eq!(dir, Vec3::new(-0.99503726, 0.099503726, 0.0));
        assert_eq!(dir.length(), 1.0);
    }
}