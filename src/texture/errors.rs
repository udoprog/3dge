error_chain!{
    foreign_links {
        ImageError(::image::ImageError);
        Io(::std::io::Error);
    }

    errors {
        UnsupportedMimeType(mime_type: String) {
            description("unsupported mime type")
            display("Unsupported mime type: {}", mime_type)
        }

        UnsupportedExtension(ext: Option<String>) {
            description("unsupported extension")
            display("unsupported extension: {:?}", ext)
        }
    }
}
