use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// K-Core 분해: 반복적으로 degree < k 인 노드를 제거하여 k-core를 추출.
/// 반환: 각 노드의 coreness (최대 k 값)
pub fn k_core_decomposition(
    graph: &CompactGraph,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    // 각 노드의 현재 차수 (무방향으로 취급: out + in, 중복 제거 없이)
    let degree: Vec<usize> = (0..n)
        .map(|i| graph.out_degree(i) + graph.in_degree(i))
        .collect();

    // 최대 차수
    let max_deg = degree.iter().copied().max().unwrap_or(0);

    // bin-sort 기반 O(m) 알고리즘 (Batagelj-Zaversnik)
    let mut bin: Vec<usize> = vec![0; max_deg + 1];
    for &d in &degree {
        bin[d] += 1;
    }

    // bin을 누적합으로 변환 (시작 위치)
    let mut start = 0;
    for b in bin.iter_mut() {
        let count = *b;
        *b = start;
        start += count;
    }

    // pos[v] = v의 정렬 배열 내 위치, vert[pos] = 노드
    let mut pos = vec![0usize; n];
    let mut vert = vec![0usize; n];
    // bin의 복사본 (현재 삽입 위치 추적용)
    let mut bin_start: Vec<usize> = (0..=max_deg)
        .map(|d| if d <= max_deg {
            // bin[d]의 시작 위치를 재계산
            degree.iter().filter(|&&dd| dd < d).count()
        } else {
            n
        })
        .collect();

    // 재계산: bin_start[d] = degree < d인 노드 수
    for v in 0..n {
        let d = degree[v];
        pos[v] = bin_start[d];
        vert[bin_start[d]] = v;
        bin_start[d] += 1;
    }

    // bin_start를 다시 원래 시작 위치로
    let mut bin_pos: Vec<usize> = vec![0; max_deg + 1];
    let mut s = 0;
    for d in 0..=max_deg {
        bin_pos[d] = s;
        s += degree.iter().filter(|&&dd| dd == d).count();
    }

    // 실제 알고리즘: 가장 작은 차수부터 처리
    let mut coreness = vec![0u64; n];
    let mut sorted_verts: Vec<(usize, usize)> = (0..n).map(|v| (degree[v], v)).collect();
    sorted_verts.sort_unstable();

    let mut current_degree = degree.clone();
    let mut removed = vec![false; n];

    for &(_, v) in &sorted_verts {
        if removed[v] {
            continue;
        }
        coreness[v] = current_degree[v] as u64;
        removed[v] = true;

        // v의 모든 이웃의 차수를 감소
        for &w in graph.out_neighbors(v) {
            if !removed[w] && current_degree[w] > current_degree[v] {
                current_degree[w] -= 1;
            }
        }
        for &w in graph.in_neighbors(v) {
            if !removed[w] && current_degree[w] > current_degree[v] {
                current_degree[w] -= 1;
            }
        }
    }

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        result.insert(graph.to_node_id(idx), coreness[idx]);
    }
    Ok(result)
}
