use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::{HashMap, VecDeque};

/// BFS (Breadth-First Search)로 시작 노드에서 모든 노드까지의 최소 홉 수를 계산.
/// 반환: (노드 ID → 홉 수) 맵. 도달 불가능한 노드는 포함하지 않음.
pub fn bfs(
    graph: &CompactGraph,
    start_id: u128,
) -> Result<HashMap<u128, usize>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let start_idx = match graph.to_idx(start_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };

    let mut dist = vec![usize::MAX; n];
    dist[start_idx] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start_idx);

    while let Some(current) = queue.pop_front() {
        let next_dist = dist[current] + 1;
        for &neighbor in graph.out_neighbors(current) {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = next_dist;
                queue.push_back(neighbor);
            }
        }
    }

    let mut result = HashMap::new();
    for idx in 0..n {
        if dist[idx] != usize::MAX {
            result.insert(graph.to_node_id(idx), dist[idx]);
        }
    }

    Ok(result)
}

/// BFS 탐색 순서(방문 순서)를 반환.
pub fn bfs_order(
    graph: &CompactGraph,
    start_id: u128,
) -> Result<Vec<u128>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let start_idx = match graph.to_idx(start_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };

    let mut visited = vec![false; n];
    visited[start_idx] = true;
    let mut order = vec![graph.to_node_id(start_idx)];
    let mut queue = VecDeque::new();
    queue.push_back(start_idx);

    while let Some(current) = queue.pop_front() {
        for &neighbor in graph.out_neighbors(current) {
            if !visited[neighbor] {
                visited[neighbor] = true;
                order.push(graph.to_node_id(neighbor));
                queue.push_back(neighbor);
            }
        }
    }

    Ok(order)
}
