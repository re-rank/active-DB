use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};

/// DFS 기반 사이클 탐지.
/// 반환: (사이클 존재 여부, 사이클을 구성하는 노드 ID 목록 (있으면))
pub fn detect_cycle(
    graph: &CompactGraph,
) -> Result<(bool, Option<Vec<u128>>), GraphError> {
    let n = graph.node_count();

    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White, // 미방문
        Gray,  // 방문 중 (스택에 있음)
        Black, // 완료
    }

    let mut color = vec![Color::White; n];
    let mut parent = vec![usize::MAX; n];

    for start in 0..n {
        if color[start] != Color::White {
            continue;
        }

        // 반복적 DFS (스택 오버플로 방지)
        let mut stack = vec![(start, 0usize)]; // (node, neighbor_index)
        color[start] = Color::Gray;

        while let Some((v, ni)) = stack.last_mut() {
            let neighbors = graph.out_neighbors(*v);
            if *ni < neighbors.len() {
                let w = neighbors[*ni];
                *ni += 1;

                if color[w] == Color::Gray {
                    // 백엣지 발견 → 사이클!
                    let mut cycle = vec![graph.to_node_id(w)];
                    let mut cur = *v;
                    while cur != w {
                        cycle.push(graph.to_node_id(cur));
                        cur = parent[cur];
                    }
                    cycle.push(graph.to_node_id(w));
                    cycle.reverse();
                    return Ok((true, Some(cycle)));
                } else if color[w] == Color::White {
                    color[w] = Color::Gray;
                    parent[w] = *v;
                    stack.push((w, 0));
                }
            } else {
                color[*v] = Color::Black;
                stack.pop();
            }
        }
    }

    Ok((false, None))
}
