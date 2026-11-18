use stexp_conj::{pool::Pool, tree::Tree};

fn main() {
    // stexp_conj::experiments::explore_random_graphs(1_000_000);
    let trees = Tree::all_trees();
    let mut pool = Pool::new();
    pool = pool.next(&trees);
    pool = pool.next(&trees);
    println!("{}", pool.explorations.len());
}
