use std::{cmp::Ordering, collections::HashSet};

use crate::{constants::N, exploration::Exploration, tree::Tree};

/// The storage of a pool: decides which explorations are kept when a new one is inserted.
pub trait ExplorationSet: Default {
    fn insert(&mut self, exp: Exploration);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> impl Iterator<Item = &Exploration>;
}

/// Keeps only non-dominated explorations (Pareto front).
impl ExplorationSet for Vec<Exploration> {
    fn insert(&mut self, exp: Exploration) {
        let mut i = 0;
        while i < self.len() {
            match self[i].partial_cmp(&exp) {
                // self[i] >= exp, don't add exp to the pool and stop here
                Some(Ordering::Greater | Ordering::Equal) => return,
                // exp > self[i], remove self[i] from the pool
                Some(Ordering::Less) => {
                    self.swap_remove(i);
                }
                // No domination, do nothing
                None => i += 1,
            }
        }
        self.push(exp);
    }

    fn len(&self) -> usize {
        <[Exploration]>::len(self)
    }

    fn iter(&self) -> impl Iterator<Item = &Exploration> {
        <[Exploration]>::iter(self)
    }
}

/// Removes only identical explorations, no domination check.
impl ExplorationSet for HashSet<Exploration> {
    fn insert(&mut self, exp: Exploration) {
        HashSet::insert(self, exp);
    }

    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn iter(&self) -> impl Iterator<Item = &Exploration> {
        HashSet::iter(self)
    }
}

pub struct GenericPool<S: ExplorationSet> {
    pub steps: u8,
    pub explorations: S,
}

/// Pool keeping only the non-dominated partial explorations.
pub type Pool = GenericPool<Vec<Exploration>>;

/// A simple pool that stores the explorations in a hashset and checks only for identity, not for domination.
/// Implemented for comparison with the domination.
pub type HashPool = GenericPool<HashSet<Exploration>>;

impl<S: ExplorationSet> GenericPool<S> {
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
        let mut pool = Self {
            steps: 2,
            explorations: S::default(),
        };
        for e in &snapshots {
            pool.explorations.insert(exp.clone().next_with_edges(e));
        }
        pool
    }

    fn add(&mut self, exp: Exploration) {
        if self.steps >= (N - 1) as u8 && exp.hops_to_explore() <= (2 * N - 3) as u8 {
            // exploration finished with no more than 2N - 3 hops
            return;
        }
        self.explorations.insert(exp);
    }

    pub fn next(&self, trees: &[Tree]) -> Self {
        let mut next = Self {
            steps: self.steps + 1,
            explorations: S::default(),
        };
        println!("*** Building the pool for step {} ***", next.steps);
        for (i, exp) in self.explorations.iter().enumerate() {
            if i % 1000 == 0 {
                println!(
                    "{i}/{} {}",
                    self.explorations.len(),
                    next.explorations.len()
                );
            }
            for tree in trees {
                next.add(exp.next(tree));
            }
        }
        next
    }
}

impl<S: ExplorationSet> Default for GenericPool<S> {
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
