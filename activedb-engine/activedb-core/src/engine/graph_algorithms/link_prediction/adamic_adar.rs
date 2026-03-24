use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Adamic-Adar Index: 공통 이웃의 차수 로그 역수 가중합.
/// AA(u,v) = Σ_{w ∈ N(u) ∩ N(v)} 1 / log(degree(w))
/// 적은 연결을 가진 공통 이웃에 더 높은 가중치를 부여.
pub fn adamic_adar(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let a_neighbors: HashSet<usize> = graph.out_neighbors(node_a_idx).iter().copied().collect();
    let b_neighbors: HashSet<usize> = graph.out_neighbors(node_b_idx).iter().copied().collect();

    let mut score = 0.0;
    for &w in a_neighbors.intersection(&b_neighbors) {
        let degree = graph.out_degree(w) + graph.in_degree(w);
        if degree > 1 {
            score += 1.0 / (degree as f64).ln();
        }
    }

    Ok(score)
}

/// 모든 비연결 쌍에 대해 Adamic-Adar 스코어 계산.
pub fn adamic_adar_all(
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

            let mut score = 0.0;
            for &w in neighbors[a].intersection(&neighbors[b]) {
                let degree = graph.out_degree(w) + graph.in_degree(w);
                if degree > 1 {
                    score += 1.0 / (degree as f64).ln();
                }
            }

            if score > 0.0 {
                scores.push((graph.to_node_id(a), graph.to_node_id(b), score));
            }
        }
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_k);
    Ok(scores)
}
