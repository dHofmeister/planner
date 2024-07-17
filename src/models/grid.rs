use std::fmt;

pub struct Grid {
    pub data: Vec<u8>,
    pub size: usize,
}

impl Grid {
    pub fn new(data: Vec<u8>) -> Self {
        let size = (data.len() as f64).sqrt() as usize;
        Grid { data, size }
    }

    pub fn value_at(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.size + col]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self.value_at(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
