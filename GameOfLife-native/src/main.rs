extern crate ggez;
extern crate mio;
// extern crate rand;
extern crate rayon;
use ggez::{
    *,
    graphics::{
        DrawMode,
        Color,
        Drawable,
        MeshBuilder
    },
    event::*
};
use mio::{
    Poll, PollOpt, Ready, Token, Events,
    unix::EventedFd
};
mod grid;
mod cell;
mod sim;
mod rle;

use std::{
    env,
    io::{
        self,
        prelude::*,
    },
    fs::File,
    time::Duration,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Task {
    Step
}

struct MainState {
    grid: grid::Grid,
    loaded_grid: Vec<Vec<cell::Cell>>,
    loaded: bool,
    width: u32,
    height: u32,
    sqr_size: u32,
    curx: u32,
    cury: u32,
    update_queue: Vec<Task>,
    _poll: Poll,
    _mousing: bool,
    _running: bool,
}

impl MainState {
    fn new(ctx: &mut Context, rle_filename: String) -> GameResult<Self> {
        graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, 800f32, 800f32));
        let mut loaded_grid = Vec::new();
        let mut loaded = false;
        if !rle_filename.is_empty() {
            loaded_grid = rle::RLE::load(rle_filename).unwrap().into_inner();
            loaded = true;
        }

        // Create poll and register stdin event.
        let mut poll = Poll::new().unwrap();
        // Get fd of stdin
        use std::os::unix::io::AsRawFd;
        let raw_fd = io::stdin().as_raw_fd();
        let evented_fd = EventedFd(&raw_fd);
        poll.register(&evented_fd, Token(0), Ready::readable(), PollOpt::level()).unwrap();

        let sqr_size = 1;
        let (width, height) = (800, 800);
        let grid = grid::Grid::new(width/sqr_size, height/sqr_size);

        let win_size = graphics::get_size(ctx);
        let s = Self {
            width,
            height,
            sqr_size,
            curx: 0,
            cury: 0,
            grid,
            loaded_grid,
            loaded,
            update_queue: Vec::new(),
            _poll: poll,
            _mousing: false,
            _running: false,
        };
        println!("Actual window size: {} x {}", win_size.0, win_size.1);
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let clone_queue = self.update_queue.clone();
        let mut queue_iter = clone_queue.iter();
        while let Some(task) = queue_iter.next() {
            match task {
                Step => self.step(),
            }
        }
        if self._running {
            self.step();
        }
        let mut ev = Events::with_capacity(1024);
        self._poll.poll(&mut ev, Some(Duration::from_secs(0)));
        for ev in ev.iter() {
            match ev.token() {
                Token(0) => {
                    // Read from stdin?
                    let mut s = String::new();
                    let read = io::stdin().read_line(&mut s);
                    let bytes = b"> ";
                    let stdout = io::stdout();
                    let mut stdout = stdout.lock();
                    stdout.write(bytes);
                    stdout.flush();
                },
                _ => unreachable!(),
            }
        }
        {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            write!(stdout, "\rFPS: {}", timer::get_fps(_ctx));
            stdout.flush();
        }
        self.update_queue.clear();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let sqr_size = self.sqr_size;
        let (ww, wh) = graphics::get_size(ctx);
        graphics::set_color(ctx, Color::new(0f32, 0f32, 0f32, 1f32))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, Rect::new(
            0f32, 0f32, ww as f32, wh as f32
        ))?;
        // Draw each alive square
        graphics::set_color(ctx, Color::new(0f32, 1f32, 0f32, 1f32))?;
        for (x, y) in self.grid.alive() {
            // println!("Got cell: {0} @ ({1}, {2})", cell, x, y);
            graphics::rectangle(ctx, graphics::DrawMode::Fill, Rect::new(
                (x*sqr_size) as f32,
                (y*sqr_size) as f32,
                sqr_size as f32, sqr_size as f32
            ))?;
        }
        // Draw cursor.
        if self.grid.is_alive(self.curx, self.cury) {
            graphics::set_color(ctx, Color::new(0f32, 0f32, 0f32, 0.5));
        } else {
            graphics::set_color(ctx, Color::new(0f32, 1f32, 0f32, 0.3));
        }
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                (self.curx*sqr_size) as f32, (self.cury*sqr_size) as f32,
                sqr_size as f32, sqr_size as f32
        ))?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up | Keycode::K => {
                if self.cury > 0 {
                    self.cury -= 1;
                }
            },
            Keycode::Down | Keycode::J => {
                if self.cury < self.grid.height()-1 {
                    self.cury += 1;
                }
            },
            Keycode::Left | Keycode::H => {
                if self.curx > 0 {
                    self.curx -= 1;
                }
            },
            Keycode::Right | Keycode::L => {
                if self.curx < self.grid.width()-1 {
                    self.curx += 1;
                }
            },
            Keycode::T => {
                for _ in 0..10 {
                    self.update_queue.push(Task::Step);
                }
            },
            Keycode::B => self._running = !self._running,
            Keycode::KpEnter | Keycode::Return | Keycode::Return2 | Keycode::Space => {
                self.grid.toggle(self.curx, self.cury);
            },
            Keycode::Tab => {
                self.step();
            },
            _ => {},
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        // Convert mouse coordinate to grid x,y
        let (x, y) = self.grid.grid_pos_for_mouse_pos(self.sqr_size, x, y);
        match button {
            MouseButton::Left => {
                self.curx = x;
                self.cury = y;
                if self.loaded {
                    self.load_grid(x, y);
                    self.loaded = false;
                }
            },
            MouseButton::Right => self.grid.toggle(x, y),
            _ => {},
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        if (width%8 + height%8) != 0 {
            let default_mode: conf::WindowMode = Default::default();
            let new_gwidth = width/self.sqr_size;
            let new_gheight = height/self.sqr_size;
            let old_gwidth = self.grid.width();
            let old_gheight = self.grid.height();
            let new_width = new_gwidth * self.sqr_size;
            let new_height = new_gheight * self.sqr_size;
            graphics::set_mode(ctx, default_mode.dimensions(new_width, new_height));
            // Resize grid?
            self.grid.resize((new_gheight - old_gheight) as i32, (new_gwidth - old_gwidth) as i32);
            self.width = new_width;
            self.height = new_height;
            graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, new_width as f32, new_height as f32));
            println!("New grid width, height: {}, {}", self.grid.width(), self.grid.height());
        }
    }

}

impl MainState {
    fn step(&mut self) {
        self.grid.simulate();
    }

    fn load_grid(&mut self, x: u32, y: u32) {
        for (yy, r) in self.loaded_grid.iter().enumerate() {
            for (xx, c) in r.iter().enumerate() {
                if c == &cell::Cell::Alive {
                    self.grid.set((xx as u32) + x, (yy as u32) + y, cell::Cell::Alive);
                }
            }
        }
    }
}

fn main() {
    let c = conf::Conf {
        window_setup: conf::WindowSetup {
            title: "gol".to_owned(),
            icon: "".to_owned(),
            resizable: true,
            allow_highdpi: true,
            samples: conf::NumSamples::Two,
        },
        backend: conf::Backend::default(),
        window_mode: conf::WindowMode {
            width: 800,
            height: 800,
            .. Default::default()
        }
    };
    let ctx = &mut Context::load_from_conf("game_of_life", "ggez", c).unwrap();
    let rle_filename = std::env::args().nth(1).unwrap_or("".to_owned());
    let state = &mut MainState::new(ctx, rle_filename).unwrap();
    {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        stdout.write(b"> ");
        stdout.flush();
    }
    event::run(ctx, state).unwrap();
}
