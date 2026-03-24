use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::VecDeque;

/// 그래프 지름 추정 (BFS 샘플링 기반).
/// sample_size: BFS 시작점 수. None이면 min(노드 수, 10).
/// 반환: 추정 지름 (최장 최단 경로의 홉 수).
pub fn estimated_diameter(
    graph: &CompactGraph,
    sample_size: Option<usize>,
) -> Result<usize, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(0);
    }

    let samples = sample_size.unwrap_or(n.min(10));
    let step = if samples >= n { 1 } else { n / samples };

    let mut max_dist = 0usize;

    for i in 0..samples {
        let start = (i * step) % n;
        let furthest = bfs_eccentricity(graph, start);
        max_dist = max_dist.max(furthest);
    }

    Ok(max_dist)
}

/// 단일 노드에서 BFS로 이심률(eccentricity)을 계산.
fn bfs_eccentricity(graph: &CompactGraph, start: usize) -> usize {
    let n = graph.node_count();
    let mut dist = vec![usize::MAX; n];
    dist[start] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut max_dist = 0;

    while let Some(current) = queue.pop_front() {
        let next = dist[current] + 1;
        for &neighbor in graph.out_neighbors(current) {
            if dist[neighbor] == usize::MAX {
                dist[neighbor] = next;
                max_dist = max_dist.max(next);
                queue.push_back(neighbor);
            }
        }
    }

    max_dist
}
