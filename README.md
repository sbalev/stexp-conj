# STEXP conjecture

STEXP stands for *Shortest Temporal EXploration Problem*. The goal of this project is to check the following conjecture, stated in [this paper](https://doi.org/10.1016/j.tcs.2026.116190), for small values of $n$:

> Each constantly connected temporal graph with lifetime $\frac{n(n - 1)}{2}$ can be explored with no more than $2n - 3$ edge traversals.

Without loss of generality we suppose that the underlying graph is $K_n$ and that each snapshot is a tree.

An agent starts at vertex $0$. At each time step it can either wait or traverse one edge of the current snapshot. The exploration is complete when every vertex has been visited; the agent does not need to return to its starting vertex.

For a given temporal graph (sequence of snapshots), we use dynamic programming to compute its shortest exploration. We maintain a table $H(v, S)$ where $v$ is the current vertex and $S$ is the set of vertices visited so far. This table is updated at each time step.

It is impossible to check all the $n^{\frac{(n - 2)(n - 1)n}{2}}$ temporal graphs in reasonable time, so we hope to reduce the search space using the notion of *domination*. A partial exploration $H$ dominates $H'$ if $H(v, S) \ge H'(v, S)$ for each $v$ and $S$. If we detect dominated exploration, it can be eliminated. Indeed, $H'$ can never finish with more hops than $H$.

The idea is to maintain a pool of non-dominated partial explorations (Pareto front) for each time step. The challenge is to detect and eliminate the dominated explorations quickly. Moreover, domination up to a permutation of the vertices $1, \dots, n-1$ can also be used: the starting vertex $0$ plays a special role, but the other vertices are interchangeable.

## Usage

- The number of vertices $n$ is the constant `N` in `src/constants.rs`. Change it and rebuild.
- `cargo run --release` runs the random testing.
- `cargo test` runs the unit tests.
- `cargo bench` runs the benchmarks and requires a nightly toolchain.

## Status

- **Random testing** (`experiments.rs`): random temporal graphs are generated and explored with the DP. No counterexample has been found so far.
- **Exhaustive search** (`pool.rs`): work in progress. The pool of partial explorations is built step by step, but the domination check is not implemented yet. If the pool is not empty after time step $\frac{n(n-1)}{2}$, a counterexample has been found. Otherwise, the conjecture holds for this value of $n$.
