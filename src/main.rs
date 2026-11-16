use rand::RngExt;
use stexp_conj::{constants::N, exploration::Exploration, tree::Tree};

fn main() {
    explore_random_graphs(100_000_000);
}

fn explore_random_graphs(graph_count: u64) {
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
        for step in 1..N * (N - 1) / 2 + 1 {
            let t = rng.random_range(0..trees.len());
            snapshots.push(trees[t].id);
            exp = exp.next(&trees[t]);
            if step >= N - 1 {
                let hops = exp.hops_to_explore();
                if hops <= (2 * N - 3) as u8 {
                    stats[step][hops as usize] += 1;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("Counterexample {snapshots:?}");
            panic!("The conjecture is wrong!!!");
        }
        if i % 100_000 == 0 {
            println!("{i} graphs tested");
        }
    }
    for step in N - 1..MAX_STEPS + 1 {
        println!("{step} steps : {:?}", &stats[step][N - 1..]);
    }
}
