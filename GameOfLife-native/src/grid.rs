pub struct Grid<T: Default + Clone> {
    rows: Vec<Vec<T>>,
    pub dirty: Vec<(T, u32, u32)>,
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

        Self { rows: r, width, height, dirty: Vec::new() }
    }

    pub fn cells_with_pos(&self) -> Vec<(T, u32, u32)> {
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

    pub fn set(&mut self, x: u32, y: u32, v: T) {
        let mut rows = &mut (self.rows);
        let mut row = &mut (rows[y as usize]);
        row[x as usize] = v.clone();
        // Add to the dirty buffer.
        self.dirty.push((v.clone(), x, y));
    }

    pub fn scrap_dirty(&mut self) {
        // Assign dirty to new vector, throwing away old contents.
        self.dirty = Vec::new();
    }
}
