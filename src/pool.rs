use std::cmp::Ordering;

use crate::{constants::N, exploration::Exploration, tree::Tree};

pub struct Pool {
    pub steps: u8,
    pub explorations: Vec<Exploration>,
}

impl Pool {
    /// Creates the pool containing only the non-dominated partial explorations after 2 steps
    pub fn new() -> Self {
        // At the first step we only use the edge (0, 1)
        let exp = Exploration::new().next_with_edges(&[(0, 1)]);
        // At the second step, the non-dominated partial explorations are given by :
        // 2         2     2     2  3
        // |         |    / \    |  |
        // 0--1   0--1   0   1   O  1
        let snapshots = [
            [(0, 1), (0, 2)],
            [(0, 1), (1, 2)],
            [(0, 2), (1, 2)],
            [(0, 2), (1, 3)],
        ];
        Self {
            steps: 2,
            explorations: snapshots
                .iter()
                .map(|e| exp.clone().next_with_edges(e))
                .collect(),
        }
    }

    fn add(&mut self, exp: &Exploration) {
        if self.steps >= (N - 1) as u8 && exp.hops_to_explore() <= (2 * N - 3) as u8 {
            // exploration finished with no more than 2N - 3 hops
            return;
        }

        let mut i = 0;
        while i < self.explorations.len() {
            match self.explorations[i].partial_cmp(exp) {
                // explorations[i] >= exp, don't add exp to the pool and stop here
                Some(Ordering::Greater | Ordering::Equal) => return,
                // exp > explorations[i], remove explorations[i] from the pool
                Some(Ordering::Less) => {
                    self.explorations.swap_remove(i);
                }
                // No domination, do nothing
                None => i += 1,
            }
        }
        self.explorations.push(exp.clone());
    }

    pub fn next(&self, trees: &[Tree]) -> Self {
        let mut next = Self {
            steps: self.steps + 1,
            explorations: Vec::new(),
        };
        for exp in &self.explorations {
            for tree in trees {
                next.add(&exp.next(tree));
            }
        }
        next
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_domination() {
        let trees = Tree::all_trees();
        let mut pool = Pool::new();
        for _ in 2..4 {
            // The pull must contain only non-dominated elements
            for i in 0..pool.explorations.len() {
                for j in i + 1..pool.explorations.len() {
                    assert_eq!(
                        pool.explorations[i].partial_cmp(&pool.explorations[j]),
                        None
                    );
                    assert_eq!(
                        pool.explorations[j].partial_cmp(&pool.explorations[i]),
                        None
                    );
                }
            }
            pool = pool.next(&trees);
        }
    }
}
