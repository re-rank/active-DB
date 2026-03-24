# 08. 빌링, 모니터링, 팀 관리, DB 스키마

## 1. Stripe 빌링

### 가격 플랜
| 항목 | Free | Pro ($29/mo) | Enterprise (Custom) |
|------|------|-------------|---------------------|
| 인스턴스 | 1개 | 5개 | 무제한 |
| 스토리지 | 500MB | 10GB/inst | Custom |
| CPU / 메모리 | 0.5 vCPU / 512MB | 2 vCPU / 4GB | Dedicated |
| 백업 | 일 1회 | 시간 1회 | 분 단위 |
| 팀 멤버 | 1 | 5 | 무제한 |
| API 호출 | 10K/day | 1M/day | 무제한 |

### 종량제 (Pro+)
추가 스토리지 $0.10/GB/mo, API 호출 $0.50/100K, 추가 인스턴스 $15/mo, 네트워크 $0.09/GB

### Stripe 리소스 매핑
Customer(1:1 사용자), Product(플랜), Price(월 기본료), Meter(사용량), Subscription(구독), Invoice(청구서)

### 웹훅 처리
```typescript
// app/api/webhook/stripe/route.ts
"checkout.session.completed"     → DB 구독 레코드 생성
"invoice.paid"                   → suspended 인스턴스 재활성화
"invoice.payment_failed"         → 3일 유예 후 suspend
"customer.subscription.deleted"  → 30일 후 인스턴스 삭제 예약
"customer.subscription.updated"  → 리소스 limit 조정
```

### 사용량 보고
Pod metrics → Prometheus(1분) → 집계 크론(1시간) → Stripe Meter → 월말 합산 청구

---

## 2. 모니터링

### 스택
Prometheus(메트릭) + Grafana(운영 대시보드) + CloudWatch(로그/알람) + Next.js(사용자 대시보드)

### 인스턴스 메트릭
```
activedb_queries_total, activedb_queries_duration_ms,
activedb_active_connections, activedb_storage_bytes,
activedb_nodes_total, activedb_edges_total, activedb_vectors_total
```

### 알림 규칙
| 조건 | 심각도 | 액션 |
|------|--------|------|
| CrashLoopBackOff > 3회 | Critical | Slack + Email |
| CPU > 90% 5분 | Warning | 대시보드 표시 |
| 스토리지 > 95% | Critical | 읽기 전용 전환 |
| 노드 NotReady | Critical | 운영팀 호출 |
| 결제 실패 | Info | Email (사용자) |

---

## 3. 팀/조직 관리

### 모델
Organization(Owner 1명 + Members N명 + Instances) ↔ User(Personal Instances + Org Memberships)

### 초대 흐름
Admin 초대 → 이메일 발송 → 링크 클릭 → GitHub OAuth → 멤버 추가 (기본: developer)

### 소유권
인스턴스는 조직 또는 개인 소유. 조직 삭제 시 30일 유예 후 전체 삭제.

---

## 4. PostgreSQL (Neon) 스키마

### 4.1 사용자/인증
```sql
CREATE TABLE users (
  id TEXT PRIMARY KEY DEFAULT gen_id('usr'),
  github_id BIGINT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  name TEXT, avatar_url TEXT,
  stripe_customer_id TEXT UNIQUE,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accounts (  -- NextAuth.js
  id TEXT PRIMARY KEY, user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  type TEXT NOT NULL, provider TEXT NOT NULL, provider_account_id TEXT NOT NULL,
  access_token TEXT, refresh_token TEXT, expires_at BIGINT,
  UNIQUE(provider, provider_account_id)
);

CREATE TABLE sessions (  -- NextAuth.js
  id TEXT PRIMARY KEY, session_token TEXT UNIQUE NOT NULL,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  expires TIMESTAMPTZ NOT NULL
);

CREATE TABLE api_keys (
  id TEXT PRIMARY KEY DEFAULT gen_id('ak'),
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL, key_hash TEXT NOT NULL, key_prefix TEXT NOT NULL,
  last_used TIMESTAMPTZ, expires_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_api_keys_hash ON api_keys(key_hash);
```

### 4.2 조직
```sql
CREATE TABLE organizations (
  id TEXT PRIMARY KEY DEFAULT gen_id('org'),
  name TEXT NOT NULL, slug TEXT UNIQUE NOT NULL,
  owner_id TEXT NOT NULL REFERENCES users(id),
  stripe_customer_id TEXT UNIQUE,
  created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE org_members (
  org_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role TEXT NOT NULL DEFAULT 'developer'
    CHECK (role IN ('viewer','developer','admin','owner')),
  joined_at TIMESTAMPTZ DEFAULT NOW(),
  PRIMARY KEY (org_id, user_id)
);

CREATE TABLE org_invites (
  id TEXT PRIMARY KEY DEFAULT gen_id('inv'),
  org_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  email TEXT NOT NULL, role TEXT NOT NULL DEFAULT 'developer',
  token TEXT UNIQUE NOT NULL, invited_by TEXT NOT NULL REFERENCES users(id),
  expires_at TIMESTAMPTZ DEFAULT NOW() + INTERVAL '7 days',
  accepted_at TIMESTAMPTZ, created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### 4.3 인스턴스
```sql
CREATE TABLE instances (
  id TEXT PRIMARY KEY DEFAULT gen_id('inst'),
  name TEXT NOT NULL,
  owner_type TEXT NOT NULL CHECK (owner_type IN ('user','organization')),
  owner_id TEXT NOT NULL,
  region TEXT NOT NULL DEFAULT 'us-east-1',
  tier TEXT NOT NULL DEFAULT 'free' CHECK (tier IN ('free','pro','enterprise')),
  status TEXT NOT NULL DEFAULT 'provisioning'
    CHECK (status IN ('provisioning','running','stopped','suspended','terminating','terminated','error')),
  endpoint TEXT, image_tag TEXT,
  cpu_limit TEXT DEFAULT '500m', memory_limit TEXT DEFAULT '512Mi',
  storage_size TEXT DEFAULT '500Mi', config JSONB DEFAULT '{}',
  error_message TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);
CREATE INDEX idx_instances_owner ON instances(owner_type, owner_id);
CREATE INDEX idx_instances_status ON instances(status) WHERE deleted_at IS NULL;
```

### 4.4 배포/빌링/백업/감사
```sql
CREATE TABLE deployments (
  id TEXT PRIMARY KEY DEFAULT gen_id('dpl'),
  instance_id TEXT NOT NULL REFERENCES instances(id),
  triggered_by TEXT NOT NULL REFERENCES users(id),
  image_tag TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending','building','deploying','completed','failed')),
  build_log_url TEXT, started_at TIMESTAMPTZ DEFAULT NOW(),
  completed_at TIMESTAMPTZ, error_message TEXT
);

CREATE TABLE subscriptions (
  id TEXT PRIMARY KEY DEFAULT gen_id('sub'),
  owner_type TEXT NOT NULL, owner_id TEXT NOT NULL,
  stripe_subscription_id TEXT UNIQUE NOT NULL, stripe_price_id TEXT NOT NULL,
  plan TEXT NOT NULL CHECK (plan IN ('free','pro','enterprise')),
  status TEXT NOT NULL DEFAULT 'active'
    CHECK (status IN ('active','past_due','canceled','trialing')),
  current_period_start TIMESTAMPTZ, current_period_end TIMESTAMPTZ,
  cancel_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE usage_records (
  id TEXT PRIMARY KEY DEFAULT gen_id('usg'),
  instance_id TEXT NOT NULL REFERENCES instances(id),
  period_start TIMESTAMPTZ NOT NULL, period_end TIMESTAMPTZ NOT NULL,
  api_calls BIGINT DEFAULT 0, storage_bytes BIGINT DEFAULT 0,
  network_bytes BIGINT DEFAULT 0, reported_to_stripe BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE backups (
  id TEXT PRIMARY KEY DEFAULT gen_id('bkp'),
  instance_id TEXT NOT NULL REFERENCES instances(id),
  type TEXT NOT NULL CHECK (type IN ('automatic','manual')),
  status TEXT NOT NULL DEFAULT 'creating'
    CHECK (status IN ('creating','available','restoring','failed','deleted')),
  snapshot_id TEXT, s3_key TEXT, size_bytes BIGINT,
  created_at TIMESTAMPTZ DEFAULT NOW(), expires_at TIMESTAMPTZ
);

CREATE TABLE audit_logs (
  id TEXT PRIMARY KEY DEFAULT gen_id('aud'),
  actor_id TEXT NOT NULL REFERENCES users(id),
  action TEXT NOT NULL, resource_type TEXT NOT NULL, resource_id TEXT NOT NULL,
  metadata JSONB DEFAULT '{}', ip_address INET,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_audit_resource ON audit_logs(resource_type, resource_id, created_at DESC);
```

---

## 5. ID 생성 규칙

```typescript
function generateId(prefix: string): string {
  return `${prefix}_${crypto.randomBytes(12).toString("base64url")}`;
}
// usr_a1b2c3d4..., inst_x9y8z7..., org_m1n2o3..., ak_live_j1k2l3...
```
