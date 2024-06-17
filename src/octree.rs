use core::panic;
use std::marker::PhantomData;

use crate::{chunk::Chunk, voxel::VoxelSet};

#[derive(Debug)]
pub enum Octree<T: PartialEq + Clone + ToString> {
    Subdivised {
        childs: [Box<Octree<T>>; 8],
    },
    Uniform {
        content: T,
    }
}

impl<T: PartialEq + Clone + Copy + ToString> Octree<T> {
    pub fn string_repr(&self) -> String {
        let string = String::new();
        match self {
            Octree::Subdivised { childs } => return format!("S({},{},{},{},{},{},{},{})", childs[0].string_repr(), childs[1].string_repr(), childs[2].string_repr(), childs[3].string_repr(), childs[4].string_repr(), childs[5].string_repr(), childs[6].string_repr(), childs[7].string_repr()),
            Octree::Uniform { content } => return format!("{}", content.to_string()),
        }
    }
}

impl<T: PartialEq + Clone + Copy + ToString> From<Vec<Vec<Vec<T>>>> for Octree<T> {
    fn from(value: Vec<Vec<Vec<T>>>) -> Self {
        let lim_x = value.len();
        let lim_y = value[0].len();
        let lim_z = value[0][0].len();

        if lim_x != lim_y || lim_x != lim_z {
            panic!("ERROR: Can't create Octree from non cubic volume")
        }

        let half_x = lim_x / 2 as usize;
        let half_y = lim_y / 2 as usize;
        let half_z = lim_z / 2 as usize;

        // Check if volume is uniform (contains the same value)
        if value.iter().map(|l| { l.iter().flatten().collect::<Vec<_>>() }).flatten().all(|e| { *e == value[0][0][0] }) {
            return Octree::<T>::Uniform { content: value[0][0][0] }
        } else {
            let mut octant_000 = vec![vec![vec![value[0][0][0]; half_z]; half_y]; half_x];
            let mut octant_001 = vec![vec![vec![value[0][0][0]; lim_z - half_z]; half_y]; half_x];
            let mut octant_010 = vec![vec![vec![value[0][0][0]; half_z]; half_y]; lim_x - half_x];
            let mut octant_011 = vec![vec![vec![value[0][0][0]; lim_z - half_z]; half_y]; lim_x - half_x];
            let mut octant_100 = vec![vec![vec![value[0][0][0]; half_z]; lim_y - half_y]; half_x];
            let mut octant_101 = vec![vec![vec![value[0][0][0]; lim_z - half_z]; lim_y - half_y]; half_x];
            let mut octant_110 = vec![vec![vec![value[0][0][0]; half_z]; lim_y - half_y]; lim_x - half_x];
            let mut octant_111 = vec![vec![vec![value[0][0][0]; lim_z - half_z]; lim_y - half_y]; lim_x - half_x];

            for x in 0..lim_x {
                for y in 0..lim_y {
                    for z in 0..lim_z {
                        let e = value[x][y][z];
                        if x < half_x {
                            if y < half_y {
                                if z < half_z {
                                    octant_000[x % half_x][y % half_y][z % half_z] = e;
                                } else {
                                    octant_001[x % half_x][y % half_y][z % half_z] = e;
                                }
                            } else {
                                if z < half_z {
                                    octant_010[x % half_x][y % half_y][z % half_z] = e;
                                } else {
                                    octant_011[x % half_x][y % half_y][z % half_z] = e;
                                }
                            }
                        } else {
                            if y < half_y {
                                if z < half_z {
                                    octant_100[x % half_x][y % half_y][z % half_z] = e;
                                } else {
                                    octant_101[x % half_x][y % half_y][z % half_z] = e;
                                }
                            } else {
                                if z < half_z {
                                    octant_110[x % half_x][y % half_y][z % half_z] = e;
                                } else {
                                    octant_111[x % half_x][y % half_y][z % half_z] = e;
                                }
                            }
                        }
                    }
                }
            }
            return Octree::<T>::Subdivised { childs: [
                Box::new(Octree::<T>::from(octant_000)),
                Box::new(Octree::<T>::from(octant_001)),
                Box::new(Octree::<T>::from(octant_010)),
                Box::new(Octree::<T>::from(octant_011)),
                Box::new(Octree::<T>::from(octant_100)),
                Box::new(Octree::<T>::from(octant_101)),
                Box::new(Octree::<T>::from(octant_110)),
                Box::new(Octree::<T>::from(octant_111)),
            ] }
        }

        todo!()

    }
}