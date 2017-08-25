use gfx::errors as gfx;

error_chain! {
    foreign_links {
        Gfx(gfx::Error);
    }

    errors {
        NoCompatibleGfxBackend {
        }
    }
}
