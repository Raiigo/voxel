use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        component::Component,
        event::EventReader,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, mouse::MouseMotion, ButtonInput},
    log::info,
    math::Vec3,
    transform::components::Transform,
};

#[derive(Debug, Component)]
pub struct CameraId(u32);

pub struct CameraPlugin {
    x: f32,
    y: f32,
    z: f32,
    id: u32,
    translation_speed: f32,
    rotation_speed: f32,
    forward: KeyCode,
    backward: KeyCode,
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let id: u32 = self.id;
        let forward_key = self.forward;
        let backward_key = self.backward;
        let left_key = self.left;
        let right_key = self.right;
        let up_key = self.up;
        let down_key = self.down;
        let translation_speed = self.translation_speed;
        let rotation_speed = self.rotation_speed;
        app.add_systems(Startup, move |mut commands: Commands| {
            commands
                .spawn(Camera3dBundle {
                    transform: Transform::from_xyz(x, y, z),
                    ..Default::default()
                })
                .insert(CameraId(id));
        })
        .add_systems(
            Update,
            move |mut cameras: Query<(&mut Transform, &CameraId)>,
                  inputs: Res<ButtonInput<KeyCode>>,
                  mut mouse_motion_events: EventReader<MouseMotion>| {
                for (mut camera_transform, camera_id) in cameras.iter_mut() {
                    if camera_id.0 == id {
                        // log::info!("Current pos : {}, {}, {}", camera_transform.translation.x, camera_transform.translation.y, camera_transform.translation.z);
                        // info!("Rotation around y : {}", rotate_around_y);
                        if inputs.pressed(forward_key) {
                            let mut forward = camera_transform.forward().normalize();
                            forward.y = 0.0;
                            forward = forward.normalize();
                            camera_transform.translation += forward * translation_speed;
                        }
                        if inputs.pressed(backward_key) {
                            let mut backward = camera_transform.back().normalize();
                            backward.y = 0.0;
                            backward = backward.normalize();
                            camera_transform.translation += backward * translation_speed;
                        }
                        if inputs.pressed(left_key) {
                            let left = camera_transform.left();
                            camera_transform.translation += left * translation_speed;
                        }
                        if inputs.pressed(right_key) {
                            let right = camera_transform.right();
                            camera_transform.translation += right * translation_speed;
                        }
                        if inputs.pressed(up_key) {
                            // let up = camera_transform.up();
                            camera_transform.translation +=
                                Vec3::new(0.0, 1.0, 0.0) * translation_speed;
                        }
                        if inputs.pressed(down_key) {
                            // let down = camera_transform.down();
                            camera_transform.translation +=
                                Vec3::new(0.0, -1.0, 0.0) * translation_speed;
                        }

                        for event in mouse_motion_events.read() {
                            // info!("Mouse event : {:?}", event);
                            // info!(
                            //     "Euler angle : {} * PI",
                            //     camera_transform
                            //         .rotation
                            //         .to_euler(bevy::math::EulerRot::YXZ)
                            //         .1
                            //         / PI
                            // );
                            // info!("Add : {}", event.delta.x * rotation_speed);
                            // info!("Axis, angle : {}, {}", camera_transform.rotation.to_axis_angle().0, camera_transform.rotation.to_axis_angle().1);
                            // Change detection mechanism, it doesn't works
                            if (camera_transform
                                .rotation
                                .to_euler(bevy::math::EulerRot::YXZ)
                                .1
                                - event.delta.y * rotation_speed)
                                .abs()
                                < FRAC_PI_2
                            {
                                // info!(
                                //     "Test passed, angle : {}, PI/2 : {}",
                                //     camera_transform
                                //         .rotation
                                //         .to_euler(bevy::math::EulerRot::YXZ)
                                //         .1,
                                //     FRAC_PI_2
                                // );
                                camera_transform.rotate_local_x(-event.delta.y * rotation_speed);
                            }
                            camera_transform.rotate_y(-event.delta.x * rotation_speed);
                        }
                        break;
                    }
                }
            },
        );
    }
}

impl CameraPlugin {
    pub fn new(x: f32, y: f32, z: f32, id: u32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            id: id,
            translation_speed: 1.0,
            rotation_speed: 1.0,
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::Space,
            down: KeyCode::ControlLeft,
        }
    }

    pub fn with_forward_key(mut self, key: KeyCode) -> Self {
        self.forward = key;
        return self;
    }
    pub fn with_backward_key(mut self, key: KeyCode) -> Self {
        self.backward = key;
        return self;
    }
    pub fn with_left_key(mut self, key: KeyCode) -> Self {
        self.left = key;
        return self;
    }
    pub fn with_right_key(mut self, key: KeyCode) -> Self {
        self.right = key;
        return self;
    }
    pub fn with_up_key(mut self, key: KeyCode) -> Self {
        self.up = key;
        return self;
    }
    pub fn with_down_key(mut self, key: KeyCode) -> Self {
        self.down = key;
        return self;
    }
    pub fn with_translation_speed(mut self, translation_speed: f32) -> Self {
        self.translation_speed = translation_speed;
        return self;
    }
    pub fn with_rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.rotation_speed = rotation_speed;
        return self;
    }
}
