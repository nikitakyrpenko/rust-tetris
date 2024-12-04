use crate::utils;
use std::ops::Deref;

use crate::objects::enums::direction::Direction;
use crate::objects::enums::rotation::Rotation;
use crate::objects::enums::variant::Variant;
use rand::Rng;

#[derive(Debug)]
pub(crate) struct Coords {
    pub coords: Vec<(usize, usize)>,
}

impl Coords {
    pub fn bottoms(&self) -> Vec<&(usize, usize)> {
        utils::utils::group_by(&(self.coords), |&t| t.1)
            .into_iter()
            .map(|(_, v)| *(v.last().unwrap()))
            .collect()
    }

    pub fn lefts(&self) -> Vec<&(usize, usize)> {
        utils::utils::group_by(&(self.coords), |&t| t.0)
            .into_iter()
            .map(|(_, v)| *(v.first().unwrap()))
            .collect()
    }

    pub fn rights(&self) -> Vec<&(usize, usize)> {
        utils::utils::group_by(&(self.coords), |&t| t.0)
            .into_iter()
            .map(|(_, v)| *(v.last().unwrap()))
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Tetramino {
    pub coors: Vec<Vec<u8>>,
    pub x: usize,
    pub y: usize,
}

impl Tetramino {
    pub fn init(variant: Variant) -> Self {
        Tetramino {
            coors: variant.default_layout(),
            x: 0,
            y: 0,
        }
    }

    pub fn init_with_offset(variant: Variant, offset: (usize, usize)) -> Self {
        Tetramino {
            coors: variant.default_layout(),
            y: offset.0,
            x: offset.1,
        }
    }

    pub fn init_random(offset: (usize, usize)) -> Self {
        let seed = rand::thread_rng().gen_range(0..Variant::ALL.len());

        Tetramino {
            coors: Variant::ALL[seed].default_layout(),
            y: offset.0,
            x: offset.1,
        }
    }

    pub fn mv(&mut self, dir: &Direction) {
        match dir {
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::DOWN => self.y += 1,
        }
    }

    pub fn wall_kick(&mut self, offset: (i8, i8)) {
        let (x_offset, y_offset) = offset;

        if x_offset < 0 && y_offset < 0 {
            return;
        }

        // Apply the horizontal offset
        if x_offset > 0 {
            self.x = (self.x as i8 - x_offset) as usize;
        }

        // Apply the vertical offset
        if y_offset > 0 {
            self.y = (self.y as i8 - y_offset) as usize;
        }
    }

    pub fn rotate(&mut self, rot: &Rotation) {
        match rot {
            Rotation::CLOCKWISE => self.coors = self.transpose(),
            Rotation::COUNTERCLOCKWISE => {}
        }
    }

    pub fn height(&self) -> usize {
        self.coors.len()
    }

    pub fn length(&self) -> usize {
        self.coors[0].len()
    }

    pub fn to_coords(&self) -> Coords {
        let mut coords = Vec::new();

        for (i, row) in self.coors.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    coords.push((i + self.y, j + self.x))
                }
            }
        }
        Coords { coords }
    }

    fn transpose(&mut self) -> Vec<Vec<u8>> {
        let m = self.coors[0].len();
        let n = self.coors.len();

        let mut transposed = vec![vec![0; n]; m];

        for (i, row) in self.coors.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                transposed[j][n - 1 - i] = val;
            }
        }
        transposed
    }
}

mod tests {
    use super::Tetramino;
    use crate::objects::enums::rotation::Rotation::CLOCKWISE;
    use crate::objects::enums::variant::Variant;

    #[test]
    fn test_transpose_t_shape() {
        let mut t = Tetramino::init(Variant::T);
        t.rotate(&CLOCKWISE);

        assert_eq!(t.coors, vec![vec![1, 0], vec![1, 1], vec![1, 0]])
    }

    #[test]
    fn test_transpose_t_shape_twice() {
        let mut t = Tetramino::init(Variant::T);
        t.rotate(&CLOCKWISE);

        assert_eq!(t.coors, vec![vec![1, 1, 1], vec![0, 1, 0]])
    }

    #[test]
    fn test_transpose_i_shape() {
        let mut t = Tetramino::init(Variant::I);
        t.rotate(&CLOCKWISE);

        assert_eq!(
            t.coors,
            vec![vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1]]
        )
    }
}
