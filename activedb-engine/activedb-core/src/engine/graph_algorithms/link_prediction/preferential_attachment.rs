use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};

/// Preferential Attachment: degree(u) × degree(v).
/// "부익부" 현상을 반영. 허브 노드 간 연결 가능성이 높다고 예측.
pub fn preferential_attachment(
    graph: &CompactGraph,
    node_a_idx: usize,
    node_b_idx: usize,
) -> Result<f64, GraphError> {
    let deg_a = graph.out_degree(node_a_idx) + graph.in_degree(node_a_idx);
    let deg_b = graph.out_degree(node_b_idx) + graph.in_degree(node_b_idx);
    Ok((deg_a * deg_b) as f64)
}

/// 모든 비연결 쌍에 대해 Preferential Attachment 스코어 계산.
pub fn preferential_attachment_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let degrees: Vec<usize> = (0..n)
        .map(|i| graph.out_degree(i) + graph.in_degree(i))
        .collect();

    let mut scores = Vec::new();

    for a in 0..n {
        let neighbors_a: std::collections::HashSet<usize> =
            graph.out_neighbors(a).iter().copied().collect();
        for b in (a + 1)..n {
            if neighbors_a.contains(&b) {
                continue;
            }
            let score = (degrees[a] * degrees[b]) as f64;
            if score > 0.0 {
                scores.push((graph.to_node_id(a), graph.to_node_id(b), score));
            }
        }
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_k);
    Ok(scores)
}
