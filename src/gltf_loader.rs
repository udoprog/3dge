use super::errors::*;
use super::model::Model;
use gltf::{Gltf, Mesh};
use gltf_importer::{self, Buffers, Config};
use gltf_importer::config::ValidationStrategy;
use std::path::{Path, PathBuf};

pub struct GltfLoader {
    gltf: Gltf,
    path: PathBuf,
    buffers: Buffers,
}

impl GltfLoader {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<GltfLoader> {
        let path = path.as_ref().to_owned();
        let config = Config { validation_strategy: ValidationStrategy::Complete };
        let (gltf, buffers) = gltf_importer::import_with_config(&path, config)?;

        Ok(GltfLoader {
            gltf: gltf,
            path: path,
            buffers: buffers,
        })
    }

    fn model_from_mesh(&self, mesh: &Mesh) -> Result<Model> {
        Model::from_gltf(&self.path, &self.buffers, mesh)
    }

    pub fn model_from_first(&self) -> Result<Option<Model>> {
        if let Some(ref m) = self.gltf.meshes().nth(0) {
            return Ok(Some(self.model_from_mesh(m)?));
        }

        Ok(None)
    }

    pub fn model_from_node(&self, name: &str) -> Result<Option<Model>> {
        for node in self.gltf.nodes() {
            if let Some(node_name) = node.name() {
                if node_name == name {
                    if let Some(m) = node.mesh() {
                        return Ok(Some(self.model_from_mesh(&m)?));
                    }
                }
            }
        }

        Ok(None)
    }
}
