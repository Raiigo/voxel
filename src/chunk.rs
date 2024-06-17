use std::mem;
use bevy::{math::{UVec3, Vec3}, prelude::Component, render::mesh::Mesh};

use crate::voxel::{Voxel, VoxelSet};

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 128;

#[derive(Debug, Component)]
pub struct ChunkMarker;
pub struct Chunk<T: VoxelSet> {
    voxels: [[[T::Id; WIDTH]; HEIGHT]; WIDTH],
}

impl<T: VoxelSet> Chunk<T> {

    /// Create chunk from voxel list
    pub fn new(voxels: [[[T::Id; WIDTH]; HEIGHT]; WIDTH]) -> Self {
        Self {
            voxels: voxels
        }
    }

    /// Return voxel in chunk coordinates
    pub fn get_voxel(&self, pos: UVec3) -> Voxel {
        return T::get_voxel_by_id(self.voxels[pos.x as usize][pos.y as usize][pos.z as usize]);
    }

    /// Return voxel id in chunk coordinates
    pub fn get_voxel_id(&self, pos: UVec3) -> T::Id {
        return self.voxels[pos.x as usize][pos.y as usize][pos.z as usize];
    }

    /// Clone chunk content into a new dynamic array
    pub fn clone_voxels(&self) -> Vec<Vec<Vec<T::Id>>> {
        let mut volume = Vec::<Vec<Vec<T::Id>>>::new();
        for x in 0..WIDTH {
            let mut surface = Vec::<Vec<T::Id>>::new();
            for y in 0..HEIGHT {
                let mut line = Vec::<T::Id>::new();
                for z in 0..WIDTH {
                    line.push(self.get_voxel_id(UVec3::new(x as u32, y as u32, z as u32)));
                }
                surface.push(line);
            }
            volume.push(surface);
        }
        return volume;
    }
}