use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Cosine Similarity (이웃 기반): 이웃 집합을 이진 벡터로 변환 후 코사인 계산.
/// cosine(A, B) = |A ∩ B| / (sqrt(|A|) * sqrt(|B|))
pub fn cosine_neighbor_similarity(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let neighbors_a = get_undirected_neighbors(graph, node_a_idx);
    let neighbors_b = get_undirected_neighbors(graph, node_b_idx);

    if neighbors_a.is_empty() || neighbors_b.is_empty() {
        return Ok(0.0);
    }

    let intersection = neighbors_a.intersection(&neighbors_b).count() as f64;
    let norm_a = (neighbors_a.len() as f64).sqrt();
    let norm_b = (neighbors_b.len() as f64).sqrt();

    Ok(intersection / (norm_a * norm_b))
}

/// 모든 노드 쌍의 Cosine Similarity 계산 (top_k로 제한)
pub fn cosine_neighbor_similarity_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    let mut results = Vec::new();

    let neighbor_sets: Vec<HashSet<usize>> = (0..n)
        .map(|i| get_undirected_neighbors(graph, i))
        .collect();

    for i in 0..n {
        if neighbor_sets[i].is_empty() {
            continue;
        }
        for j in (i + 1)..n {
            if neighbor_sets[j].is_empty() {
                continue;
            }
            let intersection = neighbor_sets[i].intersection(&neighbor_sets[j]).count() as f64;
            if intersection == 0.0 {
                continue;
            }
            let norm_a = (neighbor_sets[i].len() as f64).sqrt();
            let norm_b = (neighbor_sets[j].len() as f64).sqrt();
            let score = intersection / (norm_a * norm_b);
            results.push((graph.to_node_id(i), graph.to_node_id(j), score));
        }
    }

    results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(top_k);

    Ok(results)
}

fn get_undirected_neighbors(graph: &CompactGraph, idx: usize) -> HashSet<usize> {
    let mut set = HashSet::new();
    for &j in graph.out_neighbors(idx) {
        set.insert(j);
    }
    for &j in graph.in_neighbors(idx) {
        set.insert(j);
    }
    set
}
