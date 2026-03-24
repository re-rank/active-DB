use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
/// Maximal Independent Set (MIS): 서로 인접하지 않는 최대 노드 집합.
/// Greedy 알고리즘: 차수 오름차순으로 노드를 선택 (작은 차수 우선).
/// 반환: MIS에 속하는 노드 ID 목록.
pub fn maximal_independent_set(
    graph: &CompactGraph,
) -> Result<Vec<u128>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    // 차수 오름차순 정렬
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&idx| graph.out_degree(idx) + graph.in_degree(idx));

    let mut in_set = vec![false; n];
    let mut excluded = vec![false; n];

    for &node in &order {
        if excluded[node] {
            continue;
        }

        in_set[node] = true;
        excluded[node] = true;

        // 이웃을 제외
        for &neighbor in graph.out_neighbors(node) {
            excluded[neighbor] = true;
        }
        for &neighbor in graph.in_neighbors(node) {
            excluded[neighbor] = true;
        }
    }

    let result: Vec<u128> = (0..n)
        .filter(|&idx| in_set[idx])
        .map(|idx| graph.to_node_id(idx))
        .collect();

    Ok(result)
}

/// MIS 크기만 반환.
pub fn mis_size(graph: &CompactGraph) -> Result<usize, GraphError> {
    Ok(maximal_independent_set(graph)?.len())
}
