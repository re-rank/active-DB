use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Greedy Graph Coloring: 인접 노드에 다른 색을 할당하는 탐욕 알고리즘.
/// 반환: (노드 ID → 색상 번호) 맵.
/// 최적 해를 보장하지는 않지만, O(V+E)로 빠르게 실행.
pub fn greedy_coloring(
    graph: &CompactGraph,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut colors = vec![u64::MAX; n];

    // 차수 내림차순으로 노드를 정렬 (Largest-First Ordering)
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let deg_a = graph.out_degree(a) + graph.in_degree(a);
        let deg_b = graph.out_degree(b) + graph.in_degree(b);
        deg_b.cmp(&deg_a)
    });

    for &node in &order {
        // 이웃이 사용하고 있는 색상 수집
        let mut used = std::collections::HashSet::new();
        for &neighbor in graph.out_neighbors(node) {
            if colors[neighbor] != u64::MAX {
                used.insert(colors[neighbor]);
            }
        }
        for &neighbor in graph.in_neighbors(node) {
            if colors[neighbor] != u64::MAX {
                used.insert(colors[neighbor]);
            }
        }

        // 사용되지 않은 가장 작은 색 할당
        let mut color = 0u64;
        while used.contains(&color) {
            color += 1;
        }
        colors[node] = color;
    }

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        result.insert(graph.to_node_id(idx), colors[idx]);
    }

    Ok(result)
}

/// 사용된 색상 수 반환 (chromatic number의 상한).
pub fn chromatic_number_upper_bound(
    graph: &CompactGraph,
) -> Result<u64, GraphError> {
    let coloring = greedy_coloring(graph)?;
    Ok(coloring.values().copied().max().map_or(0, |m| m + 1))
}
