use std::fmt::Debug;

use bevy::{log::info, render::{mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}};

pub enum Orientation {
    /// X+
    North,
    /// X-
    South,
    /// Z+
    East,
    /// Z-
    West,
    /// Y+
    Up,
    /// Y-
    Down,
}

pub enum Voxel {
    Air,
    Grass,
    Error,
}

impl Voxel {

    pub fn has_faces(&self) -> bool {
        match self {
            Self::Air => false,
            _ => true,
        }
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
            .with_inserted_indices(Indices::U32(vec![]));
        mesh.merge(self.get_face_mesh(Orientation::South));
        mesh.merge(self.get_face_mesh(Orientation::North));
        mesh.merge(self.get_face_mesh(Orientation::East));
        mesh.merge(self.get_face_mesh(Orientation::West));
        mesh.merge(self.get_face_mesh(Orientation::Up));
        mesh.merge(self.get_face_mesh(Orientation::Down));
        dbg!(mesh.attribute(Mesh::ATTRIBUTE_POSITION));
        return mesh;
    }

    pub fn get_face_mesh(&self, orientation: Orientation) -> Mesh {
        if !self.has_faces() {
            return Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());;
        }
        let (pos, indices) = match orientation {
            Orientation::North => (vec![[1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]], Indices::U32(vec![0, 2, 1, 0, 3, 2])),
            Orientation::South => (vec![[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]], Indices::U32(vec![0, 1, 2, 0, 2, 3])),
            Orientation::East => (vec![[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]], Indices::U32(vec![0, 2, 1, 0, 3, 2])),
            Orientation::West => (vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]], Indices::U32(vec![0, 1, 2, 0, 2, 3])),
            Orientation::Up => (vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]], Indices::U32(vec![0, 2, 1, 0, 3, 2])),
            Orientation::Down => (vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 0.0, 1.0]], Indices::U32(vec![0, 1, 2, 0, 2, 3])),
        };
        let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
            .with_inserted_indices(indices);
        return mesh;
    }
}

pub trait VoxelSet {

    type Id: Copy + Clone + PartialEq + Debug;

    fn get_voxel_by_id(voxel_id: Self::Id) -> Voxel;

    fn is_transparent(voxel_id: Self::Id) -> bool;

    /// Used in the generation of a chunk mesh
    fn get_default_voxel_id() -> Self::Id;
}