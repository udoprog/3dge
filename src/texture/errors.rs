error_chain!{
    foreign_links {
        ImageError(::image::ImageError);
    }
}
