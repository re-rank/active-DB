use crate::engine::{
    graph_algorithms::{
        community::triangle_count::triangle_count,
        compact_graph::CompactGraph,
    },
    types::GraphError,
};
use std::collections::{HashMap, HashSet};

/// 클러스터링 계수: 삼각형 수 / (degree * (degree - 1) / 2)
/// 반환: (전역 클러스터링 계수, 노드별 로컬 클러스터링 계수)
pub fn clustering_coefficient(
    graph: &CompactGraph,
) -> Result<(f64, HashMap<u128, f64>), GraphError> {
    let n = graph.node_count();
    let (_, node_tri) = triangle_count(graph)?;

    let mut local_cc = HashMap::with_capacity(n);
    let mut total_triplets = 0u64;
    let mut total_triangles = 0u64;

    for idx in 0..n {
        let node_id = graph.to_node_id(idx);

        // 무방향 차수
        let mut neighbors = HashSet::new();
        for &j in graph.out_neighbors(idx) {
            neighbors.insert(j);
        }
        for &j in graph.in_neighbors(idx) {
            neighbors.insert(j);
        }
        let deg = neighbors.len();

        let tri = node_tri.get(&node_id).copied().unwrap_or(0);

        if deg < 2 {
            local_cc.insert(node_id, 0.0);
        } else {
            let possible = (deg * (deg - 1)) / 2;
            total_triplets += possible as u64;
            total_triangles += tri;
            local_cc.insert(node_id, tri as f64 / possible as f64);
        }
    }

    let global_cc = if total_triplets > 0 {
        total_triangles as f64 / total_triplets as f64
    } else {
        0.0
    };

    Ok((global_cc, local_cc))
}
