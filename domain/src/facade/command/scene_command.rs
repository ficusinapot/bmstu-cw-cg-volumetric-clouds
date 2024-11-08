use egui::Color32;
use log::debug;

use crate::facade::Command;
use crate::managers::ManagerSolution;
use crate::object::Component;
use crate::object::objects::texture3d::WorleyBuilder;

pub enum SceneCommand {
    AddObject(&'static str, Component),
    GetObject(Component),
    RemoveObject(Component),

    SetNumSteps(&'static str, usize),
    SetCloudScale(&'static str, f32),
    SetDensityMultiplier(&'static str, f32),
    SetDensityThreshold(&'static str, f32),
    SetDensityOffset(&'static str, f32),
    SetOffset(&'static str, glam::Vec3),
    SetAlphaThreshold(&'static str, u8),

    SetNoise(&'static str, WorleyBuilder),
    SetDetailNoise(&'static str, WorleyBuilder),

    SetDetailNoiseScale(&'static str, f32),
    SetDetailNoiseWeight(&'static str, f32),
    SetDetailWeights(&'static str, glam::Vec3),
    SetShapeNoiseWeights(&'static str, glam::Vec4),
    SetPhaseParams(&'static str, glam::Vec4),
    SetShapeOffset(&'static str, glam::Vec3),
    SetDetailOffset(&'static str, glam::Vec3),

    SetLightAbsorptionTowardSun(&'static str, f32),
    SetLightAbsorptionThroughCloud(&'static str, f32),
    SetDarknessThreshold(&'static str, f32),
    SetLightColor(&'static str, Color32),
    SetColA(&'static str, Color32),
    SetColB(&'static str, Color32),
}

impl Command for SceneCommand {
    type ReturnType = ();
    fn exec(self, manager: &mut ManagerSolution) {
        match self {
            SceneCommand::AddObject(name, component) => {
                let sm = manager.get_mut_scene_manager();
                sm.add_object(name, component);
            }
            SceneCommand::GetObject(_component) => {
                debug!("get object");
            }
            SceneCommand::RemoveObject(_component) => {
                debug!("remove object");
            }
            SceneCommand::SetNumSteps(id, num_steps) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.num_steps = num_steps;
                }
            }
            SceneCommand::SetCloudScale(id, cloud_scale) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.cloud_scale = (cloud_scale)
                }
            }
            SceneCommand::SetDensityMultiplier(id, density_multiplier) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.density_multiplier = (density_multiplier)
                }
            }
            SceneCommand::SetDensityThreshold(id, density_threshold) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.density_threshold = (density_threshold)
                }
            }
            SceneCommand::SetDensityOffset(id, d) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.density_offset = (d)
                }
            }
            SceneCommand::SetOffset(id, offset) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.clouds_offset = (offset)
                }
            }
            SceneCommand::SetAlphaThreshold(id, threshold) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.alpha_threshold = (threshold)
                }
            }
            SceneCommand::SetNoise(id, noise) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.regenerate_noise(noise)
                }
            }
            SceneCommand::SetDetailNoise(id, noise) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.regenerate_detail_noise(noise)
                }
            }
            SceneCommand::SetDetailNoiseScale(id, detail_noise_scale) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.detail_noise_scale = (detail_noise_scale)
                }
            }
            SceneCommand::SetDetailNoiseWeight(id, detail_noise_weight) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.detail_noise_weight = (detail_noise_weight)
                }
            }
            SceneCommand::SetDetailWeights(id, detail_weights) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.detail_weights = (detail_weights)
                }
            }
            SceneCommand::SetShapeNoiseWeights(id, shape_noise_weights) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.shape_noise_weights = (shape_noise_weights)
                }
            }
            SceneCommand::SetPhaseParams(id, phase_params) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.phase_params = (phase_params)
                }
            }
            SceneCommand::SetShapeOffset(id, shape_offset) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.shape_offset = (shape_offset)
                }
            }
            SceneCommand::SetDetailOffset(id, detail_offset) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.detail_offset = (detail_offset)
                }
            }
            SceneCommand::SetLightAbsorptionTowardSun(id, light_absorption_toward_sun) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.light_absorption_toward_sun = (light_absorption_toward_sun)
                }
            }
            SceneCommand::SetLightAbsorptionThroughCloud(id, light_absorption_through_cloud) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.light_absorption_through_cloud = (light_absorption_through_cloud)
                }
            }
            SceneCommand::SetDarknessThreshold(id, darkness_threshold) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.darkness_threshold = (darkness_threshold)
                }
            }
            SceneCommand::SetLightColor(id, light_color) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.light_color = (light_color)
                }
            }
            SceneCommand::SetColA(id, col_a) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.col_a = (col_a)
                }
            }
            SceneCommand::SetColB(id, col_b) => {
                if let Some(Component::Cloud(cloud)) =
                    manager.get_mut_scene_manager().get_mut_object(id)
                {
                    cloud.col_b = (col_b)
                }
            }
        }
    }
}
