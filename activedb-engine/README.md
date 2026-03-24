# ActiveDB

> An open-source graph-vector database engine built from scratch in Rust.

ActiveDB는 그래프, 벡터, 전문 검색(BM25)을 하나의 엔진에 통합한 데이터베이스입니다. AI 애플리케이션에서 별도의 그래프 DB, 벡터 DB, 검색 엔진을 조합할 필요 없이 단일 플랫폼으로 RAG 파이프라인을 구축할 수 있습니다.

## 주요 특징

- **Graph + Vector + BM25** — 하나의 스토리지 엔진에서 그래프 탐색, 벡터 유사도 검색, 키워드 검색을 동시에 수행
- **ActiveQL** — PEG 기반 타입 안전 쿼리 언어. 컴파일 타임에 타입 오류를 잡아내고 TypeScript 타입을 자동 생성
- **HNSW 벡터 인덱스** — AVX2 SIMD 가속 코사인 유사도. LMDB 기반 영속 저장
- **17개 그래프 알고리즘** — PageRank, Louvain, Betweenness Centrality 등 분석 알고리즘 내장
- **하이브리드 리랭킹** — RRF, MMR, CrossEncoder 세 가지 전략으로 BM25 + 벡터 결과 통합
- **MCP 서버 내장** — AI 에이전트가 세션 기반으로 그래프를 탐색할 수 있는 Model Context Protocol 지원
- **내장 임베딩** — 외부에서 벡터를 생성할 필요 없이 `Embed` 함수로 텍스트를 벡터화
- **Rust + LMDB** — 제로카피 읽기, 코어 어피니티 워커풀, `bumpalo` 아레나 할당으로 초저지연 달성

## 빠른 시작

### CLI 설치

```bash
curl -sSL "https://install.activedb.dev" | bash
```

### 프로젝트 생성

```bash
mkdir my-app && cd my-app
activedb init
```

### 스키마 & 쿼리 작성

`.hx` 파일에 노드, 엣지, 벡터 스키마와 쿼리를 정의합니다.

```
// 노드 정의
N::User {
    INDEX name: String,
    age: U32
}

// 엣지 정의
E::Follows {
    From: User,
    To: User,
    Properties: {
        since: String
    }
}

// 벡터 노드 정의
V::Document {
    INDEX title: String,
    content: String
}

// 쿼리
QUERY getUser(user_name: String) =>
    user <- N<User>({name: user_name})
    RETURN user

QUERY getUserFollowers(user_name: String) =>
    user <- N<User>({name: user_name})
    followers <- user.In<Follows>
    RETURN followers
```

### 검증 & 배포

```bash
activedb check    # 타입 검증 & 오류 진단
activedb build    # 컴파일
activedb push dev # 인스턴스 배포
```

### SDK로 쿼리

```typescript
import ActiveDB from "activedb-ts";

const client = new ActiveDB();

await client.query("addUser", { name: "Alice", age: 28 });

const user = await client.query("getUser", { user_name: "Alice" });
console.log(user);
```

## 아키텍처

```
activedb-engine/
├── activedb-core/       # 코어 엔진
│   ├── compiler/        #   AQL 파서 → 분석기 → 코드 생성기
│   ├── engine/          #   스토리지, HNSW 벡터, BM25, 그래프 알고리즘, 리랭커
│   ├── gateway/         #   Axum HTTP 서버, 라우터, 워커풀, MCP 서버
│   └── protocol/        #   데이터 직렬화 프로토콜
├── activedb-cli/        # CLI (init, build, check, push, auth, logs, ...)
├── activedb-container/  # 배포 인스턴스 런타임
├── activedb-macros/     # 프로시저 매크로
├── aql-tests/           # AQL 통합 테스트
└── metrics/             # 메트릭 수집
```

## 그래프 알고리즘

| 카테고리 | 알고리즘 |
|---------|---------|
| **중심성** | PageRank, Degree, Betweenness, Closeness, Eigenvector, Harmonic |
| **커뮤니티** | Louvain, Label Propagation, Connected Components, K-Core, Triangle Count, Clustering Coefficient |
| **경로** | Cycle Detection, Max Flow, Minimum Spanning Tree |
| **유사도** | Jaccard, Cosine Neighbor |

## CLI 명령어

| 명령어 | 설명 |
|--------|------|
| `activedb init` | 새 프로젝트 생성 |
| `activedb check` | 스키마 & 쿼리 타입 검증 |
| `activedb build` | AQL → Rust 코드 컴파일 |
| `activedb push <env>` | 인스턴스 배포 |
| `activedb start / stop / restart` | 로컬 인스턴스 관리 |
| `activedb logs` | 인스턴스 로그 조회 (TUI 지원) |
| `activedb status` | 인스턴스 상태 확인 |
| `activedb auth` | GitHub OAuth 인증 |
| `activedb backup` | 데이터 백업 |
| `activedb dashboard` | 웹 대시보드 열기 |
| `activedb migrate` | 스키마 마이그레이션 |

## Docker

```bash
# 프로덕션
docker build -f docker/Dockerfile -t activedb .
docker run -p 6969:6969 activedb

# 개발
docker build -f docker/Dockerfile.dev -t activedb-dev .
docker run -p 6969:6969 activedb-dev
```

## 빌드

```bash
# 전체 빌드
cargo build --workspace

# 릴리스 빌드
cargo build --release --workspace

# 테스트
cargo test --workspace

# Clippy
cargo clippy --workspace -- -D warnings
```

### Feature Flags

| Feature | 설명 |
|---------|------|
| `server` | HTTP 서버 + 컴파일러 + 벡터 (기본값) |
| `production` | API 키 인증 + 서버 |
| `compiler` | AQL 컴파일러만 |
| `vectors` | 벡터 검색 (코사인 유사도) |
| `bench` | Polars 기반 벤치마크 |
| `dev` | 디버그 출력 + 서버 + 벤치마크 |

## 기여하기

[CONTRIBUTING.md](CONTRIBUTING.md)를 참고해주세요.

## 라이선스

[AGPL-3.0](LICENSE) — 소스 코드를 수정하여 서비스를 제공하는 경우 수정본도 공개해야 합니다.

매니지드 서비스가 필요하다면 [ActiveDB Cloud](https://cloud.activedb.dev)를 확인하세요.
