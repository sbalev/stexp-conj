use stexp_conj::tree::Tree;

fn main() {
    let trees = Tree::all_trees();
    println!("{0}", trees.len());
}
