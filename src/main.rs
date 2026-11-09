use stexp_conj::tree::Tree;

fn main() {
    let trees = Tree::all_trees();
    println!("{0}", trees.len());
    for t in trees {
        println!("{0:?}", t.edges);
    }
}
