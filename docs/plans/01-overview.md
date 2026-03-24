# 01. 프로젝트 개요 및 분리 전략

## 1. 현재 상태

### 모노레포 구조 (HelixDB)
```
helix-db/          # 코어 그래프-벡터 DB 엔진 (AGPL-3.0)
helix-cli/         # CLI 도구 (배포, 인증, 통합)
helix-container/   # 컨테이너 런타임
helix-macros/      # 프로시저 매크로
hql-tests/         # HQL 통합 테스트
metrics/           # 메트릭 수집
```

### 핵심 기술 자산
- **스토리지**: LMDB 기반 영속 스토리지
- **쿼리 언어**: HQL (HelixQL) - pest 기반 파서 + 컴파일러
- **검색**: BM25 전문검색 + 벡터 유사도 검색
- **그래프**: 커뮤니티 탐지, 중심성, 유사도 알고리즘
- **배포**: Docker/Podman, ECR, Fly.io, Helix Cloud 지원

---

## 2. 분리 전략

### 2개 독립 저장소로 분리

| 구분 | activedb-engine | activedb-cloud |
|------|----------------|----------------|
| **목적** | 오픈소스 DB 엔진 | 매니지드 클라우드 서비스 |
| **라이선스** | AGPL-3.0 (유지) | Proprietary |
| **저장소** | github.com/ActiveDB/activedb-engine | Private repo |
| **언어** | Rust | Next.js + Rust (프로비저너) |

### 분리 원칙
1. **엔진은 독립 실행 가능**: 클라우드 코드에 대한 의존성 없음
2. **클라우드는 엔진을 소비**: Docker 이미지 또는 crate 참조
3. **CLI는 엔진 저장소에 포함**: 로컬 개발 + 셀프호스팅 지원
4. **클라우드 전용 CLI 확장**: 별도 플러그인 또는 클라우드 저장소 내 래퍼

---

## 3. 브랜딩

### 네이밍 규칙
| 항목 | Helix (현재) | Active (신규) |
|------|-------------|---------------|
| 엔진 | helix-db | activedb-engine |
| CLI | helix-cli | activedb-cli |
| 컨테이너 | helix-container | activedb-container |
| 매크로 | helix-macros | activedb-macros |
| 쿼리 언어 | HQL (HelixQL) | AQL (ActiveQL) |
| 클라우드 | cloud.helix-db.com | cloud.activedb.dev |
| 설정 파일 | helix.toml | activedb.toml |
| 크레덴셜 | ~/.helix/credentials | ~/.activedb/credentials |

### 리네이밍 방침
- **Phase 1**: 코드 내부 리네이밍 (crate명, 모듈명, 변수명)
- **Phase 2**: 사용자 인터페이스 리네이밍 (CLI 명령어, 설정 파일, 문서)
- **Phase 3**: 도메인 및 레지스트리 이전

---

## 4. 라이선스

### activedb-engine
```
AGPL-3.0-or-later
```
- 엔진 코드는 AGPL 유지 → 커뮤니티 기여 보호
- 상업적 사용 시 소스 공개 의무 → 클라우드 서비스 차별화 근거
- CLA (Contributor License Agreement) 도입 검토

### activedb-cloud
```
Proprietary - All rights reserved
```
- 클라우드 서비스 코드는 비공개
- 엔진 AGPL 코드와의 경계 명확화:
  - 엔진은 Docker 이미지로만 소비 (네트워크 경계)
  - 클라우드 코드에서 엔진 코드를 직접 링크하지 않음

### AGPL + Cloud 모델의 법적 고려사항
- Docker 이미지 배포 = AGPL 소스 공개 대상 아님 (네트워크 분리)
- 클라우드 API가 엔진 수정 시 → 수정분 공개 의무 발생
- 엔진 이미지를 그대로 사용하면 공개 의무 없음

---

## 5. 마일스톤

### Phase 1: 엔진 분리 (2주)
- [ ] activedb-engine 저장소 생성
- [ ] 리네이밍 (helix → activedb)
- [ ] CI/CD 파이프라인 이전
- [ ] README, CONTRIBUTING, CODE_OF_CONDUCT 업데이트
- [ ] crates.io 패키지명 확보

### Phase 2: 클라우드 MVP (4주)
- [ ] activedb-cloud 저장소 생성
- [ ] Next.js 프로젝트 초기화
- [ ] GitHub OAuth 인증
- [ ] 인스턴스 생성/삭제 (수동 프로비저닝)
- [ ] 기본 대시보드 UI

### Phase 3: 프로비저닝 자동화 (3주)
- [ ] K8s 기반 자동 프로비저닝
- [ ] 인스턴스 상태 관리
- [ ] 로그 및 메트릭 연동

### Phase 4: 빌링 및 GA (3주)
- [ ] Stripe 결제 연동
- [ ] 사용량 기반 빌링
- [ ] 팀/조직 관리
- [ ] 공개 베타 출시

---

## 6. 리스크 및 완화 전략

| 리스크 | 영향 | 완화 |
|--------|------|------|
| 리네이밍 시 기존 사용자 혼란 | 중 | 마이그레이션 가이드 + deprecated alias 제공 |
| AGPL 라이선스 해석 분쟁 | 고 | 법률 자문, 엔진-클라우드 경계 문서화 |
| K8s 운영 복잡도 | 고 | Phase 2에서 수동 프로비저닝으로 시작 |
| 단독 개발 병목 | 고 | MVP 범위 최소화, 자동화 우선 |
