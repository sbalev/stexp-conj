use rand::RngExt;

use crate::{constants::N, exploration::Exploration, tree::Tree};

pub fn explore_random_graphs(graph_count: u64) {
    print!("Generating trees ... ");
    let trees = Tree::all_trees();
    println!("done");

    let mut rng = rand::rng();
    const MAX_STEPS: usize = N * (N - 1) / 2;
    let mut snapshots = Vec::with_capacity(MAX_STEPS);
    let mut stats = [[0; 2 * N - 2]; MAX_STEPS + 1];
    println!("Testing {graph_count} random graphs ...");
    for i in 0..graph_count {
        snapshots.clear();
        let mut exp = Exploration::new();
        let mut found = false;
        for (step, stats_line) in stats.iter_mut().enumerate().skip(1) {
            let t = rng.random_range(0..trees.len());
            snapshots.push(trees[t].id);
            exp = exp.next(&trees[t]);
            if step >= N - 1 {
                let hops = exp.hops_to_explore();
                if hops <= (2 * N - 3) as u8 {
                    stats_line[hops as usize] += 1;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("Counterexample {snapshots:?}");
            panic!("The Conjecture is wrong!!!");
        }
        if i % 100_000 == 0 {
            println!("{i} graphs tested");
        }
    }
    for (step, stat_line) in stats.iter().enumerate().skip(N - 1) {
        println!("{step} steps : {:?}", &stat_line[N - 1..]);
    }
}
