use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Kruskal 알고리즘으로 Minimum Spanning Tree(Forest) 계산.
/// 방향 그래프를 무방향으로 취급.
/// weight_fn: 엣지 가중치를 제공하는 함수. None이면 모든 가중치 = 1.0
pub fn minimum_spanning_tree(
    graph: &CompactGraph,
    weights: Option<&HashMap<(usize, usize), f64>>,
) -> Result<(f64, Vec<(u128, u128, f64)>), GraphError> {
    let n = graph.node_count();

    // 모든 엣지를 수집 (무방향: (min, max) 로 정규화)
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for i in 0..n {
        for &j in graph.out_neighbors(i) {
            let (a, b) = if i < j { (i, j) } else { (j, i) };
            if seen.insert((a, b)) {
                let w = weights
                    .and_then(|wm| wm.get(&(i, j)).or_else(|| wm.get(&(j, i))))
                    .copied()
                    .unwrap_or(1.0);
                edges.push((w, a, b));
            }
        }
    }

    // 가중치 기준 정렬
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Union-Find
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0u32; n];

    let mut mst_edges = Vec::new();
    let mut total_weight = 0.0;

    for (w, u, v) in edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            union(&mut parent, &mut rank, ru, rv);
            total_weight += w;
            mst_edges.push((graph.to_node_id(u), graph.to_node_id(v), w));
        }
    }

    Ok((total_weight, mst_edges))
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

fn union(parent: &mut [usize], rank: &mut [u32], a: usize, b: usize) {
    if rank[a] < rank[b] {
        parent[a] = b;
    } else if rank[a] > rank[b] {
        parent[b] = a;
    } else {
        parent[b] = a;
        rank[a] += 1;
    }
}
