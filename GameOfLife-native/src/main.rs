extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Color, Rect, Drawable};
use ggez::event::*;
mod grid;
mod chunk;

struct MainState {
    grid: grid::Grid,
    width: u32,
    height: u32,
    sqr_size: u32,
    curx: u32,
    cury: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        graphics::set_mode(ctx, conf::WindowMode {
            width: 640,
            height: 640,
            max_width: 0,
            max_height: 0,
            .. Default::default()
        });
        graphics::set_screen_coordinates(ctx, Rect::new(0f32, 0f32, 640f32, 640f32));

        let win_size = graphics::get_size(ctx);
        let s = Self {
            width: 640,
            height: 640,
            sqr_size: 8,
            curx: 0,
            cury: 0,
            grid: grid::Grid::new((win_size.0 / 8), (win_size.1 / 8)),
        };
        println!("Actual window size: {} x {}", win_size.0, win_size.1);
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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
                (x*(sqr_size-1)+sqr_size/2) as f32,
                (y*(sqr_size-1)+sqr_size/2) as f32,
                sqr_size as f32, sqr_size as f32
            ))?;
        }
        // Draw cursor.
        graphics::set_color(ctx, Color::new(0f32, 1f32, 0f32, 0.3));
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                (self.curx*(sqr_size-1)+sqr_size/2) as f32, (self.cury*(sqr_size-1)+sqr_size/2) as f32,
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
                if self.cury < self.grid.height() {
                    self.cury += 1;
                }
            },
            Keycode::Left => {
                if self.curx > 0 {
                    self.curx -= 1;
                }
            },
            Keycode::Right => {
                if self.curx < self.grid.width() {
                    self.curx += 1;
                }
            },
            Keycode::KpEnter | Keycode::Return | Keycode::Return2 => {
                self.grid.toggle(self.curx, self.cury);
            },
            Keycode::Tab => {
                self.step(ctx);
            }
            _ => {},
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, mut w: u32, mut h: u32) {
        if w % chunk::CHUNK_SIZE != 0 {
            w = w + chunk::CHUNK_SIZE - (w % chunk::CHUNK_SIZE);
        }
        if h % chunk::CHUNK_SIZE != 0 {
            h = h + chunk::CHUNK_SIZE - (h % chunk::CHUNK_SIZE);
        }

        graphics::set_resolution(ctx, w, h);
    }
}

impl MainState {
    fn step(&mut self, ctx: &mut Context) {
        // Split grid into chunks
        let (ww, wh) = graphics::get_size(ctx);
        let mut chunks = Vec::new();
        for _i in 0..wh / chunk::CHUNK_SIZE {
            let mut r = Vec::new();
            for __i in 0..ww / chunk::CHUNK_SIZE {
                r.push(chunk::Chunk::new());
            }
            chunks.push(r);
        }
        for (v, x, y) in self.grid.cells_with_pos() {
            let chunk_x = x / chunk::CHUNK_SIZE;
            let chunk_y = y / chunk::CHUNK_SIZE;
            chunks[chunk_y as usize][chunk_x as usize].add(v).unwrap();
        }
        for cr in chunks {
            for c in cr {
                println!("Got chunk: {:?}", c);
            }
        }
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
