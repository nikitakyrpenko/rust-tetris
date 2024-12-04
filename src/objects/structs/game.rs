use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::objects::enums::control::Control;
use crate::objects::enums::variant::Variant;
use crate::objects::structs::grid::Grid;
use crate::objects::structs::tetramino::Tetramino;

pub(crate) struct Game {
    grid: Grid,
}

impl Game {
    pub fn init() -> Self {
        Game { grid: Grid::init() }
    }

    pub fn run(&mut self, rx: Receiver<Control>) {
        loop {
            //if cannot attach then game is over
            self.grid
                .attach(Tetramino::init_random((0, self.grid.l / 2 - 1)));

            while self.grid.is_not_collide() {
                self.grid.clear();
                self.grid.gravitate();
                self.grid.snapshot();
                println!("{:?}", self.grid);
                thread::sleep(Duration::from_millis(500));

                if let Ok(c) = rx.try_recv() {
                    self.grid.clear();
                    self.grid.mv(c);
                    self.grid.snapshot();
                    println!("{:?}", self.grid);
                }
            }
            self.grid.snapshot();
            self.grid.detach();
        }
    }
}
