use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Graph-based K-Nearest Neighbors 분류.
/// 레이블이 있는 노드를 기반으로, 레이블이 없는 노드를 k-hop 이웃의 다수결로 분류.
///
/// labels: 알려진 노드 레이블 (node_idx → label).
/// k: 참조할 이웃 수.
/// 반환: 모든 노드의 레이블 (기존 레이블 유지 + 미분류 노드에 레이블 부여).
pub fn knn_classify(
    graph: &CompactGraph,
    labels: &HashMap<usize, u64>,
    k: usize,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut result: HashMap<u128, u64> = HashMap::with_capacity(n);

    // 이미 레이블된 노드는 그대로 유지
    for (&idx, &label) in labels {
        result.insert(graph.to_node_id(idx), label);
    }

    // 미분류 노드에 대해 k-NN 투표
    for idx in 0..n {
        if labels.contains_key(&idx) {
            continue;
        }

        let mut label_votes: HashMap<u64, usize> = HashMap::new();
        let mut count = 0;

        // 나가는 이웃 중 레이블이 있는 것
        for &neighbor in graph.out_neighbors(idx) {
            if count >= k {
                break;
            }
            if let Some(&label) = labels.get(&neighbor) {
                *label_votes.entry(label).or_default() += 1;
                count += 1;
            }
        }

        // 들어오는 이웃도 포함
        for &neighbor in graph.in_neighbors(idx) {
            if count >= k {
                break;
            }
            if let Some(&label) = labels.get(&neighbor) {
                *label_votes.entry(label).or_default() += 1;
                count += 1;
            }
        }

        // 다수결
        if let Some((&best_label, _)) = label_votes.iter().max_by_key(|&(_, &v)| v) {
            result.insert(graph.to_node_id(idx), best_label);
        }
    }

    Ok(result)
}
