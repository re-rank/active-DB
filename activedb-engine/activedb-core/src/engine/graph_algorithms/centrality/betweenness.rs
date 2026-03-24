use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

/// Brandes 알고리즘 + rayon 병렬 BFS로 Betweenness Centrality 계산.
/// sample_size: Some(k)이면 랜덤 k개 소스 노드만 사용 (근사).
pub fn betweenness_centrality(
    graph: &CompactGraph,
    sample_size: Option<usize>,
    normalized: bool,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    if n <= 2 {
        let mut result = HashMap::with_capacity(n);
        for idx in 0..n {
            result.insert(graph.to_node_id(idx), 0.0);
        }
        return Ok(result);
    }

    // 소스 노드 선택
    let sources: Vec<usize> = match sample_size {
        Some(k) if k < n => {
            use rand::seq::SliceRandom;
            let mut rng = rand::rng();
            let mut all: Vec<usize> = (0..n).collect();
            all.shuffle(&mut rng);
            all.truncate(k);
            all
        }
        _ => (0..n).collect(),
    };

    // 각 소스에서 BFS → 부분 betweenness 를 병렬로 계산
    let partial_scores: Vec<Vec<f64>> = sources
        .par_iter()
        .map(|&s| brandes_single_source(graph, s))
        .collect();

    // 합산
    let mut scores = vec![0.0f64; n];
    for partial in &partial_scores {
        for (i, &val) in partial.iter().enumerate() {
            scores[i] += val;
        }
    }

    // 비방향 그래프 보정: /2 (여기선 방향 그래프이므로 생략)
    // 정규화
    if normalized && n > 2 {
        let norm = ((n - 1) * (n - 2)) as f64;
        for s in scores.iter_mut() {
            *s /= norm;
        }
    }

    let mut result = HashMap::with_capacity(n);
    for (idx, &score) in scores.iter().enumerate() {
        result.insert(graph.to_node_id(idx), score);
    }
    Ok(result)
}

/// 단일 소스 Brandes BFS
fn brandes_single_source(graph: &CompactGraph, s: usize) -> Vec<f64> {
    let n = graph.node_count();
    let mut stack = Vec::with_capacity(n);
    let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut sigma = vec![0.0f64; n]; // 최단 경로 수
    let mut dist = vec![-1i64; n];
    let mut delta = vec![0.0f64; n];

    sigma[s] = 1.0;
    dist[s] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(s);

    // BFS
    while let Some(v) = queue.pop_front() {
        stack.push(v);
        for &w in graph.out_neighbors(v) {
            // 처음 방문
            if dist[w] < 0 {
                dist[w] = dist[v] + 1;
                queue.push_back(w);
            }
            // 최단 경로상의 선행자
            if dist[w] == dist[v] + 1 {
                sigma[w] += sigma[v];
                predecessors[w].push(v);
            }
        }
    }

    // 역순으로 의존도 누적
    while let Some(w) = stack.pop() {
        for &v in &predecessors[w] {
            delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
        }
    }

    // 소스 자신은 0
    delta[s] = 0.0;
    delta
}
