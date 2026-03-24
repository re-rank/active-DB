use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Total Neighbors: |N(u) ∪ N(v)|. 이웃 합집합 크기.
pub fn total_neighbors(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let a_neighbors: HashSet<usize> = graph.out_neighbors(node_a_idx).iter().copied().collect();
    let b_neighbors: HashSet<usize> = graph.out_neighbors(node_b_idx).iter().copied().collect();
    Ok(a_neighbors.union(&b_neighbors).count() as f64)
}

/// 모든 비연결 쌍에 대해 Total Neighbors 스코어 계산.
pub fn total_neighbors_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let neighbors: Vec<HashSet<usize>> = (0..n)
        .map(|i| graph.out_neighbors(i).iter().copied().collect())
        .collect();

    let mut scores = Vec::new();

    for a in 0..n {
        for b in (a + 1)..n {
            if neighbors[a].contains(&b) {
                continue;
            }
            let score = neighbors[a].union(&neighbors[b]).count() as f64;
            if score > 0.0 {
                scores.push((graph.to_node_id(a), graph.to_node_id(b), score));
            }
        }
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_k);
    Ok(scores)
}
