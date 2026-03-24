use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Eigenvector Centrality: 인접 행렬의 주 고유벡터를 power iteration으로 계산.
pub fn eigenvector_centrality(
    graph: &CompactGraph,
    max_iterations: usize,
    tolerance: f64,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    // 초기 벡터: 균등
    let mut scores = vec![1.0 / n as f64; n];
    let mut new_scores = vec![0.0f64; n];

    for _ in 0..max_iterations {
        // 각 노드의 새 점수 = 이웃들의 점수 합
        for i in 0..n {
            let mut sum = 0.0;
            for &src in graph.in_neighbors(i) {
                sum += scores[src];
            }
            new_scores[i] = sum;
        }

        // L2 정규화
        let norm: f64 = new_scores.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for s in new_scores.iter_mut() {
                *s /= norm;
            }
        }

        // 수렴 체크
        let diff: f64 = scores
            .iter()
            .zip(new_scores.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        std::mem::swap(&mut scores, &mut new_scores);

        if diff < tolerance {
            break;
        }
    }

    let mut result = HashMap::with_capacity(n);
    for (idx, &score) in scores.iter().enumerate() {
        result.insert(graph.to_node_id(idx), score);
    }
    Ok(result)
}
