extern crate ggez;
extern crate rayon;
use ggez::{
    *,
    graphics::{
        DrawMode,
        Color,
        Rect,
        Drawable,
    },
    event::*
};
mod grid;
mod cell;
mod sim;

use std::{
    env,
    io::prelude::*,
    fs::File,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Task {
    Step
}

struct MainState {
    grid: grid::Grid,
    width: u32,
    height: u32,
    sqr_size: u32,
    curx: u32,
    cury: u32,
    update_queue: Vec<Task>,
    _mousing: bool,
}

impl MainState {
    fn new(ctx: &mut Context, rle_filename: String) -> GameResult<Self> {
        graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, 800f32, 800f32));
        let mut grid = grid::Grid::new(100, 100);
        if !rle_filename.is_empty() {
            // Load file and read contents.
            let mut rle_f = File::open(rle_filename).expect("Failed to open file.");
            let mut rle_s = String::new();
            rle_f.read_to_string(&mut rle_s);
            // First line.
            let mut lines = rle_s.lines();
            let (first_line, second_line) = (lines.nth(0).unwrap(), lines.nth(0).unwrap());
            println!("{}", second_line);
            let mut fls = first_line.split(",");
            // for x in fls {
            //     println!("{}", x);
            //     panic!();
            // }
            let fls1 = fls.nth(0).unwrap().clone().trim();
            let fls2 = fls.nth(0).unwrap().clone().trim();
            let (x, y) = (fls1.split("=").nth(1).unwrap().trim(), fls2.split("=").nth(1).unwrap().trim());
            let x = x.parse::<u32>().unwrap();
            let y = y.parse::<u32>().unwrap();
            // Now read run-length-encoding.
            let mut g = Vec::with_capacity(y as usize);
            for _ in 0..y {
                g.push(Vec::with_capacity(x as usize));
            }
            let mut finished_lines = 0;
            let mut pkc_sl = second_line.chars().peekable();
            let mut num_str = String::new();
            while let Some(ch) = pkc_sl.next() {
                use std::ops::IndexMut;
                match ch {
                    '0'...'9' => num_str.push(ch),
                    c @ 'b' | c @ 'o' => {
                        // Handle cell.
                        let mut num = 1;
                        if !num_str.is_empty() {
                            num = num_str.parse::<u32>().expect("Failed to parse number");
                        }
                        let c = match c {
                            'b' => cell::Cell::Dead,
                            'o' => cell::Cell::Alive,
                            _ => unreachable!(),
                        };
                        let mut v = g.index_mut(finished_lines as usize);
                        for _ in 0..num {
                            if v.len() >= (x as _) {
                                break;
                            }
                            v.push(c);
                        }
                        num_str.clear();
                    },
                    c @ '$' | c @ '!' => {
                        let mut v = g.index_mut(finished_lines as usize);
                        let to_fill = (x as usize) - v.len();
                        for _ in 0..to_fill {
                            v.push(cell::Cell::Dead);
                        }
                        num_str.clear();
                        match c {
                            '$' => finished_lines += 1,
                            '!' => break,
                            _ => unreachable!(),
                        }
                        continue;
                    },
                    _ => {},
                }
            }
            // Processed grid.
            // Add to larger grid.
            for (y, row) in g.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    grid.set(x as u32, y as u32, c.clone());
                    match c {
                        &cell::Cell::Alive => print!("o"),
                        &cell::Cell::Dead => print!("b"),
                    }
                }
                println!();
            }
        }

        let win_size = graphics::get_size(ctx);
        let s = Self {
            width: 800,
            height: 800,
            sqr_size: 8,
            curx: 0,
            cury: 0,
            grid,
            update_queue: Vec::new(),
            _mousing: false,
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
        let (x, y) = self.grid.grid_pos_for_mouse_pos(8, x, y);
        match button {
            MouseButton::Left => {
                self.curx = x;
                self.cury = y;
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
    event::run(ctx, state).unwrap();
}
