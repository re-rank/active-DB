# 02. activedb-engine 저장소 구조

## 1. 목표 디렉토리 구조

```
activedb-engine/
├── Cargo.toml                    # Workspace manifest
├── Cargo.lock
├── LICENSE                       # AGPL-3.0
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGELOG.md
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                # PR 검증 (clippy, test, fmt)
│   │   ├── release.yml           # 릴리스 빌드 + crates.io 배포
│   │   ├── docker.yml            # Docker 이미지 빌드 + push
│   │   └── hql-tests.yml         # HQL 통합 테스트
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── activedb-core/                # 코어 DB 엔진 (현 helix-db)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── engine/               # 쿼리 실행 엔진
│       │   ├── storage_core/     # LMDB 스토리지
│       │   ├── traversal_core/   # 그래프 순회
│       │   ├── vector_core/      # 벡터 검색
│       │   ├── bm25/             # 전문검색
│       │   ├── graph_algorithms/ # 그래프 알고리즘
│       │   └── reranker/         # 리랭킹
│       ├── compiler/             # AQL 컴파일러
│       │   ├── parser/           # pest 기반 파서
│       │   ├── analyzer/         # 타입 체크, 검증
│       │   └── generator/        # 코드 생성
│       ├── gateway/              # API 게이트웨이
│       ├── protocol/             # 와이어 프로토콜
│       └── utils/
├── activedb-cli/                 # CLI 도구 (현 helix-cli)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── init.rs
│       │   ├── build.rs
│       │   ├── push.rs
│       │   ├── start.rs
│       │   ├── stop.rs
│       │   ├── auth.rs           # GitHub OAuth 유지
│       │   ├── status.rs
│       │   ├── logs.rs
│       │   └── integrations/
│       │       ├── ecr.rs        # AWS ECR
│       │       ├── fly.rs        # Fly.io
│       │       └── docker.rs     # 로컬 Docker/Podman
│       ├── config.rs             # activedb.toml 파싱
│       ├── project.rs
│       └── output.rs
├── activedb-container/           # 컨테이너 런타임 (현 helix-container)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── queries.rs
├── activedb-macros/              # 프로시저 매크로 (현 helix-macros)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── aql-tests/                    # AQL 통합 테스트 (현 hql-tests)
│   ├── Cargo.toml
│   └── src/
├── metrics/                      # 메트릭 수집
│   ├── Cargo.toml
│   └── src/
├── docker/
│   ├── Dockerfile                # 프로덕션 이미지
│   └── Dockerfile.dev            # 개발용 이미지
└── examples/
    ├── basic-graph/
    ├── vector-search/
    └── full-text-search/
```

---

## 2. Cargo Workspace 설정

### 루트 Cargo.toml
```toml
[workspace]
members = [
    "activedb-core",
    "activedb-cli",
    "activedb-container",
    "activedb-macros",
    "aql-tests",
    "metrics",
]
resolver = "2"

[workspace.package]
version = "2.0.0"
edition = "2024"
license = "AGPL-3.0-or-later"
repository = "https://github.com/ActiveDB/activedb-engine"
homepage = "https://activedb.dev"
description = "A graph-vector database engine"

[workspace.dependencies]
# 공통 의존성 버전 통일
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "2"
anyhow = "1"

[profile.release]
opt-level = 2
codegen-units = 1
lto = true
panic = "abort"
strip = "debuginfo"

[profile.dev]
opt-level = 0
codegen-units = 256
lto = false
panic = "abort"
incremental = true
debug = 1
```

### activedb-core/Cargo.toml
```toml
[package]
name = "activedb-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[features]
default = ["compiler"]
compiler = []                     # AQL 컴파일러 (기존 helixc feature)

[dependencies]
activedb-macros = { path = "../activedb-macros" }
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
tracing.workspace = true
thiserror.workspace = true

# 엔진 전용
lmdb-rkv-sys = "0.11"
mimalloc = { version = "0.1", default-features = false }
pest = "2"
pest_derive = "2"
```

### activedb-cli/Cargo.toml
```toml
[package]
name = "activedb-cli"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "activedb"
path = "src/main.rs"

[dependencies]
activedb-core = { path = "../activedb-core", features = ["compiler"] }
activedb-macros = { path = "../activedb-macros" }
serde.workspace = true
tokio.workspace = true
anyhow.workspace = true

# CLI 전용
clap = { version = "4", features = ["derive"] }
dialoguer = "0.11"
indicatif = "0.17"
reqwest = { version = "0.12", features = ["json", "stream"] }
```

---

## 3. crate 역할 및 의존 관계

```
activedb-macros  ←─  activedb-core  ←─  activedb-cli
                          │                    │
                          ▼                    ▼
                    activedb-container   integrations/
                          │               (ecr, fly)
                          ▼
                       metrics
```

| crate | 역할 | 의존성 |
|-------|------|--------|
| `activedb-macros` | 컴파일 타임 코드 생성 | syn, quote, proc-macro2 |
| `activedb-core` | DB 엔진 핵심 로직 | macros, lmdb, pest, mimalloc |
| `activedb-cli` | 사용자 CLI 인터페이스 | core, clap, reqwest, dialoguer |
| `activedb-container` | 배포된 인스턴스 런타임 | core |
| `aql-tests` | 쿼리 언어 통합 테스트 | core |
| `metrics` | 메트릭 수집/전송 | reqwest, serde |

---

## 4. Feature Flags 설계

```toml
[features]
default = ["compiler", "vector", "graph", "bm25"]

# 코어 기능
compiler = []          # AQL 컴파일러
vector = []            # 벡터 유사도 검색
graph = []             # 그래프 알고리즘
bm25 = []              # BM25 전문검색

# 선택적 기능
reranker = []          # 리랭킹 엔진
enterprise = []        # 엔터프라이즈 기능 (HA, 멀티테넌트)
```

---

## 5. 버전 전략

### SemVer 규칙
- `2.0.0` 부터 시작 (리네이밍 = 메이저 버전 범프)
- activedb-core와 activedb-cli는 동일 버전 유지
- activedb-macros, metrics는 독립 버전

### 릴리스 프로세스
1. `dev` 브랜치에서 개발
2. PR → `main` 머지 시 CI 실행
3. GitHub Release 태그 시:
   - crates.io 배포
   - Docker 이미지 빌드/Push (ghcr.io/activedb/activedb-engine)
   - GitHub Release 아티팩트 (Linux/macOS/Windows 바이너리)

---

## 6. Docker 이미지

### 프로덕션 Dockerfile
```dockerfile
FROM rust:1.83-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin activedb-container

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/activedb-container /usr/local/bin/
EXPOSE 6969
CMD ["activedb-container"]
```

### 이미지 태그 규칙
- `activedb/engine:latest` - 최신 릴리스
- `activedb/engine:2.0.0` - 특정 버전
- `activedb/engine:dev` - dev 브랜치 최신
