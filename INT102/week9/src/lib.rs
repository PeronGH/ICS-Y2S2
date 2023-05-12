use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt::Debug;
use std::usize::MAX;

pub fn travelling_salesman<N: Clone + Debug>(
    graph: &UnGraph<N, f64>,
    start_node: NodeIndex,
) -> TspResult {
    let adj_matrix = graph_to_adj_matrix(graph);
    tsp_helper(adj_matrix, start_node.index())
}

fn graph_to_adj_matrix<N: Clone + Debug>(graph: &UnGraph<N, f64>) -> Vec<Vec<usize>> {
    let mut adj_matrix = vec![vec![0; graph.node_count()]; graph.node_count()];
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let weight = graph.edge_weight(edge).unwrap();
        adj_matrix[source.index()][target.index()] = *weight as usize;
        adj_matrix[target.index()][source.index()] = *weight as usize;
    }
    adj_matrix
}

#[derive(Clone)]
struct TspState {
    adj: Vec<Vec<usize>>,
    curr_bound: usize,
    curr_weight: usize,
    curr_path: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct TspResult {
    final_res: usize,
    final_path: Vec<usize>,
}

fn find_mins(adj: &Vec<Vec<usize>>, i: usize) -> (usize, usize) {
    let mut others: Vec<usize> = adj[i]
        .iter()
        .enumerate()
        .filter(|(j, _)| *j != i)
        .map(|(_, &item)| item)
        .collect();
    others.sort_unstable();
    let first_min = *others.get(0).unwrap_or(&MAX);
    let second_min = *others.get(1).unwrap_or(&MAX);
    (first_min, second_min)
}

fn tsp_rec(state: TspState) -> TspResult {
    let TspState {
        adj,
        mut curr_path,
        mut curr_bound,
        mut curr_weight,
    } = state;
    let mut final_res = MAX;
    let mut final_path = vec![];

    if curr_path.len() == adj.len() {
        if adj[curr_path[curr_path.len() - 1]][curr_path[0]] != 0 {
            let curr_res = curr_weight + adj[curr_path[curr_path.len() - 1]][curr_path[0]];
            if curr_res < final_res {
                final_path = curr_path.clone();
                final_path.push(curr_path[0]);
                final_res = curr_res;
            }
        }
        return TspResult {
            final_res,
            final_path,
        };
    }

    for i in 0..adj.len() {
        if adj[curr_path[curr_path.len() - 1]][i] != 0 && !curr_path.contains(&i) {
            let temp = curr_bound;
            curr_weight += adj[curr_path[curr_path.len() - 1]][i];

            let (first_min, ..) = find_mins(&adj, i);
            if curr_path.len() == 1 {
                curr_bound -= (find_mins(&adj, curr_path[curr_path.len() - 1]).0 + first_min) / 2;
            } else {
                curr_bound -= (find_mins(&adj, curr_path[curr_path.len() - 1]).1 + first_min) / 2;
            }

            if curr_bound + curr_weight < final_res {
                curr_path.push(i);
                let result = tsp_rec(TspState {
                    adj: adj.clone(),
                    curr_bound,
                    curr_weight,
                    curr_path: curr_path.clone(),
                });
                if result.final_res < final_res {
                    final_res = result.final_res;
                    final_path = result.final_path;
                }
                curr_path.pop();
            }

            curr_weight -= adj[curr_path[curr_path.len() - 1]][i];
            curr_bound = temp;
        }
    }

    TspResult {
        final_res,
        final_path,
    }
}

pub fn tsp_helper(adj: Vec<Vec<usize>>, start_node: usize) -> TspResult {
    let curr_path = vec![start_node];
    let mut curr_bound = 0;

    for i in 0..adj.len() {
        let (first_min, second_min) = find_mins(&adj, i);
        curr_bound += first_min + second_min;
    }

    curr_bound = curr_bound / 2 + curr_bound % 2;

    let state = TspState {
        adj,
        curr_bound,
        curr_weight: 0,
        curr_path,
    };

    tsp_rec(state)
}
