# Karger Algorithm for Computing Minimum Cut
Computes the number of edges in the minimum cut of an undirected graph.  This is the number of edges which must be removed to create two disjoint, non-empty subgraphs.  At each iteration, algorithm chooses one edge randomly each iteration and merges its two nodes into a single node.  Any resulting self-loops are removed.  Process repeats until two nodes remain.  These two nodes have a significant chance of representing the disjoint subgraphs of the minimum cut.

## Usage
Graph definition is read from stdin following the file specification described [here](https://github.com/beaunus/stanford-algs/tree/master/testCases/course1/assignment4MinCut#1).  An example graph definition is provided in `kargerMinCut.txt`.  With Rust installed, the program can be executed with:
```
cat kargerMinCut.txt | cargo run
```
