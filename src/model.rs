use super::errors::*;
use cgmath::Point3;
use gfx::{GeometryId, Vertex};
use gfx::primitive::Primitive;
use gfx::primitives::Primitives;
use gltf::Mesh;
use gltf::image::Data;
use gltf_importer::Buffers;
use gltf_utils::PrimitiveIterators;
use std::iter;
use std::path::Path;
use texture;

#[derive(Debug)]
pub struct Model {
    id: GeometryId,
    pub location: Point3<f32>,
    pub primitives: Primitives,
}

impl Model {
    pub fn from_gltf(path: &Path, buffers: &Buffers, mesh: &Mesh) -> Result<Model> {
        let mut primitives = Vec::new();

        for p in mesh.primitives() {
            let mut positions = p.positions(buffers).ok_or(ErrorKind::NoPositions)?;
            let mut normals = p.normals(buffers).ok_or(ErrorKind::NoNormals)?;

            let mut tex_coords = p.tex_coords_f32(0, buffers)
                .map(|t| Box::new(t) as Box<Iterator<Item = [f32; 2]>>)
                .unwrap_or_else(|| Box::new(iter::repeat([0.0, 0.0])));

            let mut vertices = Vec::new();

            loop {
                match (positions.next(), normals.next(), tex_coords.next()) {
                    (Some(p), Some(normal), Some(tex_coord)) => {
                        vertices.push(Vertex {
                            position: [p[2], -p[1], -p[0]],
                            normal: normal,
                            tex_coord: tex_coord,
                        });
                    }
                    _ => {
                        break;
                    }
                }
            }

            let indices: Vec<u32> = p.indices_u32(buffers)
                .ok_or(ErrorKind::NoIndices)?
                .collect();

            let material = p.material();

            let mut color_texture = None;

            let pbr_metallic_roughness = material.pbr_metallic_roughness();

            if let Some(base_color_texture) = pbr_metallic_roughness.base_color_texture() {
                let texture = base_color_texture.texture();
                let source = texture.source();
                let data = source.data();

                match data {
                    Data::Uri { uri, mime_type } => {
                        if let Some(parent) = path.parent() {
                            let path = parent.join(uri);
                            color_texture = Some(texture::load_from_path(mime_type, &path)?);
                        }
                    }
                    Data::View { view, mime_type } => {
                        if let Some(buffer) = buffers.view(&view) {
                            color_texture = Some(texture::load_from_memory(mime_type, buffer)?);
                        }
                    }
                }
            }

            primitives.push(Primitive {
                vertices: vertices,
                color_texture: color_texture,
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
