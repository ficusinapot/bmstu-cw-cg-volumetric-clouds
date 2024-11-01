use crate::facade::Command;
use crate::managers::ManagerSolution;
use crate::object::Component;
use egui::Color32;
use log::debug;

pub enum SceneCommand {
    AddObject(&'static str, Component),
    GetObject(Component),
    RemoveObject(Component),

    SetNumSteps(&'static str, usize),
    SetCloudScale(&'static str, f32),
    SetDensityMultiplier(&'static str, f32),
    SetDensityThreshold(&'static str, f32),
    SetOffset(&'static str, glam::Vec3),
    SetAlphaThreshold(&'static str, u8),
    SetColor(&'static str, Color32),
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
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.num_steps = num_steps;
                }
            }
            SceneCommand::SetCloudScale(id, cloud_scale) => {
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.set_cloud_scale(cloud_scale)
                }
            }
            SceneCommand::SetDensityMultiplier(id, density_multiplier) => {
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.set_density_multiplier(density_multiplier)
                }
            }
            SceneCommand::SetDensityThreshold(id, density_threshold) => {
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.set_density_threshold(density_threshold)
                }
            }
            SceneCommand::SetOffset(id, offset) => {
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.set_clouds_offset(offset)
                }
            }
            SceneCommand::SetAlphaThreshold(id, threshold) => {
                let sm = manager.get_mut_scene_manager();
                let cloud = sm.get_mut_object(id);
                if let Some(Component::Cloud(cloud)) = cloud {
                    cloud.set_alpha_threshold(threshold)
                }
            }
            _ => {
                
            }
        }
    }
}
