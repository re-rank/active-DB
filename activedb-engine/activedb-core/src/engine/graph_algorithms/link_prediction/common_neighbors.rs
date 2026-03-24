use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Common Neighbors: 두 노드의 공통 이웃 수.
/// 링크 예측에서 가장 단순하고 직관적인 지표.
pub fn common_neighbors(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let a_neighbors: HashSet<usize> = graph.out_neighbors(node_a_idx).iter().copied().collect();
    let b_neighbors: HashSet<usize> = graph.out_neighbors(node_b_idx).iter().copied().collect();
    Ok(a_neighbors.intersection(&b_neighbors).count() as f64)
}

/// 모든 비연결 노드 쌍에 대해 Common Neighbors 스코어 계산.
pub fn common_neighbors_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let mut scores: Vec<(u128, u128, f64)> = Vec::new();
    let neighbors: Vec<HashSet<usize>> = (0..n)
        .map(|i| graph.out_neighbors(i).iter().copied().collect())
        .collect();

    for a in 0..n {
        for b in (a + 1)..n {
            // Skip if already connected
            if neighbors[a].contains(&b) {
                continue;
            }
            let score = neighbors[a].intersection(&neighbors[b]).count() as f64;
            if score > 0.0 {
                scores.push((graph.to_node_id(a), graph.to_node_id(b), score));
            }
        }
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_k);
    Ok(scores)
}
