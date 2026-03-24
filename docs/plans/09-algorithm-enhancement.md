# ActiveDB 그래프 알고리즘 고도화 기획

## 1. 현황 분석

### ActiveDB 현재 알고리즘 (17개)

| 카테고리 | 알고리즘 | 파일 |
|---------|---------|------|
| Centrality (6) | PageRank, Degree, Betweenness, Closeness, Eigenvector, Harmonic | `engine/graph_algorithms/centrality/` |
| Community (6) | Louvain, Label Propagation, Connected Components, K-Core, Triangle Count, Clustering Coefficient | `engine/graph_algorithms/community/` |
| Path (3) | Cycle Detection, Max Flow, Minimum Spanning Tree | `engine/graph_algorithms/path_ext/` |
| Similarity (2) | Jaccard, Cosine Neighbor | `engine/graph_algorithms/similarity/` |

### TigerGraph 알고리즘 라이브러리 (70+개, 8개 카테고리)

| 카테고리 | 알고리즘 |
|---------|---------|
| Centrality (8) | PageRank, Degree, Betweenness, Closeness, Eigenvector, Harmonic, **Article Rank**, **Influence Maximization** |
| Community (9) | Louvain, Label Propagation, Connected Components, K-Core, Triangle Count, Clustering Coefficient, **K-Means**, **Map Equation**, **SLPA (Speaker-Listener Label Propagation)** |
| Path (10) | Cycle Detection, Max Flow, MST, **BFS**, **A* Shortest Path**, **Shortest Path (Weighted/Unweighted)**, **Path Between Two Vertices**, **Estimated Diameter**, **Cycle Component**, **Minimum Spanning Forest** |
| Similarity (3) | Jaccard, Cosine, **Approximate Nearest Neighbors** |
| Classification (3) | **Greedy Graph Coloring**, **K-Nearest Neighbors**, **Maximal Independent Set** |
| Link Prediction (6) | **Adamic-Adar**, **Common Neighbors**, **Preferential Attachment**, **Resource Allocation**, **Same Community**, **Total Neighbors** |
| Embeddings (3) | **FastRP**, **Weisfeiler-Lehman**, **Embedding Similarity** |
| Patterns | **패턴 매칭 쿼리** |

> **굵은 글씨** = ActiveDB에 없는 알고리즘

---

## 2. 갭 분석 (ActiveDB에 없는 것)

### 완전히 누락된 카테고리 (3개)
- **Classification** — 그래프 컬러링, KNN, 최대 독립 집합
- **Link Prediction** — 링크 예측 6개 알고리즘 전체
- **Embeddings** — 그래프 임베딩 3개 알고리즘 전체

### 기존 카테고리 내 누락 알고리즘
- **Centrality**: Article Rank, Influence Maximization
- **Community**: K-Means, Map Equation, SLPA
- **Path**: BFS, A*, Shortest Path 변형들, Estimated Diameter, MST Forest, Path Between Two Vertices
- **Similarity**: Approximate Nearest Neighbors (ANN)

---

## 3. 고도화 로드맵

### Phase 1 — 핵심 경로 알고리즘 (우선순위: 높음)

가장 빈번하게 사용되며 기본 그래프 DB에 반드시 있어야 하는 알고리즘.

| 알고리즘 | 난이도 | 설명 | 구현 위치 |
|---------|-------|------|----------|
| **BFS (Breadth-First Search)** | 낮음 | 너비 우선 탐색. 모든 그래프 분석의 기초 | `path_ext/bfs.rs` |
| **Shortest Path (Dijkstra)** | 중간 | 가중 최단 경로. 네트워크 분석 필수 | `path_ext/shortest_path.rs` |
| **A* Search** | 중간 | 휴리스틱 기반 최단 경로. 지리/네트워크 라우팅 | `path_ext/astar.rs` |
| **Path Between Two Vertices** | 낮음 | 두 노드 간 모든 경로 탐색 | `path_ext/all_paths.rs` |
| **Minimum Spanning Forest** | 낮음 | 비연결 그래프 대응 MST 확장 | `path_ext/msf.rs` |
| **Estimated Diameter** | 중간 | 그래프 지름 추정 (BFS 샘플링) | `path_ext/diameter.rs` |

**예상 작업량**: 2~3주
**검증**: 각 알고리즘별 단위 테스트 + `aql-tests/`에 통합 테스트 추가

### Phase 2 — Link Prediction (우선순위: 높음)

AI/ML 파이프라인에서 그래프 기반 추천 시스템의 핵심. ActiveDB의 RAG 포지셔닝과 직결.

| 알고리즘 | 난이도 | 설명 |
|---------|-------|------|
| **Common Neighbors** | 낮음 | 공통 이웃 수 기반 링크 예측 |
| **Jaccard Prediction** | 낮음 | 기존 Jaccard 유사도를 링크 예측에 활용 |
| **Adamic-Adar** | 낮음 | 공통 이웃의 연결 수 역수 가중합 |
| **Preferential Attachment** | 낮음 | 차수 곱 기반 예측 (허브 노드 선호) |
| **Resource Allocation** | 낮음 | Adamic-Adar의 변형 (역수 대신 1/degree) |
| **Total Neighbors** | 낮음 | 이웃 합집합 크기 기반 |

**구현 위치**: `engine/graph_algorithms/link_prediction/` (새 모듈)
**예상 작업량**: 1~2주 (대부분 기존 traversal 인프라 재사용 가능)
**검증**: 알려진 그래프 데이터셋(Karate Club 등)으로 정확도 검증

### Phase 3 — Classification (우선순위: 중간)

그래프 구조 분석 및 최적화 문제 해결.

| 알고리즘 | 난이도 | 설명 |
|---------|-------|------|
| **Greedy Graph Coloring** | 중간 | 인접 노드에 다른 색 할당. 스케줄링, 레지스터 할당 등 |
| **K-Nearest Neighbors** | 중간 | 그래프 구조 기반 KNN 분류 |
| **Maximal Independent Set** | 중간 | 서로 인접하지 않는 최대 노드 집합 |

**구현 위치**: `engine/graph_algorithms/classification/` (새 모듈)
**예상 작업량**: 1~2주

### Phase 4 — Graph Embeddings (우선순위: 중간)

ActiveDB의 벡터 엔진(HNSW)과 결합하면 강력한 차별점이 됨.

| 알고리즘 | 난이도 | 설명 |
|---------|-------|------|
| **FastRP (Fast Random Projection)** | 높음 | 노드를 저차원 벡터로 임베딩. 대규모 그래프에서 빠른 근사 |
| **Node2Vec** | 높음 | 랜덤 워크 기반 노드 임베딩 (skip-gram) |
| **Weisfeiler-Lehman** | 높음 | 그래프 동형 판별용 해시 기반 임베딩 |

**핵심 연동**: 생성된 임베딩 → HNSW 인덱스에 자동 저장 → 벡터 유사도 검색 가능
**구현 위치**: `engine/graph_algorithms/embeddings/` (새 모듈)
**예상 작업량**: 3~4주

### Phase 5 — 커뮤니티/중심성 보강 (우선순위: 낮음)

기존 카테고리의 완성도를 높이는 추가 알고리즘.

| 알고리즘 | 카테고리 | 난이도 |
|---------|---------|-------|
| **Article Rank** | Centrality | 낮음 |
| **Influence Maximization** | Centrality | 높음 |
| **SLPA** | Community | 중간 |
| **Map Equation (InfoMap)** | Community | 높음 |
| **K-Means (그래프 기반)** | Community | 중간 |
| **Strongly Connected Components** | Community | 중간 |

**예상 작업량**: 2~3주

---

## 4. 구현 가이드라인

### 디렉토리 구조 (목표)

```
activedb-core/src/engine/graph_algorithms/
├── centrality/            # 기존 6 + 2 = 8개
│   ├── pagerank.rs
│   ├── degree.rs
│   ├── betweenness.rs
│   ├── closeness.rs
│   ├── eigenvector.rs
│   ├── harmonic.rs
│   ├── article_rank.rs       ← NEW
│   └── influence_max.rs      ← NEW
├── community/             # 기존 6 + 4 = 10개
│   ├── louvain.rs
│   ├── label_propagation.rs
│   ├── connected_components.rs
│   ├── k_core.rs
│   ├── triangle_count.rs
│   ├── clustering_coefficient.rs
│   ├── slpa.rs               ← NEW
│   ├── map_equation.rs       ← NEW
│   ├── k_means.rs            ← NEW
│   └── strongly_connected.rs ← NEW
├── path_ext/              # 기존 3 + 6 = 9개
│   ├── cycle_detection.rs
│   ├── max_flow.rs
│   ├── mst.rs
│   ├── bfs.rs                ← NEW
│   ├── shortest_path.rs      ← NEW
│   ├── astar.rs              ← NEW
│   ├── all_paths.rs          ← NEW
│   ├── msf.rs                ← NEW
│   └── diameter.rs           ← NEW
├── similarity/            # 기존 2 (변경 없음)
│   ├── jaccard.rs
│   └── cosine_neighbor.rs
├── link_prediction/       ← NEW 카테고리
│   ├── common_neighbors.rs
│   ├── adamic_adar.rs
│   ├── preferential_attachment.rs
│   ├── resource_allocation.rs
│   ├── total_neighbors.rs
│   └── mod.rs
├── classification/        ← NEW 카테고리
│   ├── graph_coloring.rs
│   ├── knn.rs
│   ├── maximal_independent_set.rs
│   └── mod.rs
├── embeddings/            ← NEW 카테고리
│   ├── fast_rp.rs
│   ├── node2vec.rs
│   ├── weisfeiler_lehman.rs
│   └── mod.rs
├── access.rs
├── compact_graph.rs
├── result_types.rs
└── mod.rs
```

### 코드 컨벤션

1. **공통 trait 사용**: 모든 알고리즘은 `GraphAlgorithm` trait을 구현
2. **CompactGraph 활용**: 기존 `compact_graph.rs`의 인접 리스트 구조 재사용
3. **병렬 처리**: `rayon`을 사용한 병렬 반복 (대규모 그래프 대응)
4. **제네릭 가중치**: 가중/비가중 그래프 모두 지원하는 인터페이스 설계
5. **결과 타입**: `result_types.rs`에 정의된 공통 결과 구조체 확장

### 테스트 전략

- 각 알고리즘별 `*_test.rs` 파일 작성
- 알려진 그래프(Karate Club, Les Misérables 등)로 결과 검증
- `criterion` 벤치마크 추가 (10K, 100K, 1M 노드 스케일)

---

## 5. 우선순위 요약

| Phase | 카테고리 | 추가 알고리즘 수 | 예상 기간 | 누적 총 알고리즘 |
|-------|---------|----------------|----------|---------------|
| 1 | Path 보강 | +6 | 2~3주 | 23개 |
| 2 | Link Prediction (신규) | +6 | 1~2주 | 29개 |
| 3 | Classification (신규) | +3 | 1~2주 | 32개 |
| 4 | Embeddings (신규) | +3 | 3~4주 | 35개 |
| 5 | Centrality/Community 보강 | +6 | 2~3주 | 41개 |
| **합계** | | **+24** | **9~14주** | **41개** |

---

## 6. 차별화 전략 (ActiveDB만의 강점)

TigerGraph와 단순 알고리즘 수로 경쟁하는 것이 아닌, ActiveDB의 고유 아키텍처를 활용한 차별화:

### 6.1 그래프 임베딩 → HNSW 자동 연동
- FastRP/Node2Vec로 생성한 노드 임베딩을 HNSW 벡터 인덱스에 자동 저장
- `QUERY findSimilarNodes(node_id) => embeddings -> vector_search` 패턴
- TigerGraph에 없는 그래프+벡터 네이티브 통합

### 6.2 Link Prediction + BM25 하이브리드
- 그래프 구조적 링크 예측 + 텍스트 유사도(BM25)를 RRF로 결합
- 지식 그래프 기반 RAG에서 관련 엔티티 추천에 활용

### 6.3 MCP를 통한 알고리즘 노출
- 모든 그래프 알고리즘을 MCP 도구로 자동 등록
- AI 에이전트가 자연어로 "이 네트워크에서 영향력 있는 노드를 찾아줘" → PageRank 자동 실행
- TigerGraph는 MCP 미지원

### 6.4 AQL에서 알고리즘 직접 호출
```
QUERY influencers() =>
    users <- N<User>
    ranked <- users.PageRank(iterations: 20, damping: 0.85)
    RETURN ranked.Top(10)
```
- 쿼리 언어 레벨에서 알고리즘을 1급 함수로 지원
- TigerGraph의 GSQL은 별도 설치/호출 과정 필요
