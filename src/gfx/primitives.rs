use super::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct Primitives {
    pub primitives: Vec<Primitive>,
}

impl Primitives {
    pub fn new(primitives: Vec<Primitive>) -> Primitives {
        Primitives { primitives: primitives }
    }
}
