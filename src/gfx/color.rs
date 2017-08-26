#[derive(Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }
}

impl Copy for Color {}

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b]
    }
}
