use crate::{
    constants::{INF, N, S},
    tree::Tree,
};

#[derive(Clone)]
pub struct Exploration {
    /// Minimal number of hops for each configuration (current vertex, visited set).
    /// The visited set is a mask over vertices 1..N-1, vertex v coded by bit v-1
    /// (vertex 0 is always visited and has no bit).
    /// Slots 0..2S: agent in 0, slot = full mask.
    /// Slots (v+1)*S .. (v+2)*S for v >= 1: agent in v, slot = mask without the bit of v.
    pub hops: [u8; S * (N + 1)],
}

impl Exploration {
    pub fn new() -> Self {
        let mut exp = Self {
            hops: [INF; S * (N + 1)],
        };
        exp.hops[0] = 0;
        exp
    }

    fn transition_from_0(&self, next: &mut Self, v: usize) {
        let start_v = (v + 1) * S;
        for s in 0..(S << 1) {
            let cur = self.hops[s];
            if cur < INF {
                let s_v = delete_bit(s, v - 1);
                next.hops[start_v + s_v] = next.hops[start_v + s_v].min(cur + 1);
            }
        }
    }

    fn transition_to_0(&self, next: &mut Self, u: usize) {
        let start_u = (u + 1) * S;
        for s in 0..S {
            let cur = self.hops[start_u + s];
            if cur < INF {
                let s_0 = insert_one(s, u - 1);
                next.hops[s_0] = next.hops[s_0].min(cur + 1);
            }
        }
    }

    fn transition(&self, next: &mut Self, u: usize, v: usize) {
        let start_u = (u + 1) * S;
        let start_v = (v + 1) * S;
        for s in 0..S {
            let cur = self.hops[start_u + s];
            if cur < INF {
                let s_v = delete_bit(insert_one(s, u - 1), v - 1);
                next.hops[start_v + s_v] = next.hops[start_v + s_v].min(cur + 1);
            }
        }
    }

    /// Returns the next configuration after a time step with a given snapshot
    pub fn next(&self, snapshot: &Tree) -> Self {
        let mut next = self.clone();
        for (a, b) in snapshot.edges {
            let (a, b) = (a as usize, b as usize);
            for (u, v) in [(a, b), (b, a)] {
                if u == 0 {
                    self.transition_from_0(&mut next, v);
                } else if v == 0 {
                    self.transition_to_0(&mut next, u);
                } else {
                    self.transition(&mut next, u, v);
                }
            }
        }
        next
    }
}

/// Deletes the bit at the specified position. Keeps the less significant bits
/// and shifts the more significant bits one position to the right
fn delete_bit(number: usize, position: usize) -> usize {
    let low_mask = (1usize << position) - 1;
    (number & low_mask) | ((number >> 1) & !low_mask)
}

/// Inserts 1 at the specified position. Keeps the less significant bits
/// and shifts the most significant bits one position to the left
fn insert_one(number: usize, position: usize) -> usize {
    let low_mask = (1usize << position) - 1;
    (number & low_mask) | (1 << position) | ((number << 1) & !low_mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Slot of configuration (current vertex `v`, visited mask `m` over vertices 1..N-1).
    /// Written directly from the description of `hops`. Requires `v` to be in `m` if `v != 0`.
    fn index(v: usize, m: usize) -> usize {
        if v == 0 {
            m
        } else {
            (v + 1) * S + delete_bit(m, v - 1)
        }
    }

    /// Star centered at 0: edges (0, v) for v in 1..N
    fn star() -> Tree {
        Tree {
            edges: std::array::from_fn(|i| (0, (i + 1) as u8)),
        }
    }

    /// Path 0 - 1 - ... - N-1
    fn path() -> Tree {
        Tree {
            edges: std::array::from_fn(|i| (i as u8, (i + 1) as u8)),
        }
    }

    #[test]
    fn test_delete_bit() {
        assert_eq!(delete_bit(0b10110, 0), 0b1011);
        assert_eq!(delete_bit(0b10110, 2), 0b1010);
        assert_eq!(delete_bit(0b10110, 4), 0b0110);
        assert_eq!(delete_bit(0b10110, 5), 0b10110);
    }

    #[test]
    fn test_insert_one() {
        assert_eq!(insert_one(0b1011, 0), 0b10111);
        assert_eq!(insert_one(0b1010, 2), 0b10110);
        assert_eq!(insert_one(0b0110, 4), 0b10110);
        assert_eq!(insert_one(0b0110, 5), 0b100110);
    }

    #[test]
    fn test_delete_inverts_insert() {
        for x in 0..(1 << N) {
            for p in 0..N {
                let y = insert_one(x, p);
                assert_eq!((y >> p) & 1, 1);
                assert_eq!(delete_bit(y, p), x);
            }
        }
    }

    #[test]
    /// Each configuration has its own slot and all slots are used
    fn test_index_is_bijection() {
        let mut seen = [false; S * (N + 1)];
        for v in 0..N {
            for m in 0..(S << 1) {
                if v == 0 || m & (1 << (v - 1)) != 0 {
                    let i = index(v, m);
                    assert!(!seen[i], "slot {i} used twice");
                    seen[i] = true;
                }
            }
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn test_new() {
        let exp = Exploration::new();
        assert_eq!(exp.hops[index(0, 0)], 0);
        assert_eq!(exp.hops.iter().filter(|&&h| h < INF).count(), 1);
    }

    #[test]
    /// From the start, one step on a star reaches every neighbor of 0 in one hop
    fn test_one_step_star() {
        let exp = Exploration::new().next(&star());
        assert_eq!(exp.hops[index(0, 0)], 0);
        for v in 1..N {
            assert_eq!(exp.hops[index(v, 1 << (v - 1))], 1);
        }
        assert_eq!(exp.hops.iter().filter(|&&h| h < INF).count(), N);
    }

    #[test]
    /// Going out and back on a star: in 0 again with one visited vertex, after two hops
    fn test_back_to_0() {
        let exp = Exploration::new().next(&star()).next(&star());
        for v in 1..N {
            assert_eq!(exp.hops[index(0, 1 << (v - 1))], 2);
        }
    }

    #[test]
    /// Walking along a path visits all vertices in N - 1 hops
    fn test_path() {
        let mut exp = Exploration::new();
        for _ in 0..N - 1 {
            exp = exp.next(&path());
        }
        let all = (S << 1) - 1;
        assert_eq!(exp.hops[index(N - 1, all)], (N - 1) as u8);
    }
}
