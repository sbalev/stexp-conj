use std::cmp::{max, min};

use crate::constants::N;

const TREE_COUNT: usize = N.pow(N as u32 - 2); // Cayley's formula

/// A tree is just a list of `N - 1` edges
pub struct Tree {
    pub edges: [(u8, u8); N - 1],
    pub id: usize,
}

impl Tree {
    /// Converts an id in 0..TREE_COUNT into a tree.
    /// The id (in base N) is interpreted as the Prüfer sequence of the tree.
    /// See https://en.wikipedia.org/wiki/Pr%C3%BCfer_sequence
    pub fn from_id(id: usize) -> Self {
        let id = id % TREE_COUNT; // make sure that the id is in 0..TREE_COUNT
        let mut tree = Self {
            edges: [(0, 0); N - 1],
            id,
        };

        let mut degrees = [1u8; N];
        let mut prufer = id;
        for _ in 0..N - 2 {
            degrees[prufer % N] += 1;
            prufer /= N;
        }

        let mut j = 0;
        prufer = id;
        for _ in 0..N - 2 {
            let u = prufer % N;
            prufer /= N;
            // find the first leaf
            let v = degrees.iter().position(|&d| d == 1).unwrap();
            // add the edge (u, v) to the tree and decrement the degrees of u and v
            tree.edges[j] = (min(u as u8, v as u8), max(u as u8, v as u8));
            j += 1;
            degrees[u] -= 1;
            degrees[v] -= 1;
        }
        // At the end there are two vertices with degree 1. Connect them with edge.
        let u = degrees.iter().position(|&d| d == 1).unwrap();
        let v = u + 1 + degrees[u + 1..].iter().position(|&d| d == 1).unwrap();
        tree.edges[j] = (u as u8, v as u8);

        tree
    }

    /// Generates all labeled trees of N vertices
    pub fn all_trees() -> Vec<Self> {
        let mut trees = Vec::with_capacity(TREE_COUNT);
        // Each Prüfer sequence gives a tree
        for id in 0..TREE_COUNT {
            trees.push(Self::from_id(id));
        }
        trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Tree {
        // Very inefficient way to check if a graph is connected,
        // but it is only for the tests, so no need to build adjacency lists
        fn is_connected(&self) -> bool {
            let mut visited = [false; N];
            visited[0] = true;
            let mut visited_count = 1;
            let mut new_visit = true;
            while new_visit {
                new_visit = false;
                for (u, v) in self.edges {
                    if visited[u as usize] != visited[v as usize] {
                        visited[u as usize] = true;
                        visited[v as usize] = true;
                        new_visit = true;
                        visited_count += 1;
                        if visited_count == N {
                            return true;
                        }
                    }
                }
            }
            false
        }
    }

    #[test]
    // Don't run this for N > 7, it takes too much time
    fn test_trees() {
        let mut all = Tree::all_trees();
        assert_eq!(all.len(), TREE_COUNT);

        for tree in &all {
            assert!(tree.is_connected())
        }

        // sort the edges so that we can compare the trees
        for t in &mut all {
            t.edges.sort_unstable();
        }

        for i in 0..all.len() {
            for j in i + 1..all.len() {
                assert_ne!(all[i].edges, all[j].edges)
            }
        }
    }
}
