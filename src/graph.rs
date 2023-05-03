use std::collections::{HashMap, VecDeque};
type Vertex = String;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<usize>>;


#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub outedges: AdjacencyLists,
    pub vertex_labels: Vec<Vertex>,
    pub vertex_indices: HashMap<Vertex, usize>,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            let u_index = self.vertex_indices[u.as_str()];
            let v_index = self.vertex_indices[v.as_str()];
            self.outedges[u_index].push(v_index);
        }
    }

    pub fn create_directed(edges: &ListOfEdges) -> Graph {
        let mut vertex_labels: Vec<Vertex> = edges.iter().flat_map(|(u, v)| vec![u.clone(), v.clone()]).collect::<Vec<_>>();
        vertex_labels.sort();
        vertex_labels.dedup();

        let n = vertex_labels.len();

        let mut vertex_indices = HashMap::new();
        for (i, vertex) in vertex_labels.iter().enumerate() {
            vertex_indices.insert(vertex.clone(), i);
        }

        let mut outedges: AdjacencyLists = vec![vec![]; n];
        let mut g = Graph {
            n,
            outedges,
            vertex_labels,
            vertex_indices,
        };

        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    pub fn create_undirected(edges: &ListOfEdges) -> Graph {
        let mut directed_edges: ListOfEdges = Vec::with_capacity(edges.len() * 2);
        for (u, v) in edges {
            directed_edges.push((u.clone(), v.clone()));
            directed_edges.push((v.clone(), u.clone()));
        }
        let mut g = Graph::create_directed(&directed_edges);
        g.sort_graph_lists();
        g
    }
    pub fn compute_and_print_centralities(graph: &Graph) {
        let n = graph.n;
    
        // Initialize a vector to store the centrality of each vertex.
        let mut centrality: Vec<f64> = vec![0.0; n];
    
        // Compute the centrality of each vertex.
        for s in 0..n {
            let mut distance: Vec<Option<u32>> = vec![None; n];
            let mut num_paths: Vec<u32> = vec![0; n];
            let mut queue: VecDeque<Vertex> = VecDeque::new();
    
            distance[s] = Some(0);
            num_paths[s] = 1;
            queue.push_back(graph.vertex_labels[s].clone());
    
            while let Some(v) = queue.pop_front() {
                let v_index = graph.vertex_indices[&v];
                for &u_index in graph.outedges[v_index].iter() {
                    let u = &graph.vertex_labels[u_index];
                    let u_index = graph.vertex_indices[u];
                    if distance[u_index] == None {
                        distance[u_index] = Some(distance[v_index].unwrap() + 1);
                        num_paths[u_index] += num_paths[v_index];
                        queue.push_back(u.clone());
                    } else if distance[u_index].unwrap() == distance[v_index].unwrap() + 1 {
                        num_paths[u_index] += num_paths[v_index];
                    }
                }
            }
    
            // Compute the centrality of vertex s.
            let mut c = 0.0;
            for t in 0..n {
                if s != t {
                    let t_index = graph.vertex_indices[&graph.vertex_labels[t]];
                    if distance[t_index] == Some(distance[s].unwrap() + 1) {
                        c += num_paths[t_index] as f64 / num_paths[s] as f64;
                    }
                }
            }
    
            centrality[s] = c;
            println!("{}: {}", graph.vertex_labels[s], centrality[s]);
        }
        
        // Find and print the top 3 centralities
        let mut centrality_with_vertex: Vec<(f64, &str)> = centrality.iter().zip(graph.vertex_labels.iter()).map(|(c, v)| (*c, v.as_str())).collect();
        centrality_with_vertex.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("Top 3 centralities:");
        for i in 0..3 {
            let (c, v) = &centrality_with_vertex[i];
            println!("{}: {}", v, c);
        }
    }

}
