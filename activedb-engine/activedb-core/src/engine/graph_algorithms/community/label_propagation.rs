use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::HashMap;

/// Label Propagation 알고리즘.
/// 각 노드가 이웃들의 최빈 라벨을 채택, 수렴할 때까지 반복.
pub fn label_propagation(
    graph: &CompactGraph,
    max_iterations: usize,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    // 초기: 각 노드의 라벨 = 자기 인덱스
    let mut labels: Vec<u64> = (0..n as u64).collect();
    let mut rng = rand::rng();

    // 순회 순서 셔플용
    let mut order: Vec<usize> = (0..n).collect();

    for _ in 0..max_iterations {
        let mut changed = false;
        order.shuffle(&mut rng);

        for &i in &order {
            // 모든 이웃의 라벨 수집 (방향 무시)
            let mut label_counts: HashMap<u64, usize> = HashMap::new();

            for &j in graph.out_neighbors(i) {
                *label_counts.entry(labels[j]).or_insert(0) += 1;
            }
            for &j in graph.in_neighbors(i) {
                *label_counts.entry(labels[j]).or_insert(0) += 1;
            }

            if label_counts.is_empty() {
                continue;
            }

            // 최빈 라벨 선택 (동률 시 랜덤)
            let max_count = *label_counts.values().max().unwrap();
            let candidates: Vec<u64> = label_counts
                .into_iter()
                .filter(|(_, count)| *count == max_count)
                .map(|(label, _)| label)
                .collect();

            let new_label = *candidates.choose(&mut rng).unwrap();
            if labels[i] != new_label {
                labels[i] = new_label;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    // 라벨 재번호화
    let mut label_remap: HashMap<u64, u64> = HashMap::new();
    let mut next_id = 0u64;

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        let label = labels[idx];
        let remapped = *label_remap.entry(label).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        result.insert(graph.to_node_id(idx), remapped);
    }

    Ok(result)
}
