use crate::objects::enums::control::Control;
use crate::objects::enums::direction::Direction::{DOWN, LEFT, RIGHT};
use crate::objects::enums::rotation::Rotation::CLOCKWISE;
use crate::objects::structs::tetramino::Tetramino;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub(crate) struct Grid {
    grid: [Vec<u8>; 20],
    pub h: usize,
    pub l: usize,
    pub current: Option<Tetramino>,
}

impl Grid {
    pub fn init() -> Self {
        let width = 10;
        let height = 20;

        // Initialize a grid with `height` rows, each containing a `Vec` of `width` zeroes
        let grid = std::array::from_fn(|_| vec![0; width]);

        Self {
            grid,
            h: height,
            l: width,
            current: None,
        }
    }

    pub fn attach(&mut self, t: Tetramino) {
        self.current = Some(t)
    }

    pub fn detach(&mut self) {
        self.current = None;
        self.clear_and_shift();
    }

    pub fn gravitate(&mut self) {
        match self.current.as_mut() {
            None => {}
            Some(t) => t.mv(&DOWN),
        }
    }

    pub fn is_not_collide(&self) -> bool {
        match &self.current {
            None => true,
            Some(t) => {
                let coords = t.to_coords();

                //check is tetramino reached end of grid
                if coords
                    .bottoms()
                    .into_iter()
                    .find(|coord| coord.0 + 1 == self.h)
                    .is_some()
                {
                    return false;
                }

                // check is there occupied cell under current tetramino
                if coords
                    .bottoms()
                    .into_iter()
                    .filter(|coord| self.grid[coord.0 + 1][coord.1] == 1)
                    .next()
                    .is_some()
                {
                    return false;
                }
                return true;
            }
        }
    }

    pub fn snapshot(&mut self) {
        match &(self.current) {
            None => panic!("No tetramino on the grid"),
            Some(t) => t
                .to_coords()
                .coords
                .iter()
                .for_each(|&(r, c)| self.grid[r][c] = 1),
        };
    }

    pub fn clear(&mut self) {
        match &(self.current) {
            None => panic!("No tetramino on the grid"),
            Some(t) => t
                .to_coords()
                .coords
                .iter()
                .for_each(|&(r, c)| self.grid[r][c] = 0),
        };
    }

    pub fn mv(&mut self, con: Control) {
        let mut t = match self.current.as_mut() {
            None => panic!("No tetramino on the grid"),
            Some(t) => t,
        };

        let coords = t.to_coords();

        match con {
            Control::LEFT => {
                if t.x != 0
                    && coords
                        .lefts()
                        .iter()
                        .find(|&c| self.grid[c.0][c.1 - 1] == 1)
                        .is_none()
                {
                    t.mv(&LEFT)
                }
            }
            Control::RIGHT => {
                if self.l - t.length() != t.x
                    && coords
                        .rights()
                        .iter()
                        .find(|&c| self.grid[c.0][c.1 + 1] == 1)
                        .is_none()
                {
                    t.mv(&RIGHT)
                }
            }
            Control::UP => {
                let x_offset = (t.x + t.height()) as i8 - self.l as i8;
                let y_offset = (t.y + t.length()) as i8 - self.h as i8;

                t.wall_kick((x_offset, y_offset));
                t.rotate(&CLOCKWISE)
            }
            Control::DOWN => t.mv(&DOWN),
        }
    }
    fn clear_and_shift(&mut self) {
        let mut board = &mut self.grid;
        let mut cleared_rows = 0;

        for row in (0..board.len()).rev() {
            if board[row].iter().all(|&cell| cell == 1) {
                board[row].fill(0);
                cleared_rows += 1;
            }
        }

        for row in (0..board.len() - cleared_rows).rev() {
            board[row + cleared_rows] = board[row].clone();
        }

        for i in 0..cleared_rows {
            board[i].fill(0);
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &value in row {
                match value {
                    0 => write!(f, " 0")?,
                    _ => write!(f, "[]")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

mod tests {}
