use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Dijkstra 최단 경로 (가중 그래프).
/// weights: (source_idx, target_idx) → weight. None이면 모든 엣지 가중치 1.0.
/// 반환: (노드 ID → 최단 거리) 맵.
pub fn dijkstra(
    graph: &CompactGraph,
    start_id: u128,
    weights: Option<&HashMap<(usize, usize), f64>>,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let start_idx = match graph.to_idx(start_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };

    let mut dist = vec![f64::INFINITY; n];
    dist[start_idx] = 0.0;

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0.0, idx: start_idx });

    while let Some(State { cost, idx }) = heap.pop() {
        if cost > dist[idx] {
            continue;
        }

        for &neighbor in graph.out_neighbors(idx) {
            let edge_weight = weights
                .and_then(|w| w.get(&(idx, neighbor)))
                .copied()
                .unwrap_or(1.0);
            let new_dist = dist[idx] + edge_weight;

            if new_dist < dist[neighbor] {
                dist[neighbor] = new_dist;
                heap.push(State { cost: new_dist, idx: neighbor });
            }
        }
    }

    let mut result = HashMap::new();
    for idx in 0..n {
        if dist[idx].is_finite() {
            result.insert(graph.to_node_id(idx), dist[idx]);
        }
    }

    Ok(result)
}

/// 두 노드 간 최단 경로 (노드 ID 리스트).
pub fn shortest_path_between(
    graph: &CompactGraph,
    start_id: u128,
    end_id: u128,
    weights: Option<&HashMap<(usize, usize), f64>>,
) -> Result<Option<(f64, Vec<u128>)>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(None);
    }

    let start_idx = match graph.to_idx(start_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };
    let end_idx = match graph.to_idx(end_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };

    let mut dist = vec![f64::INFINITY; n];
    let mut prev = vec![usize::MAX; n];
    dist[start_idx] = 0.0;

    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0.0, idx: start_idx });

    while let Some(State { cost, idx }) = heap.pop() {
        if idx == end_idx {
            break;
        }
        if cost > dist[idx] {
            continue;
        }

        for &neighbor in graph.out_neighbors(idx) {
            let w = weights
                .and_then(|w| w.get(&(idx, neighbor)))
                .copied()
                .unwrap_or(1.0);
            let new_dist = dist[idx] + w;
            if new_dist < dist[neighbor] {
                dist[neighbor] = new_dist;
                prev[neighbor] = idx;
                heap.push(State { cost: new_dist, idx: neighbor });
            }
        }
    }

    if dist[end_idx].is_infinite() {
        return Ok(None);
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut current = end_idx;
    while current != usize::MAX {
        path.push(graph.to_node_id(current));
        current = prev[current];
    }
    path.reverse();

    Ok(Some((dist[end_idx], path)))
}

#[derive(PartialEq)]
struct State {
    cost: f64,
    idx: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
