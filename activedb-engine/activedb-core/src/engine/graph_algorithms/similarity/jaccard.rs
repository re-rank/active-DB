use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Jaccard Similarity: |교집합| / |합집합| (이웃 기반)
/// 두 노드의 이웃 집합을 비교하여 유사도를 계산.
pub fn jaccard_similarity(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let neighbors_a = get_undirected_neighbors(graph, node_a_idx);
    let neighbors_b = get_undirected_neighbors(graph, node_b_idx);

    if neighbors_a.is_empty() && neighbors_b.is_empty() {
        return Ok(0.0);
    }

    let intersection = neighbors_a.intersection(&neighbors_b).count();
    let union = neighbors_a.union(&neighbors_b).count();

    Ok(intersection as f64 / union as f64)
}

/// 모든 노드 쌍의 Jaccard Similarity 계산 (top_k로 제한)
pub fn jaccard_similarity_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    let mut results = Vec::new();

    // 이웃 집합 사전 계산
    let neighbor_sets: Vec<HashSet<usize>> = (0..n)
        .map(|i| get_undirected_neighbors(graph, i))
        .collect();

    for i in 0..n {
        for j in (i + 1)..n {
            if neighbor_sets[i].is_empty() && neighbor_sets[j].is_empty() {
                continue;
            }
            let intersection = neighbor_sets[i].intersection(&neighbor_sets[j]).count();
            if intersection == 0 {
                continue;
            }
            let union = neighbor_sets[i].union(&neighbor_sets[j]).count();
            let score = intersection as f64 / union as f64;
            results.push((graph.to_node_id(i), graph.to_node_id(j), score));
        }
    }

    // 점수 내림차순 정렬, top_k 제한
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
