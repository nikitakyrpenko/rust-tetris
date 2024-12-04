use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use objects::enums::variant::Variant;
use objects::structs::tetramino::Tetramino;
use std::sync::mpsc::{channel, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::objects::enums::control::Control;
use crate::objects::structs::game::Game;

mod objects;
mod utils;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut game = Game::init();

    thread::spawn(move || read_input(tx));

    game.run(rx);
}

fn read_input(tx: Sender<Control>) {
    loop {
        if event::poll(Duration::from_millis(100)).is_ok() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                let control = match code {
                    KeyCode::Left => Control::LEFT,
                    KeyCode::Right => Control::RIGHT,
                    KeyCode::Up => Control::UP,
                    _ => continue,
                };

                // Send control to game loop
                tx.send(control).unwrap();
            }
        }
    }
}
