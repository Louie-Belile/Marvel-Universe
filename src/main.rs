#[cfg(test)]

mod tests;
mod read_file;
use crate::read_file::read_files;
mod graph;
use graph::Graph;



use std::fs::File;
use std::io::{BufReader};
use csv::{ReaderBuilder, Trim};

type Vertex = String;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<usize>>;

      

fn main() {

    let edges = read_files("hero-network.csv");


    // Construct an undirected graph from the edges
    let graph = Graph::create_undirected(&edges);

    // Compute the betweenness centrality scores of each vertex in the graph
    Graph::compute_and_print_centralities(&graph);

}


    



















