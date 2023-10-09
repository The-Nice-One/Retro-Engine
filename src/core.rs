

use std::io::stdout;

use std::{time::Duration, io};

use crossterm::event::{poll, read, Event};


pub struct Terminal {
    mouse: Mouse
}

impl Terminal {
    pub fn init() -> Self {

        crossterm::execute!(
            stdout(),
            crossterm::event::EnableMouseCapture
        ).unwrap();

        Self {
            mouse: Mouse {
                pressed: false,
                position: Point { x: 0, y: 0 }
            }
        }
    }
    pub fn poll_events(&mut self, timeout: u64) -> io::Result<()> {
        if poll(Duration::from_millis(timeout))? {
            match read()? {
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                _ => ()
            }
        } else {

        }
        Ok(())
    }
}

pub struct Mouse {
    pressed: bool,
    position: Point
}

pub struct Point {
    x: u32,
    y: u32
}