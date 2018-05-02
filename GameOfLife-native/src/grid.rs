use cell::Cell;
use sim::Sim;

// use rand::{Rng, self};

#[derive(Default, Clone, PartialEq)]
pub struct Grid {
    rows: Vec<Vec<Cell>>,
    width: u32,
    height: u32
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut r = Vec::new();
        // let mut rng = rand::thread_rng();
        
        for i in 0..height {
            let mut c = Vec::new();
            for i in 0..width {
                // if rng.gen() {
                //     c.push(Cell::Alive);
                // } else {
                    c.push(Cell::Dead);
                // }
            }
            r.push(c);
        }

        Self { rows: r, width, height }
    }

    pub fn cells_with_pos(&self) -> Vec<(Cell, u32, u32)> {
        let mut res = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for col in self.rows.iter() {
            for v in col.iter() {
                res.push((v.clone(), x, y));
                x += 1;
            }
            x = 0;
            y += 1;
        }
        res
    }

    pub fn alive(&self) -> Vec<(u32, u32)> {
        self.cells_with_pos().iter().filter(|&x| {
            x.0 == Cell::Alive
        }).map(|e| { (e.1, e.2) }).collect()
    }

    pub fn set(&mut self, x: u32, y: u32, v: Cell) {
        let mut rows = &mut (self.rows);
        let mut row = &mut (rows[y as usize]);
        row[x as usize] = v.clone();
    }

    pub fn is_alive(&self, x: u32, y: u32) -> bool {
        self.rows[y as usize][x as usize] == Cell::Alive
    }
    
    pub fn toggle(&mut self, x: u32, y: u32) {
        let v = self.rows[y as usize][x as usize];
        self.set(x, y, match v {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        });
    }

    pub fn grid_pos_for_mouse_pos(&self, sqr_size: u32, x: i32, y: i32) -> (u32, u32) {
        println!("Got mouse x: {} y: {}", x, y);
        let mut xx = x;
        let mut yy = y;
        if x < 0 {
            xx = 0;
        }
        if y < 0 {
            yy = 0;
        }
        let clamped_x = (xx / sqr_size as i32);
        let clamped_y = (yy / sqr_size as i32);
        (clamped_x as u32, clamped_y as u32)
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn simulate(&mut self) {
        // Simulate using static methods from Sim.
        let new_grid = Sim::simulate(&self.rows);
        self.rows = new_grid;
    }

    pub fn resize(&mut self, dr: i32, dc: i32) {
        let new_grid_len = ((self.rows.len() as i32) + dr) as usize;
        let new_row_len = ((self.rows[0].len() as i32) + dc) as usize;
        self.rows.resize(new_grid_len, Vec::new());
        for row in self.rows.iter_mut() {
            if row.len() != new_row_len {
                row.resize(new_row_len, Cell::Dead);
            }
        }
        self.height = new_grid_len as u32;
        self.width = new_row_len as u32;
    }

    pub fn into_inner(self) -> Vec<Vec<Cell>> {
        self.rows
    }
}
