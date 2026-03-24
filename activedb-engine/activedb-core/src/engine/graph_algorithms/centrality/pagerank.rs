use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rayon::prelude::*;
use std::collections::HashMap;

/// PageRank 알고리즘 파라미터
pub struct PageRankConfig {
    pub damping: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            damping: 0.85,
            max_iterations: 20,
            tolerance: 1e-6,
        }
    }
}

/// Power iteration 기반 PageRank.
/// CompactGraph의 CSR 구조를 활용하여 rayon 병렬 처리.
pub fn pagerank(
    graph: &CompactGraph,
    config: &PageRankConfig,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let d = config.damping;
    let base = (1.0 - d) / n as f64;

    // 초기 점수: 균등 분배
    let mut scores = vec![1.0 / n as f64; n];
    let mut new_scores = vec![0.0f64; n];

    // 댕글링 노드(out-degree=0) 사전 계산
    let dangling: Vec<bool> = (0..n).map(|i| graph.out_degree(i) == 0).collect();

    for _ in 0..config.max_iterations {
        // 댕글링 노드의 총 점수
        let dangling_sum: f64 = dangling
            .iter()
            .enumerate()
            .filter(|&(_, is_dangling)| *is_dangling)
            .map(|(i, _)| scores[i])
            .sum();

        let dangling_contrib = d * dangling_sum / n as f64;

        // 병렬로 각 노드의 새 점수 계산
        new_scores
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, new_score)| {
                let mut sum = 0.0;
                for &src in graph.in_neighbors(i) {
                    let out_deg = graph.out_degree(src);
                    if out_deg > 0 {
                        sum += scores[src] / out_deg as f64;
                    }
                }
                *new_score = base + dangling_contrib + d * sum;
            });

        // 수렴 체크
        let diff: f64 = scores
            .iter()
            .zip(new_scores.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        std::mem::swap(&mut scores, &mut new_scores);

        if diff < config.tolerance {
            break;
        }
    }

    // 연속 인덱스 → 원본 ID 매핑
    let mut result = HashMap::with_capacity(n);
    for (idx, &score) in scores.iter().enumerate() {
        result.insert(graph.to_node_id(idx), score);
    }
    Ok(result)
}
