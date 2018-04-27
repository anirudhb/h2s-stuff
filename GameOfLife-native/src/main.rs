extern crate ggez;
extern crate rayon;
use ggez::*;
use ggez::graphics::{DrawMode, Color, Rect, Drawable};
use ggez::event::*;
mod grid;
mod cell;
mod sim;

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
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, 800f32, 800f32));

        let win_size = graphics::get_size(ctx);
        let s = Self {
            width: 800,
            height: 800,
            sqr_size: 8,
            curx: 0,
            cury: 0,
            grid: grid::Grid::new(100, 100),
            update_queue: Vec::new(),
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
        graphics::set_color(ctx, Color::new(0f32, 1f32, 0f32, 0.3));
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                (self.curx*sqr_size) as f32, (self.cury*sqr_size) as f32,
                sqr_size as f32, sqr_size as f32
        ))?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::Up => {
                if self.cury > 0 {
                    self.cury -= 1;
                }
            },
            Keycode::Down => {
                if self.cury < self.grid.height()-1 {
                    self.cury += 1;
                }
            },
            Keycode::Left => {
                if self.curx > 0 {
                    self.curx -= 1;
                }
            },
            Keycode::Right => {
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
            }
            _ => {},
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
            samples: conf::NumSamples::One,
        },
        backend: conf::Backend::default(),
        window_mode: conf::WindowMode {
            width: 800,
            height: 800,
            .. Default::default()
        }
    };
    let ctx = &mut Context::load_from_conf("game_of_life", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
