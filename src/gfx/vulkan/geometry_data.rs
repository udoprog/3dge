use super::geometry_entry::GeometryEntry;

pub struct GeometryData {
    pub entries: Vec<GeometryEntry>,
}

impl GeometryData {
    pub fn new() -> GeometryData {
        GeometryData { entries: Vec::new() }
    }

    pub fn push(&mut self, entry: GeometryEntry) {
        self.entries.push(entry);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
