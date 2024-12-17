use eframe::egui;
use eframe::egui::Color32;

use domain::canvas::painter::Painter3D;
use domain::facade::{CameraCommand, DrawCommand, SceneCommand};
use domain::facade::{Executor, Facade};
use domain::math::transform::glam;
use domain::math::transform::glam::{Vec3, Vec4};
use domain::object::camera::Camera;
use domain::object::objects::cloud::CloudBuilder;
use domain::object::objects::Sun;
use domain::object::objects::terrain::TerrainBuilder;
use domain::object::objects::texture3d::{PerlinBuilder, WorleyBuilder};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        ctx.request_repaint();
        self.cloud.offset += self.offset_speed;
        self.executor
            .exec(SceneCommand::SetOffset("cloud", self.cloud.offset));
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
        self.executor.exec(DrawCommand::Draw);
    }
}

impl App {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(1024.0);
                ui.set_height(800.0);
                let (resp, painter) = self.painter(ui);
                self.executor.exec(DrawCommand::SetPainter(painter));
                self.handle_camera(&resp, ui);
            });
            self.control(ui);
        });
    }

    fn painter(&mut self, ui: &mut egui::Ui) -> (egui::Response, Painter3D) {
        let (response, painter) =
            ui.allocate_painter([1056.0, 900.0].into(), egui::Sense::click_and_drag());

        let bc = self.background_color;
        painter.rect(
            painter.clip_rect().shrink(0.0),
            0.0,
            bc,
            egui::Stroke::new(0.5, egui::Color32::BLACK),
        );
        let rect = response.rect;
        (response, Painter3D::new(painter, rect, bc))
    }

    fn handle_camera(&mut self, resp: &egui::Response, ui: &mut egui::Ui) {
        if resp.dragged_by(egui::PointerButton::Primary) {
            if ui.input(|i| i.raw.modifiers.shift_only()) {
                let pan = CameraCommand::Pan(resp.drag_delta().x, resp.drag_delta().y);
                self.executor.exec(pan);
            } else {
                let pivot = CameraCommand::Pivot(resp.drag_delta().x, resp.drag_delta().y);
                self.executor.exec(pivot);
            }
        }

        if resp.dragged_by(egui::PointerButton::Secondary) {
            let pan = CameraCommand::Pan(resp.drag_delta().x, resp.drag_delta().y);
            self.executor.exec(pan);
        }

        if resp.hovered() {
            let zoom = CameraCommand::Zoom(ui.input(|i| -i.raw_scroll_delta.y));
            self.executor.exec(zoom);
        }
    }

    fn control(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.collapsing("Параметры облаков", |ui| {
                ui.vertical(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let resp = ui.color_edit_button_srgba(&mut self.cloud.light_color);
                            ui.label("Цвет облака");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetLightColor(
                                    "cloud",
                                    self.cloud.light_color,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.move_vector.y,
                                self.terrain.bounding_box.max.y..=5.0,
                            ));
                            ui.label("Высота облака");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::MoveBoundingBox(
                                    "cloud",
                                    self.move_vector,
                                ));
                            }
                        });
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Slider::new(
                                &mut self.offset_speed.x,
                                -1.0..=1.0,
                            ));
                            ui.label("Ветер x");
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Slider::new(
                                &mut self.offset_speed.y,
                                -1.0..=1.0,
                            ));
                            ui.label("Ветер y");
                        });
                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Slider::new(
                                &mut self.offset_speed.z,
                                -1.0..=1.0,
                            ));
                            ui.label("Ветер z");
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(&mut self.cloud.num_steps, 1..=1000)
                                    .drag_value_speed(0.05),
                            );
                            ui.label("Количество трассировочных точек");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetNumSteps("cloud", self.cloud.num_steps));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.num_steps_light,
                                    1..=1000,
                                )
                                .drag_value_speed(0.05),
                            );
                            ui.label("Трассировочных точек к солнцу");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetNumStepsLight(
                                    "cloud",
                                    self.cloud.num_steps_light,
                                ));
                            }
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.cloud_scale,
                                    1.0..=1000.0,
                                )
                                .drag_value_speed(0.05),
                            );
                            ui.label("Масштаб");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetCloudScale(
                                    "cloud",
                                    self.cloud.cloud_scale,
                                ));
                            }
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.density_multiplier,
                                    1.0..=1000.0,
                                )
                                .drag_value_speed(0.05),
                            );
                            ui.label("Множитель плотности");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetDensityMultiplier(
                                    "cloud",
                                    self.cloud.density_multiplier,
                                ));
                            }
                        });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.detail_noise_scale,
                        //             1.0..=1000.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     ui.label("Detail noise scale");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDetailNoiseScale(
                        //             "cloud",
                        //             self.cloud.detail_noise_scale,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.detail_noise_weight,
                        //             1.0..=50.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     ui.label("Detail noise weight");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDetailNoiseWeight(
                        //             "cloud",
                        //             self.cloud.detail_noise_weight,
                        //         ));
                        //     }
                        // });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.density_offset,
                                    -20.0..=20.0,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Концентрация");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetDensityOffset(
                                    "cloud",
                                    self.cloud.density_offset,
                                ));
                            }
                        });

                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.density_threshold,
                        //             -10.0..=10.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     ui.label("density_threshold");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDensityThreshold(
                        //             "cloud",
                        //             self.cloud.density_threshold,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(egui::widgets::Slider::new(
                        //         &mut self.cloud.detail_weights.x,
                        //         -10.0..=10.0,
                        //     ));
                        //     ui.label("detail_weights_x");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDetailWeights(
                        //             "cloud",
                        //             self.cloud.detail_weights,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(egui::widgets::Slider::new(
                        //         &mut self.cloud.detail_weights.y,
                        //         -10.0..=10.0,
                        //     ));
                        //     ui.label("detail_weights_y");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDetailWeights(
                        //             "cloud",
                        //             self.cloud.detail_weights,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.detail_weights.z,
                        //             -10.0..=10.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     ui.label("detail_weights_z");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetDetailWeights(
                        //             "cloud",
                        //             self.cloud.detail_weights,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(egui::widgets::Slider::new(
                        //         &mut self.cloud.shape_noise_weights.x,
                        //         -0.0..=10.0,
                        //     ));
                        //     ui.label("shape_noise_weights_x");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetShapeNoiseWeights(
                        //             "cloud",
                        //             self.cloud.shape_noise_weights,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.shape_noise_weights.y,
                        //             0.0..=10.0,
                        //         ), // .drag_value_speed(0.001),
                        //     );
                        //     ui.label("shape_noise_weights_y");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetShapeNoiseWeights(
                        //             "cloud",
                        //             self.cloud.shape_noise_weights,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.shape_noise_weights.z,
                        //             0.0..=10.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     ui.label("shape_noise_weights_z");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetShapeNoiseWeights(
                        //             "cloud",
                        //             self.cloud.shape_noise_weights,
                        //         ));
                        //     }
                        // });
                        ui.separator();
                        // ui.label("Фазовая функция");
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(egui::widgets::Slider::new(
                        //         &mut self.cloud.phase_params.x,
                        //         0.0..=0.05,
                        //     ));
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetPhaseParams(
                        //             "cloud",
                        //             self.cloud.phase_params,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(egui::widgets::Slider::new(
                        //         &mut self.cloud.phase_params.y,
                        //         0.0..=1.0,
                        //     ));
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetPhaseParams(
                        //             "cloud",
                        //             self.cloud.phase_params,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.phase_params.z,
                        //             0.0..=1.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetPhaseParams(
                        //             "cloud",
                        //             self.cloud.phase_params,
                        //         ));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.phase_params.w,
                        //             -0.0..=1.0,
                        //         )
                        //         .drag_value_speed(0.001),
                        //     );
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetPhaseParams(
                        //             "cloud",
                        //             self.cloud.phase_params,
                        //         ));
                        //     }
                        // });
                        // ui.separator();
                        ui.label("Смещение");
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.cloud.offset.x,
                                -1000.0..=1000.0,
                            ));
                            ui.label("offset_x");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.cloud.offset));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.offset.y,
                                    -1000.0..=1000.0,
                                ), // .drag_value_speed(0.001),
                            );
                            ui.label("offset_y");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.cloud.offset));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.offset.z,
                                    -1000.0..=1000.0,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("offset_z");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetOffset("cloud", self.cloud.offset));
                            }
                        });
                        ui.separator();
                        // ui.horizontal(|ui| {
                        //     let resp = ui.color_edit_button_srgba(&mut self.cloud.col_a);
                        //     ui.label("Цвет");
                        //     if resp.changed() {
                        //         self.executor
                        //             .exec(SceneCommand::SetColA("cloud", self.cloud.col_a));
                        //     }
                        // });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.color_edit_button_srgba(&mut self.cloud.col_b);
                        //     ui.label("Tint Color");
                        //     if resp.changed() {
                        //         self.executor
                        //             .exec(SceneCommand::SetColB("cloud", self.cloud.col_b));
                        //     }
                        // });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.light_absorption_toward_sun,
                                    0.0..=10.0,
                                )
                                .drag_value_speed(0.01),
                            );
                            ui.label("Поглощение света по солнцу");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetLightAbsorptionTowardSun(
                                        "cloud",
                                        self.cloud.light_absorption_toward_sun,
                                    ));
                            }
                        });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.ray_offset_strength,
                        //             0.0..=100.0,
                        //         )
                        //         .drag_value_speed(0.01),
                        //     );
                        //     ui.label("ray_offset_strength");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetRayOffsetStrength(
                        //             "cloud",
                        //             self.cloud.ray_offset_strength,
                        //         ));
                        //     }
                        // });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.light_absorption_through_cloud,
                                    0.0..=10.0,
                                )
                                .drag_value_speed(0.01),
                            );
                            ui.label("Поглощение света");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetLightAbsorptionThroughCloud(
                                        "cloud",
                                        self.cloud.light_absorption_through_cloud,
                                    ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.darkness_threshold,
                                    0.0..=1.0,
                                )
                                .drag_value_speed(0.01),
                            );
                            ui.label("Порог теней");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetDarknessThreshold(
                                    "cloud",
                                    self.cloud.darkness_threshold,
                                ));
                            }
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.height_map_factor,
                                    0.0..=10.0,
                                )
                                .drag_value_speed(0.01),
                            );
                            ui.label("Фактор карты высот");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetHeightMapFactor(
                                    "cloud",
                                    self.cloud.height_map_factor + 2.0,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(
                                egui::widgets::Slider::new(
                                    &mut self.cloud.edge_distance,
                                    1.0..=5.0,
                                )
                                .drag_value_speed(0.01),
                            );
                            ui.label("Дистанция до края");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetEdgeDistance(
                                    "cloud",
                                    self.cloud.edge_distance,
                                ));
                            }
                        });
                        // ui.horizontal(|ui| {
                        //     let resp = ui.add(
                        //         egui::widgets::Slider::new(
                        //             &mut self.cloud.volume_offset,
                        //             -1.0..=1.0,
                        //         )
                        //         .drag_value_speed(0.01),
                        //     );
                        //     ui.label("volume factor");
                        //     if resp.changed() {
                        //         self.executor.exec(SceneCommand::SetVolumeOffset(
                        //             "cloud",
                        //             self.cloud.volume_offset,
                        //         ));
                        //     }
                        // });
                    });

                    ui.collapsing("Шум Вороного", |ui| {
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.noise_mode, NoiseMode::Shape, "Вид");
                            ui.radio_value(&mut self.noise_mode, NoiseMode::Detail, "Детализация");
                        });
                        let worley_builder = match self.noise_mode {
                            NoiseMode::Shape => &mut self.cloud.noise,
                            NoiseMode::Detail => &mut self.cloud.detail_noise,
                        };

                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::DragValue::new(&mut worley_builder.seed));
                            ui.label("Сид");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut worley_builder.persistence,
                                    0.0..=1.0,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Устойчивости");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut worley_builder.num_points_a,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Слой 1");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut worley_builder.num_points_b,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Слой 2");
                        });

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::widgets::Slider::new(
                                    &mut worley_builder.num_points_c,
                                    1..=30,
                                )
                                .drag_value_speed(0.001),
                            );
                            ui.label("Слой 3");
                        });

                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Checkbox::new(
                                &mut worley_builder.invert_noise,
                                "Инверсия шума",
                            ));
                        });

                        if ui.button("Сгенерировать").clicked() {
                            let command = match self.noise_mode {
                                NoiseMode::Shape => SceneCommand::SetNoise,
                                NoiseMode::Detail => SceneCommand::SetDetailNoise,
                            };
                            self.executor.exec(command("cloud", *worley_builder));
                        }
                    });
                });
            });
            ui.collapsing("Параметры солнца", |ui| {
                ui.vertical(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let resp =
                                ui.add(egui::widgets::Slider::new(&mut self.sun.1, 0.0..=179.0));
                            ui.label("Азумутальный угол");
                            if resp.changed() {
                                self.executor
                                    .exec(SceneCommand::SetSunAngle("sun", (-self.sun.1, self.sun.2).into()));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp =
                                ui.add(egui::widgets::Slider::new(&mut self.sun.2, 0.0..=179.0));
                            ui.label("Зенитный угол");
                            if resp.changed() {
                                self.executor
                                    
                                    .exec(SceneCommand::SetSunAngle("sun", (-self.sun.1, self.sun.2).into()));
                            }
                        });
                    })
                });
            });
            ui.collapsing("Параметры ландшафта", |ui| {
                ui.vertical(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let resp = ui
                                .add(egui::widgets::Slider::new(&mut self.terrain.scale, 1..=100));
                            ui.label("Масштаб");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainScale(
                                    "terrain",
                                    self.terrain.scale,
                                ));
                            }
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.terrain.diffuse_factor,
                                0.0..=1.0,
                            ));
                            ui.label("Диффузия цвета");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainDiffuseFactor(
                                    "terrain",
                                    self.terrain.diffuse_factor,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.terrain.shadow_threshold,
                                0.0..=1.0,
                            ));
                            ui.label("Предел теней");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainShadowThreshold(
                                    "terrain",
                                    self.terrain.shadow_threshold,
                                ));
                            }
                        });

                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.terrain.num_shadows_steps,
                                0..=50,
                            ));
                            ui.label("Количество трассировочных точек");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainNumShadowsSteps(
                                    "terrain",
                                    self.terrain.num_shadows_steps,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.add(egui::widgets::Slider::new(
                                &mut self.terrain.density_scale,
                                0.0..=100.0,
                            ));
                            ui.label("Корректор плотности");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainDensityScale(
                                    "terrain",
                                    self.terrain.density_scale,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.color_edit_button_srgba(&mut self.terrain.top_color);
                            ui.label("Цвет вершин");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainTopColor(
                                    "terrain",
                                    self.terrain.top_color,
                                ));
                            }
                        });
                        ui.horizontal(|ui| {
                            let resp = ui.color_edit_button_srgba(&mut self.terrain.bottom_color);
                            ui.label("Цвет низины");
                            if resp.changed() {
                                self.executor.exec(SceneCommand::SetTerrainBottomColor(
                                    "terrain",
                                    self.terrain.bottom_color,
                                ));
                            }
                        });
                        ui.collapsing("Шум", |ui| {
                            let worley_builder = &mut self.terrain.noise;

                            ui.horizontal(|ui| {
                                ui.add(egui::widgets::DragValue::new(&mut worley_builder.seed));
                                ui.label("Сид");
                            });

                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::widgets::Slider::new(
                                        &mut worley_builder.persistence,
                                        0.0..=1.0,
                                    )
                                    .drag_value_speed(0.001),
                                );
                                ui.label("Устойчивости");
                            });

                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::widgets::Slider::new(
                                        &mut worley_builder.num_points_a,
                                        1..=30,
                                    )
                                    .drag_value_speed(0.001),
                                );
                                ui.label("Слой 1");
                            });

                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::widgets::Slider::new(
                                        &mut worley_builder.num_points_b,
                                        1..=30,
                                    )
                                    .drag_value_speed(0.001),
                                );
                                ui.label("Слой 2");
                            });

                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::widgets::Slider::new(
                                        &mut worley_builder.num_points_c,
                                        1..=30,
                                    )
                                    .drag_value_speed(0.001),
                                );
                                ui.label("Слой 3");
                            });

                            ui.horizontal(|ui| {
                                ui.add(egui::widgets::Checkbox::new(
                                    &mut worley_builder.invert_noise,
                                    "Инверсия шума",
                                ));
                            });

                            if ui.button("Сгенерировать").clicked() {
                                self.executor.exec(SceneCommand::SetTerrainNoise(
                                    "terrain",
                                    *worley_builder,
                                ));
                            }
                        });
                    })
                });
            });
        });
    }
}

pub fn init_app() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([2880.0, 1920.0]),
        centered: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Курсовая работа",
        native_options,
        Box::new(|_| Ok(Box::from(App::init()))),
    )
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
enum NoiseMode {
    #[default]
    Shape,
    Detail,
}

struct App {
    executor: Facade,
    noise_mode: NoiseMode,
    cloud: CloudBuilder,
    terrain: TerrainBuilder,
    sun: (f32, f32, f32),
    background_color: Color32,
    offset_speed: Vec3,
    move_vector: Vec3,
    extend_vector: Vec3
}

impl App {
    fn init() -> Self {
        let noise = WorleyBuilder::new()
            .with_seed(0)
            .with_num_points_a(6)
            .with_num_points_b(12)
            .with_num_points_c(22)
            .with_tile(1.0)
            .with_resolution(128)
            .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
            .with_persistence(0.84)
            .with_invert_noise(true);

        let detail_noise = WorleyBuilder::new()
            .with_seed(0)
            .with_num_points_a(7)
            .with_num_points_b(7)
            .with_num_points_c(11)
            .with_tile(1.0)
            .with_resolution(64)
            .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
            .with_persistence(0.89)
            .with_invert_noise(true);

        let cloud_params = CloudBuilder::default()
            .with_map_size(glam::IVec3::ZERO)
            .with_bounding_box((Vec3::new(-3.5, 2.5, -3.5), Vec3::new(3.5, 3.5, 3.5)))
            .with_shape_offset(Vec3::ZERO)
            .with_detail_offset(Vec3::ZERO)
            .with_cloud_scale(210.0)
            .with_density_threshold(0.95)
            .with_density_multiplier(360.0)
            .with_num_steps(200)
            .with_num_steps_light(20)
            .with_density_offset(-8.30)
            .with_noise(noise)
            .with_shape_noise_weights(Vec4::new(3.0, 6.0, 5.0, 1.0))
            .with_detail_noise(detail_noise)
            .with_detail_noise_weight(1.0)
            .with_detail_weights(Vec4::new(4.0, 1.5, 1.5, 3.0))
            .with_detail_noise_scale(1.09)
            .with_color(Color32::WHITE)
            .with_col_a(Color32::WHITE)
            .with_col_b(Color32::LIGHT_BLUE)
            .with_light_color(Color32::WHITE)
            .with_light_absorption_through_cloud(0.6)
            .with_light_absorption_toward_sun(0.6)
            .with_phase_params(Vec4::new(0.00, 0.48, 0.37, 0.99))
            .with_darkness_threshold(0.35)
            .with_edge_distance(1.0)
            .with_ray_offset_strength(0.0)
            .with_volume_offset(0.0)
            .with_height_map_factor(2.0)
            .with_clouds_offset(Vec3::new(0.0, 0.0, 0.0))
            .with_weather_noise(
                PerlinBuilder::new()
                    .with_num_points_a(1)
                    .with_num_points_b(1)
                    .with_num_points_c(5)
                    .with_tile(1.0)
                    .with_resolution(16)
                    .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
                    .with_persistence(0.3)
                    .with_invert_noise(true),
            );

        let mut executor = Facade::default();
        // executor.exec(SceneCommand::AddObject("grid", Grid::new(10, 1.0).into()));

        executor.exec(CameraCommand::SetCamera(Camera::default()));
        executor.exec(SceneCommand::AddObject(
            "cloud",
            cloud_params.build().into(),
        ));
        
        let sun = Sun::new(10.0, -90.0, -90.0);
        executor.exec(SceneCommand::AddObject(
            "sun",
            sun.into(),
        ));

        let terrain_params = TerrainBuilder::default()
            .with_bounding_box((Vec3::new(-2.5, 0.0, -2.5), Vec3::new(2.5, 0.5, 2.5)))
            .with_scale(65)
            .with_bottom_color(Color32::from_rgb(181, 255, 182))
            .with_top_color(Color32::from_rgb(134, 167, 134))
            .with_density_scale(75.0)
            .with_diffuse_factor(0.55)
            .with_num_shadows_steps(10)
            .with_shadow_threshold(0.65)
            .with_noise(
                PerlinBuilder::new()
                    .with_seed(100)
                    .with_num_points_a(1)
                    .with_num_points_b(2)
                    .with_num_points_c(5)
                    .with_tile(1.0)
                    .with_resolution(64)
                    .with_color_mask(Vec4::new(1.0, 1.0, 1.0, 1.0))
                    .with_persistence(1.3)
                    .with_invert_noise(true),
            );
        executor.exec(SceneCommand::AddObject(
            "terrain",
            terrain_params.build().into(),
        ));

        Self {
            executor,
            noise_mode: Default::default(),
            cloud: cloud_params,
            terrain: terrain_params,
            background_color: Color32::LIGHT_BLUE,
            offset_speed: Vec3::new(1.0, 0.0, 1.0),
            sun: (sun.d, sun.a.abs(), sun.z.abs()),
            move_vector: Vec3::ZERO,
            extend_vector: Vec3::ZERO
        }
    }
}
