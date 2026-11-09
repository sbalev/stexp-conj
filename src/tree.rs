use std::cmp::{max, min};

use crate::constants::N;

/// A tree is just a list of `N - 1` edges
pub struct Tree {
    pub edges: [(u8, u8); N - 1],
}

impl Tree {
    /// Converts a Prüfer sequence to a tree
    /// 
    /// See https://en.wikipedia.org/wiki/Pr%C3%BCfer_sequence
    /// 
    /// The sequence contains N - 2 numbers, each between 0 and N - 1
    fn from_prufer(prufer: &[u8]) -> Self {
        // The degree of each vertex is the number of times it appears in the sequence plus one
        let mut degrees = [1u8; N];
        for v in prufer {
            degrees[*v as usize] += 1;
        }

        let mut tree = Self {
            edges: [(0, 0); N - 1],
        };

        let mut j = 0;
        for u in prufer {
            // find the first leaf
            let v = degrees.iter().position(|&d| d == 1).unwrap();
            // add the edge (u, v) to the tree and decrement the degrees of u and v
            tree.edges[j] = (min(*u, v as u8), max(*u, v as u8));
            j += 1;
            degrees[*u as usize] -= 1;
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
        let tree_count = N.pow(N as u32 - 2); // Cayley's formula
        let mut trees = Vec::with_capacity(tree_count);
        // Each Prüfer sequence gives a tree
        let prufer = &mut [0; N - 2];
        for _ in 0..tree_count {
            trees.push(Self::from_prufer(prufer));
            next_prufer(prufer);
        }
        trees
    }
}

/// Gives the next Prüfer sequence
/// 
/// If we consider the sequence as the digits of a number in base `N`, this function just increments the number
fn next_prufer(prufer: &mut [u8]) {
    let n_max = N as u8 - 1;
    for p in prufer {
        if *p < n_max {
            *p += 1;
            return;
        } else {
            *p = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Tree {
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
        assert_eq!(all.len(), N.pow(N as u32 - 2));

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
