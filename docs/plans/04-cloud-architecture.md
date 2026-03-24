# 04. 클라우드 서비스 시스템 아키텍처

## 1. 시스템 개요

```
┌─────────────────────────────────────────────────────────────┐
│                    activedb-cloud                            │
│                                                             │
│  ┌──────────┐    ┌──────────────┐    ┌─────────────────┐   │
│  │ Next.js  │───▶│ Next.js API  │───▶│   PostgreSQL    │   │
│  │ Frontend │    │   Routes     │    │   (Neon)        │   │
│  │          │    │              │───▶│   메타데이터 DB   │   │
│  └──────────┘    └──────┬───────┘    └─────────────────┘   │
│                         │                                   │
│                         ▼                                   │
│                  ┌──────────────┐                           │
│                  │ Provisioner  │                           │
│                  │ (Rust Agent) │                           │
│                  └──────┬───────┘                           │
│                         │                                   │
│              ┌──────────┼──────────┐                       │
│              ▼          ▼          ▼                        │
│         ┌────────┐ ┌────────┐ ┌────────┐                  │
│         │ DB Pod │ │ DB Pod │ │ DB Pod │  ← EKS Cluster   │
│         │ inst-1 │ │ inst-2 │ │ inst-3 │                  │
│         └────────┘ └────────┘ └────────┘                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │    External APIs     │
              │  Stripe │ GitHub    │
              │  OAuth  │ Billing   │
              └─────────────────────┘
```

---

## 2. 기술 스택

| 레이어 | 기술 | 선택 이유 |
|--------|------|----------|
| **프론트엔드** | Next.js 14 (App Router) | SSR + API Routes 통합 |
| **UI** | shadcn/ui + Tailwind CSS | 빠른 프로토타이핑, 커스터마이징 용이 |
| **백엔드** | Next.js API Routes | Phase 1 단순성 |
| **프로비저너** | Rust (Phase 2) | K8s API 직접 호출, 성능 |
| **메타데이터 DB** | PostgreSQL (Neon) | 서버리스, 브랜칭 지원 |
| **ORM** | Drizzle ORM | 타입 안전, 경량 |
| **인증** | NextAuth.js + GitHub OAuth | 기존 CLI auth 패턴 호환 |
| **빌링** | Stripe | 업계 표준, usage-based 지원 |
| **인프라** | AWS EKS + ECR | 기존 CLI ECR 통합 재사용 |
| **모니터링** | Prometheus + Grafana | K8s 네이티브 |
| **로깅** | CloudWatch Logs | AWS 통합 |

---

## 3. 서비스 컴포넌트

### 3.1 Next.js 앱 (Vercel 배포)
- 프론트엔드 + API Routes 통합 배포
- Vercel Edge Functions로 글로벌 저지연
- 환경: `NEON_DATABASE_URL`, `STRIPE_SECRET_KEY`, `GITHUB_CLIENT_ID`

### 3.2 프로비저너 (Phase 2, AWS Lambda 또는 ECS)
```
역할: DB 인스턴스 생명주기 관리
- 인스턴스 생성 요청 → K8s Pod 스케줄링
- 인스턴스 삭제 요청 → Pod 정리 + 볼륨 해제
- 헬스 체크 → 상태 업데이트
- 스케일링 → HPA 조정
```

### 3.3 DB 인스턴스 (EKS Pod)
- 각 인스턴스 = 1개 Pod (activedb-container 이미지)
- PVC로 영속 스토리지 연결
- Service + Ingress로 외부 노출

---

## 4. Phase별 아키텍처

### Phase 1: MVP (수동 프로비저닝)
```
Vercel (Next.js)
    ├── Frontend (React)
    ├── API Routes
    │   ├── /api/auth/*        → NextAuth.js
    │   ├── /api/instances/*   → Neon DB CRUD
    │   └── /api/billing/*     → Stripe
    └── Server Actions
        └── provisionInstance() → kubectl exec (수동)
```

- 인스턴스 생성은 API에서 직접 `kubectl` / AWS SDK 호출
- 소수 사용자 대상, 빠른 검증 목적

### Phase 2: 자동 프로비저닝
```
Vercel (Next.js)
    ├── Frontend + API (동일)
    └── /api/instances/create
            │
            ▼ (SQS Queue)
Rust Provisioner (ECS Task)
    ├── K8s API Client (kube-rs)
    ├── ECR Image Pull
    ├── Pod/Service/Ingress 생성
    └── 상태 콜백 → Neon DB 업데이트
```

- Rust 프로비저너가 큐에서 요청을 소비
- 비동기 프로비저닝 → 웹훅/SSE로 상태 알림

### Phase 3: 멀티 리전 + HA
```
글로벌 로드밸런서 (CloudFront)
    ├── us-east-1 (EKS Cluster)
    ├── eu-west-1 (EKS Cluster)
    └── ap-northeast-2 (EKS Cluster)
```

---

## 5. 데이터 흐름

### 인스턴스 생성 흐름
```
1. 사용자 → "Create Instance" 클릭
2. Frontend → POST /api/instances
3. API Route:
   a. 사용자 인증 확인 (NextAuth session)
   b. 플랜 제한 확인 (인스턴스 수, 리소스)
   c. Stripe 결제 수단 확인
   d. Neon DB에 인스턴스 레코드 생성 (status: provisioning)
   e. 프로비저닝 요청 발행
4. Provisioner:
   a. ECR에서 이미지 pull
   b. K8s: Namespace, PVC, Deployment, Service, Ingress 생성
   c. 헬스 체크 통과 대기
   d. Neon DB 상태 업데이트 (status: running)
   e. 연결 문자열 생성
5. Frontend ← SSE/폴링으로 상태 확인
6. 사용자에게 연결 정보 표시
```

### 쿼리 실행 흐름
```
1. 사용자 앱 → HTTPS → ALB
2. ALB → K8s Ingress → Service
3. Service → Pod (activedb-container)
4. Pod: AQL 쿼리 실행 → 응답 반환
```

---

## 6. 네트워크 아키텍처

### VPC 설계
```
VPC: 10.0.0.0/16
├── Public Subnets (10.0.1.0/24, 10.0.2.0/24)
│   ├── ALB (Application Load Balancer)
│   └── NAT Gateway
├── Private Subnets (10.0.10.0/24, 10.0.20.0/24)
│   ├── EKS Worker Nodes
│   └── DB Instance Pods
└── Isolated Subnets (10.0.100.0/24)
    └── (향후 RDS 등)
```

### 보안 그룹
| SG | 인바운드 | 아웃바운드 |
|----|---------|-----------|
| ALB | 443 (0.0.0.0/0) | EKS 노드 |
| EKS 노드 | ALB SG:any | 0.0.0.0/0 |
| DB Pod | EKS 노드 SG:6969 | 없음 |

---

## 7. 인스턴스 격리

### 테넌트 격리 전략
- **Phase 1**: Namespace 기반 격리
  - 각 사용자 = 1개 K8s Namespace
  - NetworkPolicy로 Namespace 간 통신 차단
- **Phase 2**: Node Pool 기반 격리 (유료 플랜)
  - 전용 노드 풀에 Pod 스케줄링
  - 리소스 보장 (QoS: Guaranteed)

### 리소스 제한
```yaml
# Free Tier
resources:
  requests: { cpu: "100m", memory: "256Mi" }
  limits:   { cpu: "500m", memory: "512Mi" }

# Pro Tier
resources:
  requests: { cpu: "500m", memory: "1Gi" }
  limits:   { cpu: "2000m", memory: "4Gi" }

# Enterprise
resources:
  requests: { cpu: "2000m", memory: "8Gi" }
  limits:   { cpu: "8000m", memory: "32Gi" }
```

---

## 8. 가용성 및 복구

| 항목 | 목표 |
|------|------|
| 웹 앱 SLA | 99.9% (Vercel) |
| DB 인스턴스 SLA | 99.5% (Phase 1), 99.9% (Phase 3) |
| RPO | 1시간 (스냅샷 주기) |
| RTO | 15분 (Pod 재시작) |

### 백업 전략
1. PVC 스냅샷: 매 시간 (EBS Snapshots)
2. 일일 전체 백업: S3 업로드
3. 사용자 요청 백업: API 제공
