#[derive(Debug, PartialEq)]
pub(crate) enum Variant {
    O,
    I,
    S,
    Z,
    L,
    J,
    T,
}

impl Variant {
    pub const ALL: [Variant; 7] = [
        Variant::O,
        Variant::I,
        Variant::S,
        Variant::Z,
        Variant::L,
        Variant::J,
        Variant::T,
    ];

    /// Returns the shape matrix associated with each variant.
    pub fn default_layout(&self) -> Vec<Vec<u8>> {
        match self {
            Variant::O => vec![vec![1, 1], vec![1, 1]],
            Variant::I => vec![vec![1, 1, 1, 1]],
            Variant::S => vec![vec![0, 1, 1], vec![1, 1, 0]],
            Variant::Z => vec![vec![1, 1, 0], vec![0, 1, 1]],
            Variant::L => vec![vec![1, 0, 0], vec![1, 1, 1]],
            Variant::J => vec![vec![0, 0, 1], vec![1, 1, 1]],
            Variant::T => vec![vec![0, 1, 0], vec![1, 1, 1]],
        }
    }
}
