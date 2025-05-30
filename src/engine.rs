use crate::rules::Rule;

/// Represents the state of a cell in the automaton.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

/// The main simulation structure for the cellular automaton.
///
/// It holds the current grid state, its dimensions, and the rule used
/// to evolve the grid from one generation to the next.
pub struct Automaton {
    pub width: usize,    // Number of columns in the grid
    pub height: usize,   // Number of rows in the grid
    pub grid: Vec<Cell>, // 1D flattened representation of the 2D grid
    pub rule: Rule,      // Birth/survival rules that define cell behavior
    pub generation: usize,
    pub live_cells: usize,
}

impl Automaton {
    /// Creates a new automaton grid of given dimensions with all cells initially dead.
    ///
    /// # Arguments
    /// * `width` - Number of columns
    /// * `height` - Number of rows
    /// * `rule` - The rule to use for cell transitions (e.g., B3/S23)
    ///
    /// # Returns
    /// An initialized `Automaton` with a dead grid and the given rule.
    pub fn new(width: usize, height: usize, rule: Rule) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::Dead; width * height],
            rule,
            generation: 0,
            live_cells: 0,
        }
    }

    /// Advances the simulation by one generation.
    ///
    /// Applies the rule to each cell based on its live neighbors and
    /// updates the grid accordingly. The update is synchronous — all
    /// cells are considered to evolve at the same time.
    pub fn tick(&mut self) {
        let mut new_grid = self.grid.clone();
        let mut next_live_count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let live_count = self.live_neighbor_count(x, y);
                let cell = self.grid[idx];

                new_grid[idx] = match cell {
                    Cell::Alive => {
                        if self.rule.survive.contains(&live_count) {
                            next_live_count += 1;
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                    Cell::Dead => {
                        if self.rule.birth.contains(&live_count) {
                            next_live_count += 1;
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                }
            }
        }

        self.grid = new_grid;
        self.generation += 1;
        self.live_cells = next_live_count;
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.grid[idx] = Cell::Alive;
        }
    }

    pub fn as_string(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.grid[self.index(x, y)];
                output.push(if cell == Cell::Alive { '█' } else { ' ' });
            }

            output.push('\n');
        }
        output
    }

    /// Converts 2D coordinates (x, y) to a 1D index in the grid vector.
    ///
    /// # Arguments
    /// * `x` - The column index (0-based)
    /// * `y` - The row index (0-based)
    ///
    /// # Returns
    /// The corresponding index in the 1D `grid` vector.
    ///
    /// # Panics
    /// May panic if `x >= width` or `y >= height`.
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Counts the number of live neighbors surrounding the cell at (x, y).
    ///
    /// # Arguments
    /// * `x` - Column index of the target cell
    /// * `y` - Row index of the target cell
    ///
    /// # Returns
    /// The number of adjacent cells that are `Cell::Alive`. Checks all
    /// eight neighboring positions in the Moore neighborhood.
    ///
    /// Out-of-bounds neighbors are ignored (i.e., no wraparound).
    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        let width = self.width as isize;
        let height = self.height as isize;

        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as isize + dx + width) % width;
                let ny = (y as isize + dy + height) % height;

                let idx = self.index(nx as usize, ny as usize);
                if self.grid[idx] == Cell::Alive {
                    count += 1;
                }
            }
        }

        count
    }
}
