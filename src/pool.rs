use crate::{constants::N, exploration::Exploration, tree::Tree};

pub struct Pool {
    steps: u8,
    explorations: Vec<Exploration>,
}

impl Pool {
    /// Creates the pool containing only the non-dominated partial explorations after 2 steps
    pub fn new() -> Self {
        // At the first step we only use the edge (0, 1)
        let exp = Exploration::new().next_with_edges(&[(0, 1)]);
        //let mut explorations = Vec::new();
        // At the second step, the non-dominated partial explorations are given by :
        // 2         2     2    2  3
        // |         |    /\    |  |
        // 0--1   0--1   0  1   O  1
        let snapshots = [
            [(0, 1), (0, 2)],
            [(0, 1), (1, 2)],
            [(0, 2), (1, 2)],
            [(0, 2), (1, 3)],
        ];
        Self {
            steps: 2,
            explorations : snapshots.iter().map(|e| exp.clone().next_with_edges(e)).collect(),
        }
    }

    pub fn add(&mut self, exp : &Exploration) {
        if self.steps >= (N - 1) as u8 && exp.hops_to_explore() <= (2 * N - 3) as u8 {
            // exploration finished in no more than 2N - 3 hops
            return;
        }
        // TODO check if exp dominates or is dominated by some elements of the pool
        self.explorations.push(exp.clone());
    }

    pub fn next(&self, trees: &[Tree]) -> Self {
        let mut next = Self { steps: self.steps + 1, explorations: Vec::new() };
        for exp in &self.explorations {
            for tree in trees {
                next.add(&exp.next(tree));
            }
        }
        next
    }
}
