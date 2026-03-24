# 06. 클라우드 프론트엔드 설계

## 1. 기술 스택

| 항목 | 선택 | 비고 |
|------|------|------|
| 프레임워크 | Next.js 14 (App Router) | SSR + ISR |
| UI 컴포넌트 | shadcn/ui | Radix UI 기반, 완전 커스터마이징 |
| 스타일링 | Tailwind CSS 3 | shadcn/ui 기본 |
| 상태 관리 | React Server Components + SWR | 서버 우선 |
| 폼 | react-hook-form + zod | 타입 안전 밸리데이션 |
| 차트 | Recharts | shadcn/ui 통합 차트 |
| 테이블 | TanStack Table | shadcn/ui DataTable |
| 토스트/알림 | sonner | shadcn/ui 기본 |
| 아이콘 | Lucide React | shadcn/ui 기본 |
| 코드 에디터 | Monaco Editor | AQL 쿼리 편집 |

---

## 2. 프로젝트 구조

```
activedb-cloud/
├── package.json
├── next.config.ts
├── tailwind.config.ts
├── drizzle.config.ts
├── .env.local
├── public/
│   ├── logo.svg
│   └── og-image.png
├── src/
│   ├── app/
│   │   ├── layout.tsx              # 루트 레이아웃
│   │   ├── page.tsx                # 랜딩 페이지 (마케팅)
│   │   ├── (auth)/
│   │   │   ├── login/page.tsx      # 로그인
│   │   │   └── signup/page.tsx     # 회원가입 (GitHub OAuth)
│   │   ├── (dashboard)/
│   │   │   ├── layout.tsx          # 대시보드 레이아웃 (사이드바)
│   │   │   ├── page.tsx            # 대시보드 홈 (인스턴스 목록)
│   │   │   ├── instances/
│   │   │   │   ├── page.tsx        # 인스턴스 목록
│   │   │   │   ├── new/page.tsx    # 인스턴스 생성
│   │   │   │   └── [id]/
│   │   │   │       ├── page.tsx    # 인스턴스 상세
│   │   │   │       ├── metrics/page.tsx
│   │   │   │       ├── logs/page.tsx
│   │   │   │       ├── settings/page.tsx
│   │   │   │       └── query/page.tsx  # AQL 쿼리 에디터
│   │   │   ├── settings/
│   │   │   │   ├── page.tsx        # 계정 설정
│   │   │   │   ├── api-keys/page.tsx
│   │   │   │   └── billing/page.tsx
│   │   │   └── org/
│   │   │       ├── page.tsx        # 조직 설정
│   │   │       └── members/page.tsx
│   │   ├── api/                    # (05-cloud-api.md 참조)
│   │   └── invite/
│   │       └── [token]/page.tsx    # 초대 수락 페이지
│   ├── components/
│   │   ├── ui/                     # shadcn/ui 컴포넌트
│   │   ├── layout/
│   │   │   ├── sidebar.tsx
│   │   │   ├── header.tsx
│   │   │   └── breadcrumb.tsx
│   │   ├── instances/
│   │   │   ├── instance-card.tsx
│   │   │   ├── instance-status.tsx
│   │   │   ├── create-form.tsx
│   │   │   └── deploy-progress.tsx
│   │   ├── metrics/
│   │   │   ├── cpu-chart.tsx
│   │   │   ├── memory-chart.tsx
│   │   │   └── queries-chart.tsx
│   │   ├── billing/
│   │   │   ├── plan-selector.tsx
│   │   │   └── usage-meter.tsx
│   │   └── query-editor/
│   │       ├── editor.tsx          # Monaco 래퍼
│   │       └── result-table.tsx
│   ├── lib/
│   │   ├── auth.ts                 # NextAuth 설정
│   │   ├── db.ts                   # Drizzle 클라이언트
│   │   ├── stripe.ts               # Stripe 클라이언트
│   │   └── utils.ts                # cn() 등 유틸
│   ├── db/
│   │   ├── schema.ts               # Drizzle 스키마
│   │   └── migrations/             # Drizzle 마이그레이션
│   └── hooks/
│       ├── use-instance.ts
│       ├── use-metrics.ts
│       └── use-sse.ts              # SSE 커스텀 훅
```

---

## 3. 페이지별 상세

### 3.1 랜딩 페이지 (`/`)
- 히어로: "Graph-Vector Database in the Cloud"
- 기능 소개: 그래프 + 벡터 + 전문검색
- 가격 테이블 (Free / Pro / Enterprise)
- CTA: "Get Started Free" → GitHub OAuth 로그인

### 3.2 로그인 (`/login`)
```
┌─────────────────────────────┐
│       ActiveDB Cloud        │
│                             │
│  ┌───────────────────────┐  │
│  │  Continue with GitHub  │  │
│  └───────────────────────┘  │
│                             │
│  By continuing, you agree   │
│  to our Terms of Service    │
└─────────────────────────────┘
```

### 3.3 대시보드 홈 (`/dashboard`)
```
┌──────┬───────────────────────────────┐
│      │  My Instances                 │
│ Logo │                               │
│      │  ┌─────────┐  ┌─────────┐    │
│ Nav  │  │ prod-db  │  │ dev-db  │    │
│      │  │ Running  │  │ Stopped │    │
│ Inst │  │ us-east  │  │ us-east │    │
│ Sett │  └─────────┘  └─────────┘    │
│ Org  │                               │
│      │  [+ New Instance]             │
│      │                               │
│      │  Usage This Month             │
│      │  ██████░░░░  60% of plan      │
└──────┴───────────────────────────────┘
```

### 3.4 인스턴스 생성 (`/instances/new`)
```
Step 1: 기본 정보
  - 이름: [my-graph-db        ]
  - 리전: [us-east-1 ▼]

Step 2: 플랜 선택
  [Free]  [Pro ✓]  [Enterprise]
  500MB    10GB     Custom
  1 vCPU   2 vCPU   Dedicated

Step 3: 확인
  예상 비용: $29/mo + usage
  [Create Instance]
```

### 3.5 인스턴스 상세 (`/instances/[id]`)
탭 구조:
- **Overview**: 상태, 엔드포인트, 연결 정보, 빠른 액션
- **Metrics**: CPU, 메모리, QPS, 연결 수 차트
- **Logs**: 실시간 로그 뷰어 (필터: level, 시간)
- **Query**: AQL 쿼리 에디터 (Monaco) + 결과 뷰
- **Settings**: 환경 변수, 스케일링, 삭제

### 3.6 AQL 쿼리 에디터 (`/instances/[id]/query`)
```
┌────────────────────────────────────┐
│  AQL Editor                   [▶]  │
│  ┌──────────────────────────────┐  │
│  │ QUERY GetUser(id: String) { │  │
│  │   user = TRAVERSE OUT FROM  │  │
│  │     N<User>::{id == $id}    │  │
│  │   RETURN user               │  │
│  │ }                           │  │
│  └──────────────────────────────┘  │
│                                    │
│  Results                     12ms  │
│  ┌──────────────────────────────┐  │
│  │ { "id": "usr_1", ... }      │  │
│  └──────────────────────────────┘  │
└────────────────────────────────────┘
```

### 3.7 빌링 (`/settings/billing`)
- 현재 플랜 및 사용량
- 플랜 변경 UI
- 결제 수단 관리 (Stripe Elements)
- 청구서 목록 + 다운로드

### 3.8 API Keys (`/settings/api-keys`)
- API Key 목록 (마스킹된 값)
- "Create Key" → 키 이름 입력 → 키 생성 → 한 번만 표시
- 키 삭제 (확인 다이얼로그)

---

## 4. 공통 컴포넌트

### 4.1 사이드바 (sidebar.tsx)
```typescript
const navigation = [
  { name: "Instances", href: "/dashboard", icon: Database },
  { name: "Settings", href: "/settings", icon: Settings },
  { name: "Organization", href: "/org", icon: Users },
  { name: "Docs", href: "https://docs.activedb.dev", icon: Book, external: true },
];
```

### 4.2 인스턴스 상태 뱃지 (instance-status.tsx)
```typescript
const statusConfig = {
  provisioning: { color: "yellow", label: "Provisioning", animate: true },
  running:      { color: "green",  label: "Running" },
  stopped:      { color: "gray",   label: "Stopped" },
  suspended:    { color: "red",    label: "Suspended" },
  error:        { color: "red",    label: "Error" },
};
```

### 4.3 SSE 훅 (use-sse.ts)
```typescript
function useSSE<T>(url: string | null) {
  const [data, setData] = useState<T | null>(null);
  const [status, setStatus] = useState<"connecting" | "open" | "closed">("connecting");

  useEffect(() => {
    if (!url) return;
    const es = new EventSource(url);
    es.onmessage = (e) => setData(JSON.parse(e.data));
    es.onopen = () => setStatus("open");
    es.onerror = () => setStatus("closed");
    return () => es.close();
  }, [url]);

  return { data, status };
}
```

---

## 5. 다크 모드

- shadcn/ui `next-themes` 통합
- 시스템 설정 따름 (기본)
- 수동 토글: 헤더에 테마 스위처

---

## 6. 반응형

| 브레이크포인트 | 레이아웃 |
|--------------|---------|
| `< 768px` | 사이드바 숨김, 햄버거 메뉴 |
| `768px - 1024px` | 축소 사이드바 (아이콘만) |
| `> 1024px` | 풀 사이드바 |

---

## 7. 성능 최적화

- **RSC 우선**: 서버 컴포넌트로 초기 로드 최소화
- **Streaming**: `loading.tsx`로 점진적 렌더링
- **SWR**: 클라이언트 데이터 페칭 + 캐싱
- **Dynamic Import**: Monaco Editor 등 무거운 컴포넌트 지연 로드
- **Image**: `next/image`로 최적화

---

## 8. 환경 변수

```env
# .env.local
NEXTAUTH_URL=http://localhost:3000
NEXTAUTH_SECRET=...
GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...
NEON_DATABASE_URL=postgresql://...
STRIPE_SECRET_KEY=sk_test_...
STRIPE_PUBLISHABLE_KEY=pk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
NEXT_PUBLIC_APP_URL=http://localhost:3000
```
