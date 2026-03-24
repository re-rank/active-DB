use crate::engine::{
    storage_core::ActiveDBGraphStorage,
    types::GraphError,
};
use heed3::RoTxn;

/// 그래프 알고리즘이 LMDB 저장소에 접근하기 위한 트레이트.
/// prefix_iter + unpack_adj_edge_data 패턴을 알고리즘 친화적 인터페이스로 래핑.
pub trait GraphAccess {
    /// 전체 노드 ID를 순회한다.
    fn all_node_ids(&self, txn: &RoTxn) -> Result<Vec<u128>, GraphError>;

    /// 전체 노드 수를 반환한다.
    fn node_count(&self, txn: &RoTxn) -> Result<usize, GraphError>;

    /// 주어진 노드의 나가는 이웃 노드 ID를 반환한다.
    /// edge_label이 None이면 모든 라벨의 나가는 엣지를 순회.
    fn out_neighbors(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<u128>, GraphError>;

    /// 주어진 노드의 들어오는 이웃 노드 ID를 반환한다.
    fn in_neighbors(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<u128>, GraphError>;

    /// 나가는 차수
    fn out_degree(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<usize, GraphError>;

    /// 들어오는 차수
    fn in_degree(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<usize, GraphError>;

    /// 가중치 포함 나가는 이웃: (neighbor_id, edge_id) 쌍 반환
    fn out_neighbors_with_edges(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<(u128, u128)>, GraphError>;
}

impl GraphAccess for ActiveDBGraphStorage {
    fn all_node_ids(&self, txn: &RoTxn) -> Result<Vec<u128>, GraphError> {
        let mut ids = Vec::new();
        let iter = self.nodes_db.iter(txn)?;
        for result in iter {
            let (key, _) = result?;
            ids.push(key);
        }
        Ok(ids)
    }

    fn node_count(&self, txn: &RoTxn) -> Result<usize, GraphError> {
        Ok(self.nodes_db.len(txn)? as usize)
    }

    fn out_neighbors(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<u128>, GraphError> {
        let mut neighbors = Vec::new();
        let prefix = match edge_label {
            Some(label) => Self::out_edge_key(&node_id, label).to_vec(),
            None => node_id.to_be_bytes().to_vec(),
        };

        let iter = self.out_edges_db.prefix_iter(txn, &prefix)?;
        for result in iter {
            let (_, value) = result?;
            let (_edge_id, to_node) = Self::unpack_adj_edge_data(value)?;
            neighbors.push(to_node);
        }
        Ok(neighbors)
    }

    fn in_neighbors(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<u128>, GraphError> {
        let mut neighbors = Vec::new();
        let prefix = match edge_label {
            Some(label) => Self::in_edge_key(&node_id, label).to_vec(),
            None => node_id.to_be_bytes().to_vec(),
        };

        let iter = self.in_edges_db.prefix_iter(txn, &prefix)?;
        for result in iter {
            let (_, value) = result?;
            let (_edge_id, from_node) = Self::unpack_adj_edge_data(value)?;
            neighbors.push(from_node);
        }
        Ok(neighbors)
    }

    fn out_degree(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<usize, GraphError> {
        let prefix = match edge_label {
            Some(label) => Self::out_edge_key(&node_id, label).to_vec(),
            None => node_id.to_be_bytes().to_vec(),
        };
        let iter = self.out_edges_db.prefix_iter(txn, &prefix)?;
        Ok(iter.count())
    }

    fn in_degree(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<usize, GraphError> {
        let prefix = match edge_label {
            Some(label) => Self::in_edge_key(&node_id, label).to_vec(),
            None => node_id.to_be_bytes().to_vec(),
        };
        let iter = self.in_edges_db.prefix_iter(txn, &prefix)?;
        Ok(iter.count())
    }

    fn out_neighbors_with_edges(
        &self,
        txn: &RoTxn,
        node_id: u128,
        edge_label: Option<&[u8; 4]>,
    ) -> Result<Vec<(u128, u128)>, GraphError> {
        let mut neighbors = Vec::new();
        let prefix = match edge_label {
            Some(label) => Self::out_edge_key(&node_id, label).to_vec(),
            None => node_id.to_be_bytes().to_vec(),
        };

        let iter = self.out_edges_db.prefix_iter(txn, &prefix)?;
        for result in iter {
            let (_, value) = result?;
            let (edge_id, to_node) = Self::unpack_adj_edge_data(value)?;
            neighbors.push((to_node, edge_id));
        }
        Ok(neighbors)
    }
}
