use crate::engine::{
    graph_algorithms::access::GraphAccess,
    storage_core::ActiveDBGraphStorage,
    types::GraphError,
};
use heed3::RoTxn;
use std::collections::HashMap;

/// CSR(Compressed Sparse Row) 포맷의 인메모리 그래프.
/// 반복 알고리즘(PageRank, Louvain 등)에서 LMDB 반복 접근 대신 사용.
pub struct CompactGraph {
    /// 연속 인덱스 → 원본 u128 노드 ID
    pub idx_to_id: Vec<u128>,
    /// 원본 u128 노드 ID → 연속 인덱스
    pub id_to_idx: HashMap<u128, usize>,

    // CSR for outgoing edges
    /// out_offsets[i] .. out_offsets[i+1] 범위가 노드 i의 나가는 이웃 인덱스
    pub out_offsets: Vec<usize>,
    /// 나가는 이웃의 연속 인덱스 배열
    pub out_targets: Vec<usize>,

    // CSR for incoming edges
    /// in_offsets[i] .. in_offsets[i+1] 범위가 노드 i의 들어오는 이웃 인덱스
    pub in_offsets: Vec<usize>,
    /// 들어오는 이웃의 연속 인덱스 배열
    pub in_targets: Vec<usize>,
}

impl CompactGraph {
    /// LMDB에서 단일 패스로 CSR 그래프를 빌드한다.
    /// edge_label: 특정 라벨만 필터링, None이면 전체 엣지 사용
    pub fn build(
        storage: &ActiveDBGraphStorage,
        txn: &RoTxn,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Self, GraphError> {
        // 1단계: 모든 노드 ID 수집 및 인덱스 매핑
        let all_ids = storage.all_node_ids(txn)?;
        let n = all_ids.len();

        let mut id_to_idx = HashMap::with_capacity(n);
        let mut idx_to_id = Vec::with_capacity(n);
        for (idx, &id) in all_ids.iter().enumerate() {
            id_to_idx.insert(id, idx);
            idx_to_id.push(id);
        }

        // 2단계: outgoing CSR 빌드
        let mut out_offsets = Vec::with_capacity(n + 1);
        let mut out_targets = Vec::new();
        out_offsets.push(0);

        for &id in &idx_to_id {
            let neighbors = storage.out_neighbors(txn, id, edge_label)?;
            for neighbor_id in neighbors {
                if let Some(&idx) = id_to_idx.get(&neighbor_id) {
                    out_targets.push(idx);
                }
            }
            out_offsets.push(out_targets.len());
        }

        // 3단계: incoming CSR 빌드
        let mut in_offsets = Vec::with_capacity(n + 1);
        let mut in_targets = Vec::new();
        in_offsets.push(0);

        for &id in &idx_to_id {
            let neighbors = storage.in_neighbors(txn, id, edge_label)?;
            for neighbor_id in neighbors {
                if let Some(&idx) = id_to_idx.get(&neighbor_id) {
                    in_targets.push(idx);
                }
            }
            in_offsets.push(in_targets.len());
        }

        Ok(Self {
            idx_to_id,
            id_to_idx,
            out_offsets,
            out_targets,
            in_offsets,
            in_targets,
        })
    }

    /// 노드 수
    #[inline]
    pub fn node_count(&self) -> usize {
        self.idx_to_id.len()
    }

    /// 엣지 수
    #[inline]
    pub fn edge_count(&self) -> usize {
        self.out_targets.len()
    }

    /// 노드 idx의 나가는 이웃 인덱스 슬라이스
    #[inline]
    pub fn out_neighbors(&self, idx: usize) -> &[usize] {
        &self.out_targets[self.out_offsets[idx]..self.out_offsets[idx + 1]]
    }

    /// 노드 idx의 들어오는 이웃 인덱스 슬라이스
    #[inline]
    pub fn in_neighbors(&self, idx: usize) -> &[usize] {
        &self.in_targets[self.in_offsets[idx]..self.in_offsets[idx + 1]]
    }

    /// 나가는 차수
    #[inline]
    pub fn out_degree(&self, idx: usize) -> usize {
        self.out_offsets[idx + 1] - self.out_offsets[idx]
    }

    /// 들어오는 차수
    #[inline]
    pub fn in_degree(&self, idx: usize) -> usize {
        self.in_offsets[idx + 1] - self.in_offsets[idx]
    }

    /// 연속 인덱스 → 원본 노드 ID
    #[inline]
    pub fn to_node_id(&self, idx: usize) -> u128 {
        self.idx_to_id[idx]
    }

    /// 원본 노드 ID → 연속 인덱스 (없으면 None)
    #[inline]
    pub fn to_idx(&self, node_id: u128) -> Option<usize> {
        self.id_to_idx.get(&node_id).copied()
    }
}
