use crate::rules::Rule;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Automaton {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Cell>,
    pub rule: Rule,
}

impl Automaton {
    pub fn new(width: usize, height: usize, rule: Rule) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::Dead; width * height],
            rule,
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }

    fn index(&self, x: usize, y: usize) -> usize {
        todo!()
    }

    fn live_neighbor_count(&self, x: usize, y: usize) -> u8 {
        todo!()
    }
}
