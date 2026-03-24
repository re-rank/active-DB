use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::{HashMap, VecDeque};

/// Harmonic Centrality: sum(1/distance) for all reachable nodes.
/// Closeness와 달리 비연결 그래프에서도 잘 동작.
pub fn harmonic_centrality(
    graph: &CompactGraph,
    normalized: bool,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    let mut result = HashMap::with_capacity(n);

    for s in 0..n {
        let dist = bfs_distances(graph, s);
        let mut sum = 0.0;
        for (i, &d) in dist.iter().enumerate() {
            if i != s && d > 0 {
                sum += 1.0 / d as f64;
            }
        }

        let score = if normalized && n > 1 {
            sum / (n - 1) as f64
        } else {
            sum
        };

        result.insert(graph.to_node_id(s), score);
    }

    Ok(result)
}

fn bfs_distances(graph: &CompactGraph, source: usize) -> Vec<u64> {
    let n = graph.node_count();
    let mut dist = vec![0u64; n];
    let mut visited = vec![false; n];
    visited[source] = true;

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(v) = queue.pop_front() {
        for &w in graph.out_neighbors(v) {
            if !visited[w] {
                visited[w] = true;
                dist[w] = dist[v] + 1;
                queue.push_back(w);
            }
        }
    }

    dist
}
