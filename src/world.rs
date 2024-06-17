use bevy::{math::{IVec2, IVec3, UVec3, Vec3}, render::{mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}};

use crate::{chunk::{self, Chunk, HEIGHT, WIDTH}, voxel::{Orientation, Voxel, VoxelSet}};

pub struct VoxelWorld<T: VoxelSet> {
    /// Sorted in trigonometric order
    pub quadrants: [Vec<Vec<Chunk<T>>>; 4],
}

impl<T: VoxelSet> VoxelWorld<T> {
    pub fn new(quadrants: [Vec<Vec<Chunk<T>>>; 4]) -> Self {
        Self {
            quadrants: quadrants,
        }
    }

    fn chunk_pos_signature(pos: IVec2) -> (bool, bool) {
        return (pos.x >= 0, pos.y >= 0);
    }

    fn get_quadrant(&self, pos: IVec2) -> &Vec<Vec<Chunk<T>>> {
        let (x, y) = Self::chunk_pos_signature(pos);
        if x {
            if y {
                return &self.quadrants[0];
            } else {
                return &self.quadrants[3];
            }
        } else {
            if y {
                return &self.quadrants[1];
            } else {
                return &self.quadrants[2];
            }
        }
    }

    pub fn get_chunk(&self, pos: IVec2) -> Option<&Chunk<T>> {
        let quadrant = self.get_quadrant(pos);

        if quadrant.len() > pos.x.abs() as usize {
            if quadrant[pos.x.abs() as usize].len() > pos.y.abs() as usize {
                return Some(&quadrant[pos.x.abs() as usize][pos.y.abs() as usize]);
            }
        }

        return None;
    }

    pub fn get_voxel(&self, pos: IVec3) -> Voxel {
        return T::get_voxel_by_id(self.get_voxel_id(pos))
    }

    pub fn get_voxel_id(&self, pos: IVec3) -> T::Id {
        let chunk_pos = IVec2::new(pos.x.div_euclid(chunk::WIDTH as i32), pos.z.div_euclid(chunk::WIDTH as i32));

        if pos.y < 0 || pos.y >= chunk::HEIGHT as i32 {
            return T::get_default_voxel_id();
        }

        let voxel_pos_in_chunk = UVec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as u32, pos.y as u32, pos.z.rem_euclid(chunk::WIDTH as i32) as u32);
        // dbg!(voxel_pos_in_chunk);
        if let Some(chunk) = self.get_chunk(chunk_pos) {
            return chunk.get_voxel_id(voxel_pos_in_chunk);
        } else {
            return T::get_default_voxel_id();
        }
    }

    pub fn create_chunk_mesh(&self, pos: IVec2) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
            .with_inserted_indices(Indices::U32(vec![]));
        for x in 0..WIDTH {
            let x = pos.x * WIDTH as i32 + x as i32;
            for y in 0..HEIGHT {
                let y = y as i32;
                for z in 0..WIDTH {
                    let z = pos.y * WIDTH as i32 + z as i32;
                    
                    let pos = IVec3::new(x, y, z);
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x + 1, y, z))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::North).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x - 1, y, z))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::South).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x, y, z + 1))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::East).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x, y, z - 1))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::West).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x, y + 1, z))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::Up).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                    if T::is_transparent(self.get_voxel_id(IVec3::new(x, y - 1, z))) {
                        mesh.merge(self.get_voxel(pos).get_face_mesh(Orientation::Down).translated_by(Vec3::new(pos.x.rem_euclid(chunk::WIDTH as i32) as f32, pos.y as f32, pos.z.rem_euclid(chunk::WIDTH as i32) as f32)));
                    }
                }
            }
        }
        return mesh.with_duplicated_vertices().with_computed_flat_normals();
    }
}