use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Weakly Connected Components: Union-Find 기반.
/// 방향을 무시하고 연결 컴포넌트를 탐지.
pub fn weakly_connected_components(
    graph: &CompactGraph,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for &j in graph.out_neighbors(i) {
            uf.union(i, j);
        }
    }

    // 컴포넌트 ID 재번호화
    let mut comp_remap: HashMap<usize, u64> = HashMap::new();
    let mut next_id = 0u64;
    let mut result = HashMap::with_capacity(n);

    for idx in 0..n {
        let root = uf.find(idx);
        let comp_id = *comp_remap.entry(root).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        result.insert(graph.to_node_id(idx), comp_id);
    }

    Ok(result)
}

/// Strongly Connected Components: Tarjan 알고리즘.
pub fn strongly_connected_components(
    graph: &CompactGraph,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut state = TarjanState {
        index: 0,
        stack: Vec::new(),
        on_stack: vec![false; n],
        indices: vec![None; n],
        lowlinks: vec![0; n],
        component: vec![0u64; n],
        comp_id: 0,
    };

    for i in 0..n {
        if state.indices[i].is_none() {
            tarjan_dfs(graph, i, &mut state);
        }
    }

    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        result.insert(graph.to_node_id(idx), state.component[idx]);
    }
    Ok(result)
}

struct TarjanState {
    index: usize,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    indices: Vec<Option<usize>>,
    lowlinks: Vec<usize>,
    component: Vec<u64>,
    comp_id: u64,
}

fn tarjan_dfs(graph: &CompactGraph, v: usize, state: &mut TarjanState) {
    state.indices[v] = Some(state.index);
    state.lowlinks[v] = state.index;
    state.index += 1;
    state.stack.push(v);
    state.on_stack[v] = true;

    for &w in graph.out_neighbors(v) {
        if state.indices[w].is_none() {
            tarjan_dfs(graph, w, state);
            state.lowlinks[v] = state.lowlinks[v].min(state.lowlinks[w]);
        } else if state.on_stack[w] {
            state.lowlinks[v] = state.lowlinks[v].min(state.indices[w].unwrap());
        }
    }

    // 루트 노드: SCC 추출
    if state.lowlinks[v] == state.indices[v].unwrap() {
        let comp_id = state.comp_id;
        state.comp_id += 1;

        while let Some(w) = state.stack.pop() {
            state.on_stack[w] = false;
            state.component[w] = comp_id;
            if w == v {
                break;
            }
        }
    }
}

/// Union-Find (Disjoint Set Union) 자료구조
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // path halving
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
    }
}
