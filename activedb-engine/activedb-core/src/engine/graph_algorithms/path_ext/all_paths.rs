use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};

/// 두 노드 사이의 모든 단순 경로를 탐색 (DFS + 백트래킹).
/// max_depth: 최대 경로 길이 제한 (None이면 노드 수).
/// max_paths: 최대 반환 경로 수 (None이면 무제한).
pub fn all_paths_between(
    graph: &CompactGraph,
    start_id: u128,
    end_id: u128,
    max_depth: Option<usize>,
    max_paths: Option<usize>,
) -> Result<Vec<Vec<u128>>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let start_idx = match graph.to_idx(start_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };
    let end_idx = match graph.to_idx(end_id) {
        Some(idx) => idx,
        None => return Err(GraphError::NodeNotFound),
    };

    let depth_limit = max_depth.unwrap_or(n);
    let path_limit = max_paths.unwrap_or(usize::MAX);

    let mut results = Vec::new();
    let mut visited = vec![false; n];
    let mut current_path = vec![start_idx];
    visited[start_idx] = true;

    dfs(
        graph,
        end_idx,
        &mut visited,
        &mut current_path,
        &mut results,
        depth_limit,
        path_limit,
    );

    let paths = results
        .into_iter()
        .map(|path| path.into_iter().map(|idx| graph.to_node_id(idx)).collect())
        .collect();

    Ok(paths)
}

fn dfs(
    graph: &CompactGraph,
    end_idx: usize,
    visited: &mut [bool],
    current_path: &mut Vec<usize>,
    results: &mut Vec<Vec<usize>>,
    max_depth: usize,
    max_paths: usize,
) {
    let current = *current_path.last().unwrap();

    if current == end_idx {
        results.push(current_path.clone());
        return;
    }

    if current_path.len() > max_depth || results.len() >= max_paths {
        return;
    }

    for &neighbor in graph.out_neighbors(current) {
        if !visited[neighbor] {
            visited[neighbor] = true;
            current_path.push(neighbor);
            dfs(graph, end_idx, visited, current_path, results, max_depth, max_paths);
            current_path.pop();
            visited[neighbor] = false;
        }
    }
}
