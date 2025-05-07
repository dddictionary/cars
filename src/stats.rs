#[derive(Debug)]
pub struct Stats {
    pub generation: usize,
    pub live_cells: usize,
    pub entropy: f64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            generation: 0,
            live_cells: 0,
            entropy: 0.0,
        }
    }

    pub fn compute(generation: usize, live_cells: usize, total_cells: usize) -> Self {
        let entropy = compute_entropy(live_cells, total_cells);

        Self {
            generation,
            live_cells,
            entropy,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

pub fn compute_entropy(live: usize, total: usize) -> f64 {
    if live == 0 || live == total {
        0.0
    } else {
        let p_alive = live as f64 / total as f64;
        let p_dead = 1.0 - p_alive;
        -p_alive * p_alive.log2() - p_dead * p_dead.log2()
    }
}
