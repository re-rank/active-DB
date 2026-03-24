use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::{HashMap, HashSet};

/// 삼각형 수 계산: Node-iterator 방식으로 이웃 집합 교차.
/// 반환: (전체 삼각형 수, 노드별 삼각형 수)
pub fn triangle_count(
    graph: &CompactGraph,
) -> Result<(u64, HashMap<u128, u64>), GraphError> {
    let n = graph.node_count();
    let mut node_triangles = vec![0u64; n];

    // 각 노드의 이웃 집합 (방향 무시) 미리 계산
    let neighbor_sets: Vec<HashSet<usize>> = (0..n)
        .map(|i| {
            let mut set = HashSet::new();
            for &j in graph.out_neighbors(i) {
                set.insert(j);
            }
            for &j in graph.in_neighbors(i) {
                set.insert(j);
            }
            set
        })
        .collect();

    let mut total_triangles = 0u64;

    // 노드 정렬: degree 기준으로 방향 지정 (작은 → 큰)하여 중복 방지
    for u in 0..n {
        for &v in &neighbor_sets[u] {
            if v <= u {
                continue; // u < v 인 쌍만 처리
            }
            // u와 v의 공통 이웃 w (v < w)
            for &w in &neighbor_sets[u] {
                if w <= v {
                    continue;
                }
                if neighbor_sets[v].contains(&w) {
                    // 삼각형 u-v-w 발견
                    total_triangles += 1;
                    node_triangles[u] += 1;
                    node_triangles[v] += 1;
                    node_triangles[w] += 1;
                }
            }
        }
    }

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        result.insert(graph.to_node_id(idx), node_triangles[idx]);
    }

    Ok((total_triangles, result))
}
