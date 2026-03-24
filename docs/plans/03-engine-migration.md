# 03. 엔진 마이그레이션 절차

## 1. 마이그레이션 개요

### 목표
현재 `HelixDB` 모노레포에서 `activedb-engine` 독립 저장소로 이전.
Git 히스토리를 보존하면서 리네이밍을 수행한다.

### 전제 조건
- [ ] `activedb-engine` GitHub 저장소 생성 완료
- [ ] crates.io에 `activedb-core`, `activedb-cli` 이름 확보
- [ ] 도메인 `activedb.dev` 확보
- [ ] Docker Hub / ghcr.io 네임스페이스 확보

---

## 2. Step 1: Git 히스토리 보존 이전

```bash
# 1. 현재 저장소 클론
git clone https://github.com/HelixDB/helix-db.git activedb-engine
cd activedb-engine

# 2. 원격 변경
git remote remove origin
git remote add origin https://github.com/ActiveDB/activedb-engine.git

# 3. 브랜치 정리 (main만 유지)
git checkout main
git branch -D dev  # 필요시

# 4. 첫 푸시
git push -u origin main
```

---

## 3. Step 2: 디렉토리 리네이밍

### 디렉토리 이동
```bash
git mv helix-db activedb-core
git mv helix-cli activedb-cli
git mv helix-container activedb-container
git mv helix-macros activedb-macros
git mv hql-tests aql-tests
# metrics/ 는 그대로 유지
```

### 커밋
```bash
git commit -m "chore: rename directories helix-* → activedb-*"
```

---

## 4. Step 3: Cargo.toml 리네이밍

### 수정 대상 파일 목록

| 파일 | 변경 내용 |
|------|----------|
| `Cargo.toml` (루트) | workspace members 경로 변경 |
| `activedb-core/Cargo.toml` | package name: `activedb-core` |
| `activedb-cli/Cargo.toml` | package name: `activedb-cli`, bin name: `activedb` |
| `activedb-container/Cargo.toml` | package name: `activedb-container` |
| `activedb-macros/Cargo.toml` | package name: `activedb-macros` |
| `aql-tests/Cargo.toml` | package name: `aql-tests` |
| `metrics/Cargo.toml` | 변경 없음 (범용 이름) |

### 내부 의존성 참조 업데이트
```bash
# 모든 Cargo.toml에서 helix- → activedb- 치환
# 모든 Cargo.toml에서 hql- → aql- 치환
```

### 빌드 검증
```bash
cargo check --workspace
cargo test --workspace
```

---

## 5. Step 4: 소스 코드 리네이밍

### 5.1 crate 및 모듈 참조

```bash
# 전체 소스에서 치환 (정규식)
helix_db       → activedb_core
helix_cli      → activedb_cli
helix_container → activedb_container
helix_macros   → activedb_macros
helix_engine   → activedb_engine    # 내부 모듈명
helix_gateway  → activedb_gateway
helixc         → activedb_compiler  # 컴파일러 모듈
hql            → aql                # 쿼리 언어명
HQL            → AQL
HelixQL        → ActiveQL
Helix          → ActiveDB           # 사용자 표시 문자열
helix          → activedb           # 설정 파일, 경로
```

### 5.2 주요 파일별 변경 사항

**activedb-core/src/lib.rs**
```rust
// Before
pub mod helix_engine;
pub mod helix_gateway;
pub mod helixc;

// After
pub mod engine;
pub mod gateway;
pub mod compiler;
```

**activedb-cli/src/commands/auth.rs**
```rust
// Before
const CLOUD_AUTHORITY: &str = "cloud.helix-db.com";
fn credentials_dir() -> PathBuf { home_dir().join(".helix") }

// After
const CLOUD_AUTHORITY: &str = "cloud.activedb.dev";
fn credentials_dir() -> PathBuf { home_dir().join(".activedb") }
```

**activedb-cli/src/config.rs**
```rust
// Before: helix.toml
// After: activedb.toml (하위 호환: helix.toml도 fallback으로 읽기)
```

### 5.3 하위 호환성 Fallback

마이그레이션 기간 동안 기존 사용자를 위한 fallback:
```rust
fn find_config() -> Option<PathBuf> {
    // 1차: activedb.toml
    if Path::new("activedb.toml").exists() {
        return Some("activedb.toml".into());
    }
    // 2차: helix.toml (deprecated 경고 출력)
    if Path::new("helix.toml").exists() {
        eprintln!("Warning: helix.toml is deprecated. Rename to activedb.toml");
        return Some("helix.toml".into());
    }
    None
}
```

크레덴셜도 동일하게 `~/.activedb/` 우선, `~/.helix/` fallback.

---

## 6. Step 5: CI/CD 파이프라인

### GitHub Actions 마이그레이션

#### ci.yml (PR 검증)
```yaml
name: CI
on:
  pull_request:
    branches: [main, dev]
  push:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
```

#### release.yml (릴리스)
```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish -p activedb-macros
      - run: cargo publish -p activedb-core
      - run: cargo publish -p activedb-cli

  docker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      - uses: docker/build-push-action@v5
        with:
          push: true
          tags: |
            ghcr.io/activedb/engine:latest
            ghcr.io/activedb/engine:${{ github.ref_name }}

  binaries:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - run: cargo build --release --bin activedb --target ${{ matrix.target }}
      - uses: actions/upload-artifact@v4
        with:
          name: activedb-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/activedb*
```

---

## 7. Step 6: 문서 업데이트

### README.md 구조
1. 로고 + 배지 (CI, crates.io, license)
2. "ActiveDB is a graph-vector database engine" 한 줄 설명
3. 퀵 스타트 (설치 → 프로젝트 생성 → 쿼리 → 실행)
4. 주요 기능 (그래프, 벡터, 전문검색, AQL)
5. 아키텍처 다이어그램
6. 클라우드 서비스 링크 (cloud.activedb.dev)
7. 기여 가이드 링크

### CHANGELOG.md
```markdown
# Changelog

## [2.0.0] - 2026-XX-XX
### Changed
- Rebranded from HelixDB to ActiveDB
- Renamed query language from HQL to AQL
- Configuration file: helix.toml → activedb.toml
- Credentials directory: ~/.helix/ → ~/.activedb/

### Migration
- helix.toml and ~/.helix/ still work with deprecation warnings
- See MIGRATION.md for detailed upgrade guide
```

---

## 8. 마이그레이션 체크리스트

### 코드 변경
- [ ] 디렉토리 리네이밍 완료
- [ ] Cargo.toml 전체 업데이트
- [ ] 소스 코드 내 문자열 치환 완료
- [ ] `cargo check --workspace` 통과
- [ ] `cargo test --workspace` 통과
- [ ] `cargo clippy --workspace` 경고 없음

### 인프라
- [ ] GitHub 저장소 생성
- [ ] GitHub Actions 설정
- [ ] crates.io 패키지 배포 테스트
- [ ] Docker 이미지 빌드 테스트

### 문서
- [ ] README.md 업데이트
- [ ] CONTRIBUTING.md 작성
- [ ] MIGRATION.md (기존 사용자 가이드)
- [ ] CHANGELOG.md 초기화

### 하위 호환
- [ ] helix.toml → activedb.toml fallback 동작 확인
- [ ] ~/.helix/ → ~/.activedb/ fallback 동작 확인
- [ ] 기존 Docker 이미지 태그 유지 (일정 기간)
