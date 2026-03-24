# 05. 클라우드 백엔드 API 설계

## 1. API 개요

- **Base URL**: `https://cloud.activedb.dev/api`
- **프로토콜**: REST (JSON)
- **인증**: Session Cookie (웹) + API Key `x-api-key: ak_...` (CLI/SDK)
- **버전**: `/api/v1/...`

---

## 2. 인증 API

### GitHub OAuth (NextAuth.js)
```
POST /api/auth/signin/github     → GitHub OAuth 리다이렉트
GET  /api/auth/callback/github   → 콜백, 세션 생성
GET  /api/auth/session           → 현재 세션
POST /api/auth/signout           → 로그아웃
```

### CLI 디바이스 흐름 (기존 auth.rs 패턴)
```
POST /api/auth/device/code   → { device_code, user_code, verification_uri }
POST /api/auth/device/token  → { user_id, api_key }
```

### SSE 기반 CLI 로그인
```
GET /api/auth/cli/login (SSE)
→ { type: "device_code", code: "ABCD-1234", url: "..." }
→ { type: "authenticated", user_id: "usr_...", api_key: "ak_..." }
```

### API Key 관리
```
POST   /api/v1/api-keys              # 생성
GET    /api/v1/api-keys              # 목록
DELETE /api/v1/api-keys/:keyId       # 삭제
POST   /api/v1/api-keys/:keyId/rotate  # 갱신
```

Key 형식: `ak_live_` + 32자 (프로덕션), `ak_test_` + 32자 (테스트)

---

## 3. 인스턴스 관리 API

### CRUD
```
POST   /api/v1/instances              # 생성
GET    /api/v1/instances              # 목록
GET    /api/v1/instances/:id          # 상세
PATCH  /api/v1/instances/:id          # 수정
DELETE /api/v1/instances/:id          # 삭제
```

### 생성 요청/응답
```json
// POST /api/v1/instances
{ "name": "my-graph-db", "region": "us-east-1", "plan": "pro",
  "config": { "build_mode": "release", "env_vars": { "ADB_MAX_CONNECTIONS": "100" } } }

// Response
{ "id": "inst_abc123", "name": "my-graph-db", "status": "provisioning",
  "region": "us-east-1", "plan": "pro", "endpoint": null }
```

### 상태 전이
```
provisioning → running → stopped
                  ↓
              suspended (빌링 실패) → terminated (삭제)
```

### 액션
```
POST /api/v1/instances/:id/start|stop|restart|backup|restore|deploy
```

---

## 4. 쿼리 배포 API

```
POST /api/v1/instances/:id/deploy  (multipart: queries.tar.gz + build_mode)
```

배포 흐름: CLI upload → S3 저장 → 컴파일 + 이미지 빌드 → ECR push → K8s Rolling Update

### 배포 상태 (SSE)
```
GET /api/v1/instances/:id/deploy/status
→ { "stage": "uploading|compiling|building_image|deploying|completed", "progress": N }
```

---

## 5. 모니터링 API

```
GET /api/v1/instances/:id/metrics?period=1h&metrics=cpu,memory,queries
GET /api/v1/instances/:id/logs?since=...&limit=100&level=error
GET /api/v1/instances/:id/logs/stream  (SSE 실시간)
```

메트릭 응답: `{ period, data_points: [{ timestamp, cpu_percent, memory_mb, queries_per_sec }] }`

---

## 6. 빌링 API

```
GET    /api/v1/billing/subscription     # 구독 정보
POST   /api/v1/billing/subscribe        # 구독/변경
POST   /api/v1/billing/cancel           # 취소
GET    /api/v1/billing/invoices         # 청구서
GET    /api/v1/billing/usage            # 사용량
POST   /api/v1/billing/payment-method   # 결제 수단
GET    /api/v1/billing/portal           # Stripe Customer Portal
```

---

## 7. 팀/조직 API

```
POST   /api/v1/organizations                         # 생성
GET    /api/v1/organizations                         # 목록
GET    /api/v1/organizations/:orgId                  # 상세
PATCH  /api/v1/organizations/:orgId                  # 수정
POST   /api/v1/organizations/:orgId/members          # 멤버 추가
GET    /api/v1/organizations/:orgId/members          # 멤버 목록
PATCH  /api/v1/organizations/:orgId/members/:userId  # 역할 변경
DELETE /api/v1/organizations/:orgId/members/:userId  # 제거
POST   /api/v1/organizations/:orgId/invites          # 초대
POST   /api/v1/invites/:token/accept                 # 수락
```

### RBAC
| 역할 | 인스턴스 조회 | 관리 | 멤버 관리 | 빌링 | 삭제 |
|------|:---:|:---:|:---:|:---:|:---:|
| viewer | O | X | X | X | X |
| developer | O | O | X | X | X |
| admin | O | O | O | O | X |
| owner | O | O | O | O | O |

---

## 8. 에러 및 Rate Limiting

### 에러 형식
```json
{ "error": { "code": "INSTANCE_NOT_FOUND", "message": "...", "status": 404 } }
```

### 에러 코드
| HTTP | 코드 | 설명 |
|------|------|------|
| 400 | INVALID_REQUEST | 잘못된 파라미터 |
| 401 | UNAUTHORIZED | 인증 필요 |
| 403 | FORBIDDEN | 권한 부족 |
| 404 | NOT_FOUND | 리소스 없음 |
| 409 | CONFLICT | 상태 충돌 |
| 422 | PLAN_LIMIT_EXCEEDED | 한도 초과 |
| 429 | RATE_LIMITED | 빈도 초과 |

Rate: 인증 1000 req/min, 비인증 60 req/min
헤더: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`

---

## 9. API Route 파일 구조

```
app/api/
├── auth/
│   ├── [...nextauth]/route.ts
│   └── cli/login/route.ts
├── v1/
│   ├── instances/
│   │   ├── route.ts                    # GET, POST
│   │   └── [id]/
│   │       ├── route.ts                # GET, PATCH, DELETE
│   │       ├── start|stop/route.ts
│   │       ├── deploy/{route.ts, status/route.ts}
│   │       ├── metrics/route.ts
│   │       └── logs/{route.ts, stream/route.ts}
│   ├── api-keys/
│   │   ├── route.ts
│   │   └── [keyId]/{route.ts, rotate/route.ts}
│   ├── billing/{subscription,usage,invoices,portal}/route.ts
│   ├── organizations/
│   │   ├── route.ts
│   │   └── [orgId]/{route.ts, members/{route.ts,[userId]/route.ts}}
│   └── webhook/stripe/route.ts
```
