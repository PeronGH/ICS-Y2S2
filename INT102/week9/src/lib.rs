use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt::Debug;

pub fn travelling_salesman<N: Clone + Debug>(
    graph: &UnGraph<N, f64>,
    start_node: NodeIndex,
) -> TspResult {
    let adj_matrix = graph_to_adj_matrix(graph);
}

type AdjacencyMatrix = Vec<Vec<f64>>;

fn graph_to_adj_matrix<N: Clone + Debug>(graph: &UnGraph<N, f64>) -> AdjacencyMatrix {
    let mut adj_matrix = vec![vec![0.0; graph.node_count()]; graph.node_count()];
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let weight = graph.edge_weight(edge).unwrap();
        adj_matrix[source.index()][target.index()] = *weight;
        adj_matrix[target.index()][source.index()] = *weight;
    }
    adj_matrix
}

fn two_min_edges(adj_matrix: &AdjacencyMatrix, vertex: usize) -> (f64, f64) {
    let (mut min1, mut min2) = (f64::MAX, f64::MAX);

    for &weight in adj_matrix[vertex].iter() {
        if weight == 0. {
            continue;
        } else if weight < min1 {
            min2 = min1;
            min1 = weight;
        } else if weight < min2 {
            min2 = weight;
        }
    }
    (min1, min2)
}

struct TspState<'a> {
    adj_matrix: &'a AdjacencyMatrix,
    current_bound: f64,
    current_weight: f64,
    current_path: &'a mut Vec<usize>,
}

struct TspResult {
    path: Vec<usize>,
    cost: f64,
}

fn tsp_rec(state: TspState) -> TspResult {}
