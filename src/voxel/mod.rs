use std::fmt::Debug;
use crate::math::linear_algebra::types::Vec3;
use crate::MeshBuilder;

pub type ChunkMesh<T: Clone + Debug> = (Chunk<T>, MeshBuilder);
pub type PosRot = [Vec3; 2];

#[derive(Debug, Clone)]
pub struct Chunk<T: Debug + Clone> {
    position: Vec3,
    data: Vec<Vec<Vec<Option<T>>>>,
}

impl<T: Debug + Clone> Chunk<T> {
    pub fn new(position: Vec3, size: [usize; 3]) -> Chunk<T> {
        Self {
            position,
            data: vec![vec![vec![None; size[2]]; size[1]]; size[0]],
        }
    }

    pub fn get(&self, xyz: [usize; 3]) -> &Option<T> {
        &self.data[xyz[0]][xyz[1]][xyz[2]]
    }

    pub fn get_mut(&mut self, xyz: [usize; 3]) -> &mut Option<T> {
        &mut self.data[xyz[0]][xyz[1]][xyz[2]]
    }

    pub fn set(&mut self, xyz: [usize; 3], value: Option<T>) {
        *self.get_mut(xyz) = value;
    }
}

pub struct Voxel<T: Debug + Clone> {
    position: Vec3,
    chunk: Vec<ChunkMesh<T>>,
}

impl<T: Debug + Clone> Voxel<T> {
    pub fn new(size: [usize; 3], chunk_size: [usize; 3], position: Vec3) -> Voxel<T> {
        let x = size[0] / chunk_size[0];
        let y = size[1] / chunk_size[1];
        let z = size[2] / chunk_size[2];

        let mut chunks = vec![];

        for x in 0..x {
            for y in 0..y {
                for z in 0..z {
                    let position = [(x * chunk_size[0]) as f32 + position[0], (y * chunk_size[1]) as f32 + position[1], (z * chunk_size[2]) as f32 + position[2]];
                    chunks.push((Chunk::<T>::new(position, chunk_size), MeshBuilder::new()));
                }
            }
        }
        Self {
            position,
            chunk: vec![]
        }
    }
    pub fn add_face(&mut self, _value: T, _position: [usize; 3]) {

    }
}