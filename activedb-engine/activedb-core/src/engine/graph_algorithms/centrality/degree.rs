use crate::engine::{
    graph_algorithms::access::GraphAccess,
    storage_core::ActiveDBGraphStorage,
    types::GraphError,
};
use heed3::RoTxn;
use std::collections::HashMap;

/// Degree Centrality 종류
pub enum DegreeType {
    Out,
    In,
    Total,
}

/// Degree Centrality: LMDB에서 직접 스트리밍 (CompactGraph 불필요).
/// 정규화: degree / (n - 1)
pub fn degree_centrality(
    storage: &ActiveDBGraphStorage,
    txn: &RoTxn,
    degree_type: DegreeType,
    edge_label: Option<&[u8; 4]>,
) -> Result<HashMap<u128, f64>, GraphError> {
    let all_ids = storage.all_node_ids(txn)?;
    let n = all_ids.len();
    if n <= 1 {
        return Ok(all_ids.into_iter().map(|id| (id, 0.0)).collect());
    }

    let norm = (n - 1) as f64;
    let mut result = HashMap::with_capacity(n);

    for &id in &all_ids {
        let deg = match degree_type {
            DegreeType::Out => storage.out_degree(txn, id, edge_label)?,
            DegreeType::In => storage.in_degree(txn, id, edge_label)?,
            DegreeType::Total => {
                storage.out_degree(txn, id, edge_label)?
                    + storage.in_degree(txn, id, edge_label)?
            }
        };
        result.insert(id, deg as f64 / norm);
    }

    Ok(result)
}
