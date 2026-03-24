use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::{HashMap, VecDeque};

/// Closeness Centrality: 각 노드에서 BFS, 1 / sum(distances).
/// normalized: true면 (n-1) / sum(distances)
pub fn closeness_centrality(
    graph: &CompactGraph,
    normalized: bool,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    let mut result = HashMap::with_capacity(n);

    for s in 0..n {
        let distances = bfs_distances(graph, s);
        let reachable: Vec<u64> = distances.iter().filter(|&&d| d > 0).copied().collect();

        let score = if reachable.is_empty() {
            0.0
        } else {
            let total_dist: u64 = reachable.iter().sum();
            if normalized {
                reachable.len() as f64 / total_dist as f64
            } else {
                1.0 / total_dist as f64
            }
        };

        result.insert(graph.to_node_id(s), score);
    }

    Ok(result)
}

/// BFS로 소스에서 모든 노드까지의 거리 계산
fn bfs_distances(graph: &CompactGraph, source: usize) -> Vec<u64> {
    let n = graph.node_count();
    let mut dist = vec![u64::MAX; n];
    dist[source] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(v) = queue.pop_front() {
        for &w in graph.out_neighbors(v) {
            if dist[w] == u64::MAX {
                dist[w] = dist[v] + 1;
                queue.push_back(w);
            }
        }
    }

    // 도달 불가능 노드는 0으로 처리 (합산에서 제외하기 위해)
    dist.iter()
        .enumerate()
        .map(|(i, &d)| if i == source || d == u64::MAX { 0 } else { d })
        .collect()
}
