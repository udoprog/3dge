use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct GeometryId(usize);

lazy_static! {
    static ref IDS: AtomicUsize = {
        AtomicUsize::new(0usize)
    };
}

impl GeometryId {
    pub fn allocate() -> GeometryId {
        GeometryId(IDS.fetch_add(1usize, Ordering::Relaxed))
    }
}
