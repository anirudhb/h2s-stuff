/* RLE loading for Game Of Life. */

use std::{
    io::{
        self,
        prelude::*,
    },
    fs::File,
    collections::HashMap,
    path::Path
};

use cell;
use grid::Grid;

pub struct RLE;

impl RLE {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Grid> {
        // Load file.
        let mut file = File::open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let mut lines = s.lines();
        // Skip lines with #
        let mut lines = lines.filter(|l| {
            !l.starts_with("#")
        });
        // First line is x, y
        let first_line = lines.next().unwrap();
        // Second "line" is the rest of the lines put together.
        let second_line = lines.map(|x| x.trim()).collect::<String>();
        // Try to convert first line into hashmap
        let info_hashmap = Self::convert_to_hashmap(first_line.to_string());
        // Get x (width), y (height)
        let width = info_hashmap.get("x").expect("Could not find x");
        let height = info_hashmap.get("y").expect("Could not find y");
        // Process using hand-written char-by-char algorithm.
        let width = width.parse::<u32>().unwrap();
        let height = height.parse::<u32>().unwrap();
        // Note that we do not load into an actual Grid.
        let mut loaded = Vec::with_capacity(height as usize);
        for _ in 0..height {
            loaded.push(Vec::with_capacity(width as usize));
        }
        let mut finished_lines = 0;
        let mut num_str = String::new();
        let mut second_line = second_line.chars();
        while let Some(ch) = second_line.next() {
            use std::ops::IndexMut;
            match ch {
                '0'...'9' => num_str.push(ch),
                c @ 'b' | c @ 'o' => {
                    // Got cell.
                    let mut num = 1;
                    if !num_str.is_empty() {
                        num = num_str.parse::<u32>().unwrap();
                    }
                    let c = match c {
                        'b' => cell::Cell::Dead,
                        'o' => cell::Cell::Alive,
                        _ => unreachable!(),
                    };
                    let mut row = loaded.index_mut(finished_lines as usize);
                    for _ in 0..num {
                        // Prevent row from becoming oversized.
                        if row.len() >= (width as _) {
                            break;
                        }
                        row.push(c);
                    }
                    num_str.clear();
                },
                c @ '$' | c @ '!' => {
                    let mut row = loaded.index_mut(finished_lines as usize);
                    let left = (width as usize) - row.len();
                    for _ in 0..left {
                        row.push(cell::Cell::Dead);
                    }
                    num_str.clear();
                    match c {
                        '$' => finished_lines += 1,
                        '!' => break,
                        _ => unreachable!(),
                    }
                    continue;
                },
                _ => panic!("Invalid RLE!"),
            }
        }
        // Convert loaded vec into grid.
        let mut grid = Grid::new(width, height);
        for (y, row) in loaded.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                // Optimize because grid is dead by default.
                if c == cell::Cell::Alive {
                    grid.set(x as u32, y as u32, cell::Cell::Alive);
                }
            }
        }
        Ok(grid)
    }

    fn convert_to_hashmap(s: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let pairs = s.split(",").map(|x| x.trim().to_owned()).collect::<Vec<String>>();
        let vals = pairs.iter().map(|x| x.split("=").map(|xx| xx.trim().to_owned()).collect::<Vec<String>>());
        let vals = vals.collect::<Vec<Vec<String>>>();
        for v in vals.iter() {
            map.insert(v[0].to_string(), v[1].to_string());
        }
        map
    }
}
