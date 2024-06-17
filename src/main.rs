use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::{app::{App, Startup}, math::Vec3A, pbr::{wireframe::{NoWireframe, Wireframe, WireframeConfig, WireframePlugin}, MaterialMeshBundle}, prelude::Commands, render::{color::Color, primitives::Sphere, settings::{RenderCreation, WgpuFeatures, WgpuSettings}, RenderPlugin}, DefaultPlugins};
use camera::CameraPlugin;
use chunk::{Chunk, ChunkMarker};
use octree::Octree;
use rand::Rng;
use voxel::{Orientation, Voxel, VoxelSet};
use bevy::prelude::*;
use world::VoxelWorld;

pub mod chunk;
pub mod voxel;
pub mod octree;
pub mod world;
pub mod camera;

pub struct BasicSet;

impl VoxelSet for BasicSet {
    type Id = u8;

    fn get_voxel_by_id(voxel_id: Self::Id) -> crate::voxel::Voxel {
        match voxel_id {
            0 => {
                Voxel::Air
            },
            1 => {
                Voxel::Grass
            },
            _ => {
                Voxel::Error
            },
        }
    }
    
    fn is_transparent(voxel_id: Self::Id) -> bool {
        match voxel_id {
            0 => true,
            _ => false,
        }
    }
    
    fn get_default_voxel_id() -> Self::Id {
        0
    }
}

fn main() {
    let mut chunk_content = [[[0; 16]; 128]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                chunk_content[x][y][z] = rand::thread_rng().gen_range(0..2);
            }
        }
    }

    println!("{}", (-1_i32).rem_euclid(16));
    
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
        ))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE.into(),
        })
        .add_plugins(CameraPlugin::new(1.0, 0.0, 1.0, 0).with_rotation_speed(0.005).with_translation_speed(0.2))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(Update, display_vertex_count)
        // .add_systems(Update, display_chunk_coordinates)
        .run()

}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {


    let mut world: VoxelWorld<BasicSet> = VoxelWorld::new([vec![vec![]], vec![vec![]], vec![vec![]], vec![vec![]]]);

    let mut quadrant = vec![];

    for i in 0..10 {
        let mut line = vec![];
        for j in 0..10 {
            let mut chunk_content = [[[0; 16]; 128]; 16];
            for x in 0..16 {
                for y in 0..128 {
                    for z in 0..16 {
                        // chunk_content[x][y][z] = rand::thread_rng().gen_range(0..2);
                        chunk_content[x][y][z] = 1;
                    }
                }
            }
            let chunk = Chunk::<BasicSet>::new(chunk_content);
            line.push(chunk);
        }
        quadrant.push(line);
    }

    dbg!(&quadrant.len());

    world.quadrants[0] = quadrant;

    for i in 0..10 {
        for j in 0..10 {
            let chunk_mesh = world.create_chunk_mesh(IVec2::new(i, j));

            commands.spawn((PbrBundle {
                mesh: meshes.add(chunk_mesh),
                material: materials.add(Color::RED),
                transform: Transform::from_xyz((chunk::WIDTH as i32 * i) as f32, 0.0, (chunk::WIDTH as i32 * j) as f32),
                ..Default::default()
            }, Wireframe, ChunkMarker));
        }
    }


    let mesh = Voxel::Air.get_mesh();

    

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_to(Vec3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        ..Default::default()
    });

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..Default::default()
    // });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid { half_size: Vec3::new(0.1, 0.1, 0.1) }),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid { half_size: Vec3::new(0.1, 0.1, 0.1) }),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid { half_size: Vec3::new(0.1, 0.1, 0.1) }),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid { half_size: Vec3::new(0.1, 0.1, 0.1) }),
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });
}

fn display_chunk_coordinates(q_chunks: Query<&Transform, With<ChunkMarker>>) {
    for transform in q_chunks.iter() {
        info!("Chunk coord : {:?}", transform.translation);
    }
}

fn update(q_camera_transform: Query<&Transform, With<Camera3d>>) {
    let camera_transform = q_camera_transform.iter().nth(0).unwrap();
    let rot_y = camera_transform.rotation.to_euler(EulerRot::YXZ).0.rem_euclid(2.0 * PI);
    let translation = camera_transform.translation;
    if (rot_y >= 0.0 && rot_y < FRAC_PI_4) || (rot_y >= 7.0 * FRAC_PI_4 && rot_y < 2.0 * PI) {
        // info!("Facing Z- West");
    }
    if rot_y >= FRAC_PI_4 && rot_y < 3.0 * FRAC_PI_4 {
        // info!("Facing X- South");
    }
    if rot_y >= 3.0 * FRAC_PI_4 && rot_y < 5.0 * FRAC_PI_4 {
        // info!("Facing Z+ East");
    }
    if rot_y >= 5.0 * FRAC_PI_4 && rot_y < 7.0 * FRAC_PI_4 {
        // info!("Facing X+ North");
    }
    // info!("{}", translation);
}

fn display_vertex_count(
    meshes: Res<Assets<Mesh>>,
    q_meshes: Query<(&Handle<Mesh>, &ViewVisibility)>
) {
    let mut counter = 0;
    for (mesh_handle, vis) in q_meshes.iter() {
        if vis.get() {
            if let Some(mesh) = meshes.get(mesh_handle) {
                counter += mesh.count_vertices();
            }
        }
    }
    info!("Vertices : {}", counter);
}

#[cfg(test)]
mod test {

    #[test]
    fn it_works() {
        
    }
}