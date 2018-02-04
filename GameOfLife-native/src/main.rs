#![feature(conservative_impl_trait)]

extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Color, Rect};
mod grid;

struct MainState {
    grid: grid::Grid<u32>,
    cur_x: u32,
    cur_y: u32,
    width: u32,
    height: u32,
    sqr_size: u32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        let s = Self {
            cur_x: 0,
            cur_y: 0,
            width: 100,
            height: 100,
            sqr_size: 10,
            grid: grid::Grid::new(100, 100)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.grid.set(self.cur_x, self.cur_y, 0);
        self.cur_x += 1;
        if self.cur_x+1 == self.width {
            self.cur_y += 1;
            if self.cur_y+1 == self.height {
                self.cur_x = 0;
                self.cur_y = 0;
            }
            self.cur_x = 0;
        }
        self.grid.set(self.cur_x, self.cur_y, 1);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let sqr_size = self.sqr_size;
        // Draw each square
        for (cell, x, y) in self.grid.cells_with_pos() {
            if *cell == 1 {
                graphics::set_color(ctx, Color::new(0f32,1f32,0f32,1f32));
            } else {
                graphics::set_color(ctx, Color::new(0f32,0f32,0f32,1f32));
            }
            graphics::rectangle(ctx, DrawMode::Fill, Rect::new(
                (x*(sqr_size-1)+sqr_size/2) as f32,
                (y*(sqr_size-1)+sqr_size/2) as f32,
                sqr_size as f32, sqr_size as f32
            ));
        }
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
