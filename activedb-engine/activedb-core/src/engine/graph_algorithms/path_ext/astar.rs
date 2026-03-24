use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A* 최단 경로 탐색.
/// heuristic: 각 노드 인덱스에서 목표까지의 추정 거리.
///   None이면 Dijkstra와 동일하게 동작 (h=0).
pub fn astar(
    graph: &CompactGraph,
    start_id: u128,
    end_id: u128,
    weights: Option<&HashMap<(usize, usize), f64>>,
    heuristic: Option<&dyn Fn(usize) -> f64>,
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

    let h = |idx: usize| -> f64 {
        heuristic.map_or(0.0, |f| f(idx))
    };

    let mut g_score = vec![f64::INFINITY; n];
    g_score[start_idx] = 0.0;

    let mut prev = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();
    heap.push(AStarState {
        f_score: h(start_idx),
        g: 0.0,
        idx: start_idx,
    });

    while let Some(AStarState { idx, g, .. }) = heap.pop() {
        if idx == end_idx {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = end_idx;
            while current != usize::MAX {
                path.push(graph.to_node_id(current));
                current = prev[current];
            }
            path.reverse();
            return Ok(Some((g_score[end_idx], path)));
        }

        if g > g_score[idx] {
            continue;
        }

        for &neighbor in graph.out_neighbors(idx) {
            let edge_w = weights
                .and_then(|w| w.get(&(idx, neighbor)))
                .copied()
                .unwrap_or(1.0);
            let tentative_g = g_score[idx] + edge_w;

            if tentative_g < g_score[neighbor] {
                g_score[neighbor] = tentative_g;
                prev[neighbor] = idx;
                heap.push(AStarState {
                    f_score: tentative_g + h(neighbor),
                    g: tentative_g,
                    idx: neighbor,
                });
            }
        }
    }

    Ok(None)
}

#[derive(PartialEq)]
struct AStarState {
    f_score: f64,
    g: f64,
    idx: usize,
}

impl Eq for AStarState {}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
