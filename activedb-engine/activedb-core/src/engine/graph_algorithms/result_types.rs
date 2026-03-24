use serde::Serialize;
use std::collections::HashMap;

/// 알고리즘 실행 결과를 담는 열거형
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AlgorithmResult {
    /// 노드별 f64 점수 (PageRank, Centrality 등)
    NodeScores(Vec<NodeScore>),
    /// 노드별 커뮤니티 ID (Louvain, Label Propagation 등)
    NodeCommunities(Vec<NodeCommunity>),
    /// 노드 쌍 유사도
    PairScores(Vec<PairScore>),
    /// 단일 스칼라 값 (삼각형 수, 전역 클러스터링 계수 등)
    Scalar(ScalarResult),
    /// 엣지 리스트 (MST, Max Flow 등)
    Edges(Vec<EdgeResult>),
    /// 사이클 탐지 결과
    Cycles(CycleResult),
    /// 연결 컴포넌트 (WCC, SCC)
    Components(Vec<ComponentResult>),
}

#[derive(Debug, Serialize)]
pub struct NodeScore {
    pub node_id: String,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct NodeCommunity {
    pub node_id: String,
    pub community_id: u64,
}

#[derive(Debug, Serialize)]
pub struct PairScore {
    pub node_a: String,
    pub node_b: String,
    pub score: f64,
}

#[derive(Debug, Serialize)]
pub struct ScalarResult {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct EdgeResult {
    pub from_node: String,
    pub to_node: String,
    pub weight: f64,
}

#[derive(Debug, Serialize)]
pub struct CycleResult {
    pub has_cycle: bool,
    pub cycle_nodes: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ComponentResult {
    pub component_id: u64,
    pub node_ids: Vec<String>,
}

/// UUID u128을 문자열로 변환하는 헬퍼
pub fn node_id_to_string(id: u128) -> String {
    uuid::Uuid::from_u128(id).to_string()
}

/// 노드 스코어 맵을 정렬된 Vec<NodeScore>로 변환
pub fn scores_to_sorted_vec(scores: &HashMap<u128, f64>) -> Vec<NodeScore> {
    let mut result: Vec<NodeScore> = scores
        .iter()
        .map(|(&id, &score)| NodeScore {
            node_id: node_id_to_string(id),
            score,
        })
        .collect();
    result.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    result
}
