use events::errors as events;
use gfx;
use texture::errors as texture;

error_chain! {
    foreign_links {
        GltfImporter(::gltf_importer::Error);
        BorrowMutError(::std::cell::BorrowMutError);
        BorrowError(::std::cell::BorrowError);
        IoError(::std::io::Error);
        Gltf(::gltf::Error);
        Events(events::Error);
        Texture(texture::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }

    links {
        Gfx(gfx::errors::Error, gfx::errors::ErrorKind);
    }

    errors {
        NoMesh {
        }

        NoPrimitive {
        }

        NoPositions {
        }

        NoNormals {
        }

        NoIndices {
        }

        ThreadJoin {
        }

        PoisonError {
        }

        NoNode(name: &'static str) {
            description("no node")
            display("no node: {}", name)
        }
    }
}
