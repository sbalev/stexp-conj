#![feature(test)]

extern crate test;

mod benches {
    use stexp_conj::{exploration::*, tree::*, constants::*};
    use test::Bencher;

    #[bench]
    fn bench_tree_generation(b: &mut Bencher) {
        b.iter(|| Tree::all_trees())
    }

    #[bench]
    fn bench_star_exploration(b: & mut Bencher) {
        let star = Tree {
            edges: std::array::from_fn(|i| (0, (i + 1) as u8)),
            id: 0,
        };
        b.iter(|| {
            let mut exp = Exploration::new();
            for _ in 0..2 * N - 3 {
                exp = exp.next(&star)
            }
        });
    }

    #[bench]
    fn bench_path_exploration(b: & mut Bencher) {
        let path = Tree {
            edges: std::array::from_fn(|i| (i as u8, (i + 1) as u8)),
            id: 0,
        };
        b.iter(|| {
            let mut exp = Exploration::new();
            for _ in 0..2 * N - 3 {
                exp = exp.next(&path)
            }
        });
    }
}
