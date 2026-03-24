use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::{HashMap, VecDeque};

/// Edmonds-Karp (BFS 기반 Ford-Fulkerson) 최대 유량 알고리즘.
/// capacity: 엣지별 용량 맵 (없으면 모든 엣지 용량 = 1.0)
pub fn max_flow(
    graph: &CompactGraph,
    source: u128,
    sink: u128,
    capacity: Option<&HashMap<(usize, usize), f64>>,
) -> Result<f64, GraphError> {
    let n = graph.node_count();
    let src_idx = graph
        .to_idx(source)
        .ok_or_else(|| GraphError::AlgorithmError("Source node not found".to_string()))?;
    let sink_idx = graph
        .to_idx(sink)
        .ok_or_else(|| GraphError::AlgorithmError("Sink node not found".to_string()))?;

    // 잔여 용량 그래프 (인접 리스트 대신 HashMap 사용)
    let mut residual: HashMap<(usize, usize), f64> = HashMap::new();

    // 초기 용량 설정
    for i in 0..n {
        for &j in graph.out_neighbors(i) {
            let cap = capacity
                .and_then(|c| c.get(&(i, j)))
                .copied()
                .unwrap_or(1.0);
            *residual.entry((i, j)).or_insert(0.0) += cap;
            // 역방향 엣지 초기화 (없으면 0)
            residual.entry((j, i)).or_insert(0.0);
        }
    }

    // 인접 리스트 (잔여 그래프용)
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(i, j) in residual.keys() {
        if !adj[i].contains(&j) {
            adj[i].push(j);
        }
        if !adj[j].contains(&i) {
            adj[j].push(i);
        }
    }

    let mut total_flow = 0.0;

    // 반복: BFS로 증가 경로 찾기
    loop {
        let path = bfs_find_path(&adj, &residual, src_idx, sink_idx, n);
        match path {
            None => break,
            Some(parent) => {
                // 경로상의 최소 잔여 용량 찾기
                let mut path_flow = f64::INFINITY;
                let mut v = sink_idx;
                while v != src_idx {
                    let u = parent[v];
                    path_flow = path_flow.min(*residual.get(&(u, v)).unwrap_or(&0.0));
                    v = u;
                }

                // 잔여 용량 갱신
                v = sink_idx;
                while v != src_idx {
                    let u = parent[v];
                    *residual.get_mut(&(u, v)).unwrap() -= path_flow;
                    *residual.get_mut(&(v, u)).unwrap() += path_flow;
                    v = u;
                }

                total_flow += path_flow;
            }
        }
    }

    Ok(total_flow)
}

/// BFS로 소스에서 싱크까지의 증가 경로 탐색
fn bfs_find_path(
    adj: &[Vec<usize>],
    residual: &HashMap<(usize, usize), f64>,
    source: usize,
    sink: usize,
    n: usize,
) -> Option<Vec<usize>> {
    let mut visited = vec![false; n];
    let mut parent = vec![usize::MAX; n];
    visited[source] = true;

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if !visited[v] && *residual.get(&(u, v)).unwrap_or(&0.0) > 1e-10 {
                visited[v] = true;
                parent[v] = u;
                if v == sink {
                    return Some(parent);
                }
                queue.push_back(v);
            }
        }
    }

    None
}
