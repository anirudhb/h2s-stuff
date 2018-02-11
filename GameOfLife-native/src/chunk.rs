// A 16x16 chunk.
#[derive(Debug, Clone)]
pub struct Chunk {
    chunk: Vec<Vec<u32>>,
}

pub const CHUNK_SIZE: u32 = 64;

impl Chunk {
    pub fn new() -> Self {
        Self { chunk: Vec::new() }
    }

    pub fn add(&mut self, val: u32) -> Result<(), &'static str> {
        if let Some(last_row) = self.chunk.last_mut() {
            if last_row.len() < CHUNK_SIZE as _ {
                last_row.push(val);
                // Last row has space for the element.
                return Ok(())
            }
        }
        // There is no last row, or the last row is full.
        if self.chunk.len() < CHUNK_SIZE as _ {
            let mut new_row = Vec::new();
            new_row.push(val);
            self.chunk.push(new_row);
            return Ok(())
        } else {
            // The entire chunk is full.
            return Err("The chunk is full. Consider adding to another chunk.")
        }
    }
}    
