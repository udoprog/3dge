use super::errors::*;
use cgmath::Point3;
use gfx::{GeometryId, Normal, Vertex};
use gfx::primitive::Primitive;
use gfx::primitives::Primitives;
use gltf::Mesh;
use gltf_importer::Buffers;
use gltf_utils::PrimitiveIterators;

#[derive(Debug)]
pub struct Model {
    id: GeometryId,
    pub location: Point3<f32>,
    pub primitives: Primitives,
}

impl Model {
    pub fn from_gltf(buffers: &Buffers, mesh: &Mesh) -> Result<Model> {
        let mut primitives = Vec::new();

        for p in mesh.primitives() {
            let vertices: Vec<Vertex> = {
                if let Some(positions) = p.positions(buffers) {
                    positions
                        .map(|v| [-v[0], v[1], v[2]])
                        .map(Into::into)
                        .collect()
                } else {
                    Vec::new()
                }
            };

            let normals: Vec<Normal> = p.normals(buffers)
                .ok_or(ErrorKind::NoPrimitive)?
                .map(Into::into)
                .collect();

            let indices = p.indices_u32(buffers)
                .ok_or(ErrorKind::NoIndices)?
                .collect();

            primitives.push(Primitive {
                vertices: vertices,
                normals: normals,
                indices: indices,
            });
        }

        Ok(Model {
            id: GeometryId::allocate(),
            location: Point3::new(0.0, 0.0, 0.0),
            primitives: Primitives::new(primitives),
        })
    }

    pub fn primitives(&self) -> Primitives {
        self.primitives.clone()
    }
}
