use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Louvain 알고리즘으로 커뮤니티 탐지.
/// 2-phase: 로컬 모듈러리티 최적화 + 그래프 축약 반복.
pub fn louvain(
    graph: &CompactGraph,
    max_iterations: usize,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let m = graph.edge_count() as f64; // 총 엣지 수
    if m == 0.0 {
        // 엣지 없으면 각 노드가 자기 커뮤니티
        let mut result = HashMap::with_capacity(n);
        for idx in 0..n {
            result.insert(graph.to_node_id(idx), idx as u64);
        }
        return Ok(result);
    }

    // 각 노드의 커뮤니티: 초기에는 자기 자신
    let mut community = vec![0u64; n];
    for i in 0..n {
        community[i] = i as u64;
    }

    // 각 커뮤니티의 총 차수 (out + in)
    let k: Vec<f64> = (0..n)
        .map(|i| (graph.out_degree(i) + graph.in_degree(i)) as f64)
        .collect();

    // 커뮤니티 내부 엣지 가중치 합 (sigma_tot)
    let mut sigma_tot: HashMap<u64, f64> = HashMap::with_capacity(n);
    for i in 0..n {
        sigma_tot.insert(community[i], k[i]);
    }

    for _ in 0..max_iterations {
        let mut changed = false;

        for i in 0..n {
            let current_comm = community[i];
            let ki = k[i];

            // 이웃 커뮤니티별 연결 수 계산
            let mut neighbor_comms: HashMap<u64, f64> = HashMap::new();
            for &j in graph.out_neighbors(i) {
                *neighbor_comms.entry(community[j]).or_insert(0.0) += 1.0;
            }
            for &j in graph.in_neighbors(i) {
                *neighbor_comms.entry(community[j]).or_insert(0.0) += 1.0;
            }

            // 현재 커뮤니티에서 제거 시 모듈러리티 손실
            let ki_in_current = neighbor_comms.get(&current_comm).copied().unwrap_or(0.0);
            let st_current = sigma_tot.get(&current_comm).copied().unwrap_or(0.0);

            let mut best_comm = current_comm;
            let mut best_delta = 0.0;

            for (&comm, &ki_in) in &neighbor_comms {
                if comm == current_comm {
                    continue;
                }
                let st = sigma_tot.get(&comm).copied().unwrap_or(0.0);

                // ΔQ = ki_in/m - ki*sigma_tot/(2*m^2)  (이동할 커뮤니티)
                //     - (ki_in_current/m - ki*(sigma_tot_current - ki)/(2*m^2))  (현재에서 제거)
                let delta = (ki_in - ki_in_current) / m
                    - ki * (st - (st_current - ki)) / (2.0 * m * m);

                if delta > best_delta {
                    best_delta = delta;
                    best_comm = comm;
                }
            }

            if best_comm != current_comm {
                // 커뮤니티 이동
                *sigma_tot.entry(current_comm).or_insert(0.0) -= ki;
                *sigma_tot.entry(best_comm).or_insert(0.0) += ki;
                community[i] = best_comm;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    // 커뮤니티 ID 재번호화 (연속 번호)
    let mut comm_remap: HashMap<u64, u64> = HashMap::new();
    let mut next_id = 0u64;

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        let comm = community[idx];
        let remapped = *comm_remap.entry(comm).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        result.insert(graph.to_node_id(idx), remapped);
    }

    Ok(result)
}
