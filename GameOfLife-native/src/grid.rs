pub struct Grid<T: Default + Clone> {
    rows: Vec<Vec<T>>,
    width: u32,
    height: u32
}

impl <T: Default + Clone> Grid<T> {
    pub fn new(width: u32, height: u32) -> Self {
        let mut r = Vec::new();
        
        for i in 0..height {
            let mut c = Vec::new();
            for i in 0..width {
                c.push(T::default());
            }
            r.push(c);
        }

        Self { rows: r, width, height }
    }

    pub fn cells_with_pos<'a>(&'a self) -> impl Iterator<Item=(&T, u32, u32)> + 'a {
        self.rows.clone().iter().flat_map(move |col| {
            let mut y = 0;
            col.iter().map(move |v| {
                let mut x = 0;
                y += 1;
                x += 1;
                (v, x-1 as u32, y-1 as u32)
            })
        })
    }

    pub fn set(&mut self, x: u32, y: u32, v: T) {
        (&mut self.rows)[y as usize][x as usize] = v;
    }
}
