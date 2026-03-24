use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Minimum Spanning Forest — 비연결 그래프에서 각 컴포넌트의 MST를 결합.
/// Kruskal 알고리즘 + Union-Find.
pub fn minimum_spanning_forest(
    graph: &CompactGraph,
    weights: Option<&HashMap<(usize, usize), f64>>,
) -> Result<(f64, Vec<(u128, u128, f64)>), GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok((0.0, Vec::new()));
    }

    // Collect all edges with weights
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();
    for u in 0..n {
        for &v in graph.out_neighbors(u) {
            if u < v {
                let w = weights
                    .and_then(|ws| ws.get(&(u, v)))
                    .copied()
                    .unwrap_or(1.0);
                edges.push((w, u, v));
            }
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0u32; n];

    let mut total_weight = 0.0;
    let mut forest_edges = Vec::new();

    for (w, u, v) in edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            union(&mut parent, &mut rank, ru, rv);
            total_weight += w;
            forest_edges.push((graph.to_node_id(u), graph.to_node_id(v), w));
        }
    }

    Ok((total_weight, forest_edges))
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]]; // path compression
        x = parent[x];
    }
    x
}

fn union(parent: &mut [usize], rank: &mut [u32], x: usize, y: usize) {
    match rank[x].cmp(&rank[y]) {
        std::cmp::Ordering::Less => parent[x] = y,
        std::cmp::Ordering::Greater => parent[y] = x,
        std::cmp::Ordering::Equal => {
            parent[y] = x;
            rank[x] += 1;
        }
    }
}
