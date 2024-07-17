pub struct Path {
    pub steps: Vec<(usize, usize)>,
    pub total_cost: u32,
}

struct Node {
    row: usize,
    col: usize,
    value: u32,
    parent: Option<(usize, usize)>,
}
