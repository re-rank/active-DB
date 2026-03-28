use std::sync::Arc;

use sonic_rs::{JsonValueTrait, json};

use crate::engine::{
    graph_algorithms::{
        access::GraphAccess,
        centrality::{
            betweenness::betweenness_centrality,
            closeness::closeness_centrality,
            degree::{degree_centrality, DegreeType},
            eigenvector::eigenvector_centrality,
            harmonic::harmonic_centrality,
            pagerank::{pagerank, PageRankConfig},
        },
        community::{
            clustering_coefficient::clustering_coefficient,
            connected_components::{strongly_connected_components, weakly_connected_components},
            k_core::k_core_decomposition,
            label_propagation::label_propagation,
            louvain::louvain,
            triangle_count::triangle_count,
        },
        compact_graph::CompactGraph,
        path_ext::{
            cycle_detection::detect_cycle,
            max_flow::max_flow,
            mst::minimum_spanning_tree,
            bfs::bfs,
            shortest_path::{shortest_path_between, dijkstra},
            astar::astar,
            all_paths::all_paths_between,
            msf::minimum_spanning_forest,
            diameter::estimated_diameter,
        },
        link_prediction::{
            common_neighbors::common_neighbors_all,
            adamic_adar::adamic_adar_all,
            preferential_attachment::preferential_attachment_all,
            resource_allocation::resource_allocation_all,
            total_neighbors::total_neighbors_all,
        },
        classification::{
            graph_coloring::greedy_coloring,
            maximal_independent_set::maximal_independent_set,
        },
        result_types::{
            node_id_to_string, scores_to_sorted_vec, AlgorithmResult, ComponentResult,
            CycleResult, EdgeResult, NodeCommunity, NodeScore, PairScore, ScalarResult,
        },
        similarity::{
            cosine_neighbor::cosine_neighbor_similarity_all,
            jaccard::jaccard_similarity_all,
        },
    },
    types::GraphError,
};
use crate::gateway::router::router::{Handler, HandlerInput, HandlerSubmission};
use crate::protocol;
use crate::utils::label_hash::hash_label;

// ============================================================================
// Helper: 요청에서 edge_label 파싱
// ============================================================================

fn parse_edge_label(params: &sonic_rs::Value) -> Option<[u8; 4]> {
    params
        .get("edge_label")
        .and_then(|v| v.as_str())
        .map(|label| hash_label(label, None))
}

fn parse_usize(params: &sonic_rs::Value, key: &str, default: usize) -> usize {
    params
        .get(key)
        .and_then(|v| v.as_u64())
        .map(|v| v as usize)
        .unwrap_or(default)
}

fn parse_f64(params: &sonic_rs::Value, key: &str, default: f64) -> f64 {
    params
        .get(key)
        .and_then(|v| v.as_f64())
        .unwrap_or(default)
}

fn parse_bool(params: &sonic_rs::Value, key: &str, default: bool) -> bool {
    params
        .get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

fn parse_params(body: &[u8]) -> sonic_rs::Value {
    if body.is_empty() {
        json!({})
    } else {
        sonic_rs::from_slice(body).unwrap_or_else(|_| json!({}))
    }
}

fn build_compact_graph(
    input: &HandlerInput,
    edge_label: Option<&[u8; 4]>,
) -> Result<CompactGraph, GraphError> {
    let db = Arc::clone(&input.graph.storage);
    let txn = db.graph_env.read_txn().map_err(GraphError::from)?;
    CompactGraph::build(&db, &txn, edge_label)
}

fn json_response(result: &AlgorithmResult) -> Result<protocol::Response, GraphError> {
    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": result }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

// ============================================================================
// PageRank
// ============================================================================

pub fn algo_pagerank(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let config = PageRankConfig {
        damping: parse_f64(&params, "damping", 0.85),
        max_iterations: parse_usize(&params, "iterations", 20),
        tolerance: parse_f64(&params, "tolerance", 1e-6),
    };

    let scores = pagerank(&graph, &config)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_pagerank", algo_pagerank, false))
}

// ============================================================================
// Degree Centrality
// ============================================================================

pub fn algo_degree_centrality(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let degree_type = match params.get("type").and_then(|v| v.as_str()).unwrap_or("total") {
        "out" => DegreeType::Out,
        "in" => DegreeType::In,
        _ => DegreeType::Total,
    };

    let db = Arc::clone(&input.graph.storage);
    let txn = db.graph_env.read_txn().map_err(GraphError::from)?;
    let scores = degree_centrality(&db, &txn, degree_type, edge_label.as_ref())?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_degree_centrality", algo_degree_centrality, false))
}

// ============================================================================
// Betweenness Centrality
// ============================================================================

pub fn algo_betweenness(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let sample_size = params
        .get("sample_size")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);
    let normalized = parse_bool(&params, "normalized", true);

    let scores = betweenness_centrality(&graph, sample_size, normalized)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_betweenness", algo_betweenness, false))
}

// ============================================================================
// Closeness Centrality
// ============================================================================

pub fn algo_closeness(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let normalized = parse_bool(&params, "normalized", true);

    let scores = closeness_centrality(&graph, normalized)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_closeness", algo_closeness, false))
}

// ============================================================================
// Eigenvector Centrality
// ============================================================================

pub fn algo_eigenvector(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let max_iter = parse_usize(&params, "iterations", 100);
    let tolerance = parse_f64(&params, "tolerance", 1e-6);

    let scores = eigenvector_centrality(&graph, max_iter, tolerance)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_eigenvector", algo_eigenvector, false))
}

// ============================================================================
// Harmonic Centrality
// ============================================================================

pub fn algo_harmonic(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let normalized = parse_bool(&params, "normalized", true);

    let scores = harmonic_centrality(&graph, normalized)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&scores));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_harmonic", algo_harmonic, false))
}

// ============================================================================
// Louvain Community Detection
// ============================================================================

pub fn algo_louvain(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let max_iter = parse_usize(&params, "max_iterations", 100);

    let communities = louvain(&graph, max_iter)?;
    let result = AlgorithmResult::NodeCommunities(
        communities
            .into_iter()
            .map(|(id, comm)| NodeCommunity {
                node_id: node_id_to_string(id),
                community_id: comm,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_louvain", algo_louvain, false))
}

// ============================================================================
// Label Propagation
// ============================================================================

pub fn algo_label_propagation(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let max_iter = parse_usize(&params, "max_iterations", 100);

    let communities = label_propagation(&graph, max_iter)?;
    let result = AlgorithmResult::NodeCommunities(
        communities
            .into_iter()
            .map(|(id, comm)| NodeCommunity {
                node_id: node_id_to_string(id),
                community_id: comm,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_label_propagation", algo_label_propagation, false))
}

// ============================================================================
// Connected Components (WCC / SCC)
// ============================================================================

pub fn algo_wcc(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let components = weakly_connected_components(&graph)?;

    // 컴포넌트별로 그룹핑
    let mut comp_map: std::collections::HashMap<u64, Vec<String>> =
        std::collections::HashMap::new();
    for (id, comp_id) in components {
        comp_map
            .entry(comp_id)
            .or_default()
            .push(node_id_to_string(id));
    }

    let result = AlgorithmResult::Components(
        comp_map
            .into_iter()
            .map(|(comp_id, node_ids)| ComponentResult {
                component_id: comp_id,
                node_ids,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_wcc", algo_wcc, false))
}

pub fn algo_scc(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let components = strongly_connected_components(&graph)?;

    let mut comp_map: std::collections::HashMap<u64, Vec<String>> =
        std::collections::HashMap::new();
    for (id, comp_id) in components {
        comp_map
            .entry(comp_id)
            .or_default()
            .push(node_id_to_string(id));
    }

    let result = AlgorithmResult::Components(
        comp_map
            .into_iter()
            .map(|(comp_id, node_ids)| ComponentResult {
                component_id: comp_id,
                node_ids,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_scc", algo_scc, false))
}

// ============================================================================
// K-Core Decomposition
// ============================================================================

pub fn algo_k_core(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let coreness = k_core_decomposition(&graph)?;
    let result = AlgorithmResult::NodeScores(
        coreness
            .into_iter()
            .map(|(id, k)| NodeScore {
                node_id: node_id_to_string(id),
                score: k as f64,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_k_core", algo_k_core, false))
}

// ============================================================================
// Triangle Count
// ============================================================================

pub fn algo_triangle_count(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let (total, per_node) = triangle_count(&graph)?;
    let result = json!({
        "total_triangles": total,
        "per_node": per_node.into_iter().map(|(id, count)| {
            json!({
                "node_id": node_id_to_string(id),
                "count": count
            })
        }).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": result }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_triangle_count", algo_triangle_count, false))
}

// ============================================================================
// Clustering Coefficient
// ============================================================================

pub fn algo_clustering_coefficient(
    input: HandlerInput,
) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let (global_cc, local_cc) = clustering_coefficient(&graph)?;
    let result = json!({
        "global_clustering_coefficient": global_cc,
        "per_node": local_cc.into_iter().map(|(id, cc)| {
            json!({
                "node_id": node_id_to_string(id),
                "coefficient": cc
            })
        }).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": result }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_clustering_coefficient", algo_clustering_coefficient, false))
}

// ============================================================================
// Jaccard Similarity
// ============================================================================

pub fn algo_jaccard(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = jaccard_similarity_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_jaccard", algo_jaccard, false))
}

// ============================================================================
// Cosine Neighbor Similarity
// ============================================================================

pub fn algo_cosine_neighbor(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = cosine_neighbor_similarity_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_cosine_neighbor", algo_cosine_neighbor, false))
}

// ============================================================================
// Cycle Detection
// ============================================================================

pub fn algo_cycle_detection(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let (has_cycle, cycle_nodes) = detect_cycle(&graph)?;
    let result = AlgorithmResult::Cycles(CycleResult {
        has_cycle,
        cycle_nodes: cycle_nodes.map(|nodes| nodes.into_iter().map(node_id_to_string).collect()),
    });
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_cycle_detection", algo_cycle_detection, false))
}

// ============================================================================
// Max Flow
// ============================================================================

pub fn algo_max_flow(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let source_str = params
        .get("source")
        .and_then(|v| v.as_str())
        .ok_or_else(|| GraphError::AlgorithmError("source is required".to_string()))?;
    let sink_str = params
        .get("sink")
        .and_then(|v| v.as_str())
        .ok_or_else(|| GraphError::AlgorithmError("sink is required".to_string()))?;

    let source = parse_node_id(source_str)?;
    let sink = parse_node_id(sink_str)?;

    let flow = max_flow(&graph, source, sink, None)?;
    let result = AlgorithmResult::Scalar(ScalarResult {
        name: "max_flow".to_string(),
        value: flow,
    });
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_max_flow", algo_max_flow, false))
}

// ============================================================================
// Minimum Spanning Tree
// ============================================================================

pub fn algo_mst(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let (total_weight, edges) = minimum_spanning_tree(&graph, None)?;
    let result = json!({
        "total_weight": total_weight,
        "edges": edges.into_iter().map(|(from, to, w)| {
            json!({
                "from": node_id_to_string(from),
                "to": node_id_to_string(to),
                "weight": w
            })
        }).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": result }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_mst", algo_mst, false))
}

// ============================================================================
// Helper: UUID 문자열 → u128 파싱
// ============================================================================

fn parse_node_id(s: &str) -> Result<u128, GraphError> {
    match uuid::Uuid::parse_str(s) {
        Ok(uuid) => Ok(uuid.as_u128()),
        Err(_) => s
            .parse::<u128>()
            .map_err(|_| GraphError::AlgorithmError(format!("Invalid node ID: {s}"))),
    }
}

fn parse_required_node(params: &sonic_rs::Value, key: &str) -> Result<u128, GraphError> {
    let s = params
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| GraphError::AlgorithmError(format!("{key} is required")))?;
    parse_node_id(s)
}

// ============================================================================
// BFS
// ============================================================================

pub fn algo_bfs(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let start = parse_required_node(&params, "start")?;

    let distances = bfs(&graph, start)?;
    let result = AlgorithmResult::NodeScores(
        distances
            .into_iter()
            .map(|(id, dist)| NodeScore {
                node_id: node_id_to_string(id),
                score: dist as f64,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_bfs", algo_bfs, false))
}

// ============================================================================
// Shortest Path (single pair)
// ============================================================================

pub fn algo_shortest_path(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let start = parse_required_node(&params, "start")?;
    let end = parse_required_node(&params, "end")?;

    let result = shortest_path_between(&graph, start, end, None)?;
    let response = match result {
        Some((distance, path)) => json!({
            "found": true,
            "distance": distance,
            "path": path.into_iter().map(node_id_to_string).collect::<Vec<_>>()
        }),
        None => json!({ "found": false }),
    };

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": response }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_shortest_path", algo_shortest_path, false))
}

// ============================================================================
// Dijkstra (SSSP)
// ============================================================================

pub fn algo_dijkstra(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let start = parse_required_node(&params, "start")?;

    let distances = dijkstra(&graph, start, None)?;
    let result = AlgorithmResult::NodeScores(scores_to_sorted_vec(&distances));
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_dijkstra", algo_dijkstra, false))
}

// ============================================================================
// A* Search
// ============================================================================

pub fn algo_astar(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let start = parse_required_node(&params, "start")?;
    let end = parse_required_node(&params, "end")?;

    let result = astar(&graph, start, end, None, None)?;
    let response = match result {
        Some((distance, path)) => json!({
            "found": true,
            "distance": distance,
            "path": path.into_iter().map(node_id_to_string).collect::<Vec<_>>()
        }),
        None => json!({ "found": false }),
    };

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": response }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_astar", algo_astar, false))
}

// ============================================================================
// All Paths
// ============================================================================

pub fn algo_all_paths(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let start = parse_required_node(&params, "start")?;
    let end = parse_required_node(&params, "end")?;
    let max_depth = params.get("max_depth").and_then(|v| v.as_u64()).map(|v| v as usize);
    let max_paths = params.get("max_paths").and_then(|v| v.as_u64()).map(|v| v as usize);

    let paths = all_paths_between(&graph, start, end, max_depth, max_paths)?;
    let response = json!({
        "count": paths.len(),
        "paths": paths.into_iter().map(|p| {
            p.into_iter().map(node_id_to_string).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": response }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_all_paths", algo_all_paths, false))
}

// ============================================================================
// Minimum Spanning Forest
// ============================================================================

pub fn algo_msf(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let (total_weight, edges) = minimum_spanning_forest(&graph, None)?;
    let result = json!({
        "total_weight": total_weight,
        "edges": edges.into_iter().map(|(from, to, w)| {
            json!({
                "from": node_id_to_string(from),
                "to": node_id_to_string(to),
                "weight": w
            })
        }).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": result }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_msf", algo_msf, false))
}

// ============================================================================
// Diameter (estimated)
// ============================================================================

pub fn algo_diameter(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let sample_size = params.get("sample_size").and_then(|v| v.as_u64()).map(|v| v as usize);

    let diameter = estimated_diameter(&graph, sample_size)?;
    let result = AlgorithmResult::Scalar(ScalarResult {
        name: "estimated_diameter".to_string(),
        value: diameter as f64,
    });
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_diameter", algo_diameter, false))
}

// ============================================================================
// Common Neighbors (Link Prediction)
// ============================================================================

pub fn algo_common_neighbors(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = common_neighbors_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_common_neighbors", algo_common_neighbors, false))
}

// ============================================================================
// Adamic-Adar (Link Prediction)
// ============================================================================

pub fn algo_adamic_adar(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = adamic_adar_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_adamic_adar", algo_adamic_adar, false))
}

// ============================================================================
// Preferential Attachment (Link Prediction)
// ============================================================================

pub fn algo_preferential_attachment(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = preferential_attachment_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_preferential_attachment", algo_preferential_attachment, false))
}

// ============================================================================
// Resource Allocation (Link Prediction)
// ============================================================================

pub fn algo_resource_allocation(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = resource_allocation_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_resource_allocation", algo_resource_allocation, false))
}

// ============================================================================
// Total Neighbors (Link Prediction)
// ============================================================================

pub fn algo_total_neighbors(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;
    let top_k = parse_usize(&params, "top_k", 100);

    let pairs = total_neighbors_all(&graph, top_k)?;
    let result = AlgorithmResult::PairScores(
        pairs
            .into_iter()
            .map(|(a, b, score)| PairScore {
                node_a: node_id_to_string(a),
                node_b: node_id_to_string(b),
                score,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_total_neighbors", algo_total_neighbors, false))
}

// ============================================================================
// Graph Coloring (Classification)
// ============================================================================

pub fn algo_graph_coloring(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let colors = greedy_coloring(&graph)?;
    let result = AlgorithmResult::NodeCommunities(
        colors
            .into_iter()
            .map(|(id, color)| NodeCommunity {
                node_id: node_id_to_string(id),
                community_id: color,
            })
            .collect(),
    );
    json_response(&result)
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_graph_coloring", algo_graph_coloring, false))
}

// ============================================================================
// Maximal Independent Set (Classification)
// ============================================================================

pub fn algo_maximal_independent_set(input: HandlerInput) -> Result<protocol::Response, GraphError> {
    let params = parse_params(&input.request.body);
    let edge_label = parse_edge_label(&params);
    let graph = build_compact_graph(&input, edge_label.as_ref())?;

    let nodes = maximal_independent_set(&graph)?;
    let response = json!({
        "count": nodes.len(),
        "node_ids": nodes.into_iter().map(node_id_to_string).collect::<Vec<_>>()
    });

    Ok(protocol::Response {
        body: sonic_rs::to_vec(&json!({ "result": response }))
            .map_err(|e| GraphError::New(e.to_string()))?,
        fmt: Default::default(),
    })
}

inventory::submit! {
    HandlerSubmission(Handler::new("algo_maximal_independent_set", algo_maximal_independent_set, false))
}
