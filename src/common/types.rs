use std::collections::VecDeque;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct Path {
    pub steps: VecDeque<(usize, usize)>,
    pub total_cost: u32,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path: ")?;
        for (x, y) in &self.steps {
            write!(f, "({},{}) ", x, y)?;
        }
        write!(f, "| Cost: {}", self.total_cost)
    }
}

#[derive(Clone)]
pub struct Node {
    pub row: usize,
    pub col: usize,
    pub value: u32,
    pub parent: Option<(usize, usize)>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Eq for Node {}

#[derive(Clone)]
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
