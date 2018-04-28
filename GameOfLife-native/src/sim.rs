/* Handles simulating the actual game of life. */
use rayon::prelude::*;
use cell::Cell;

pub struct Sim;

impl Sim {
    pub fn simulate(g: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        let rown = g.len() as u32;
        let coln = g[0].len() as u32;
        (0..rown).into_par_iter().map(|y| {
            (0..coln).into_par_iter().map(|x| {
                let c = g[y as usize][x as usize].clone();
                // Get number of alive neighbors
                let alive_neighbors = Self::alive_neighbors(g, x, y);
                // Do stuff based on that
                if alive_neighbors < 2 && c == Cell::Alive {
                    Cell::Dead
                } else if (alive_neighbors == 2 || alive_neighbors == 3) && c == Cell::Alive {
                    Cell::Alive
                } else if alive_neighbors > 3 && c == Cell::Alive {
                    Cell::Dead
                } else if alive_neighbors == 3 && c == Cell::Dead {
                    Cell::Alive
                } else {
                    c
                }
            }).collect::<Vec<Cell>>()
        }).collect::<Vec<Vec<Cell>>>()
    }

    fn alive_neighbors(g: &Vec<Vec<Cell>>, x: u32, y: u32) -> u32 {
        let x = x as i32;
        let y = y as i32;
        let ltn = Self::cell_to_u32(Self::gf_index(g, x-1, y-1));
        let ctn = Self::cell_to_u32(Self::gf_index(g, x, y-1));
        let rtn = Self::cell_to_u32(Self::gf_index(g, x+1, y-1));
        let ln  = Self::cell_to_u32(Self::gf_index(g, x-1, y));
        let rn  = Self::cell_to_u32(Self::gf_index(g, x+1, y));
        let lbn = Self::cell_to_u32(Self::gf_index(g, x-1, y+1));
        let cbn = Self::cell_to_u32(Self::gf_index(g, x, y+1));
        let rbn = Self::cell_to_u32(Self::gf_index(g, x+1, y+1));
        ltn+ctn+rtn+ln+rn+lbn+cbn+rbn
    }

    fn gf_index(g: &Vec<Vec<Cell>>, x: i32, y: i32) -> Cell {
        *Self::flexible_index(&(Self::flexible_index(g, y)), x)
    }

    fn cell_to_u32(c: Cell) -> u32 {
        match c == Cell::Alive {
            true => 1,
            false => 0,
        }
    }

    fn flexible_index<T>(v: &Vec<T>, i: i32) -> &T {
        if i < 0 || i > (v.len() - 1) as i32 {
            // Should wraparound
            // Use MODULO
            let a = i;
            let b = v.len() as i32;
            &v[(((a % b) + b) % b) as usize]
        } else {
            &v[i as usize]
        }
    }
}
