use super::errors::*;
use super::model::Model;
use gltf::{Gltf, Mesh};
use gltf_importer::{self, Buffers, Config};
use gltf_importer::config::ValidationStrategy;
use std::path::Path;

pub struct GltfLoader {
    gltf: Gltf,
    buffers: Buffers,
}

impl GltfLoader {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<GltfLoader> {
        let config = Config { validation_strategy: ValidationStrategy::Complete };
        let (gltf, buffers) = gltf_importer::import_with_config(path, config)?;

        Ok(GltfLoader {
            gltf: gltf,
            buffers: buffers,
        })
    }

    fn from_model(&self, mesh: &Mesh) -> Result<Model> {
        Model::from_gltf(&self.buffers, mesh)
    }

    pub fn model_from_first(&self) -> Result<Option<Model>> {
        if let Some(ref m) = self.gltf.meshes().nth(0) {
            return Ok(Some(self.from_model(m)?));
        }

        Ok(None)
    }

    pub fn model_from_node(&self, name: &str) -> Result<Option<Model>> {
        if let Some(ref m) = self.gltf.meshes().filter(|m| m.name() == Some(name)).nth(0) {
            return Ok(Some(self.from_model(m)?));
        }

        Ok(None)
    }
}
