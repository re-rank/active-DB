# 07. K8s 기반 인스턴스 프로비저닝

## 1. 프로비저닝 개요

1개 DB 인스턴스 = Deployment + Service + Ingress + PVC + Secret + NetworkPolicy

---

## 2. EKS 클러스터 구성

### 노드 그룹
| 그룹 | 인스턴스 타입 | 용도 | 스케일 |
|------|-------------|------|--------|
| system | t3.medium | 시스템 컴포넌트 | 2 고정 |
| db-free | t3.small | Free 티어 DB | 2~10 |
| db-pro | m6i.large | Pro 티어 DB | 1~20 |

각 db 노드 그룹에 taint 적용 → 해당 티어 Pod만 스케줄링.

### 필수 애드온
- AWS Load Balancer Controller (ALB Ingress)
- EBS CSI Driver (PVC 동적 프로비저닝)
- External DNS (Route53 자동 레코드)
- Cert Manager (Let's Encrypt TLS)
- Metrics Server (HPA)

---

## 3. 인스턴스 K8s 매니페스트

### Deployment (핵심 부분)
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: inst-${INSTANCE_ID}
  namespace: activedb-instances
spec:
  replicas: 1
  selector:
    matchLabels: { instance-id: ${INSTANCE_ID} }
  template:
    spec:
      nodeSelector: { role: db, tier: ${TIER} }
      tolerations:
        - { key: dedicated, value: "db-${TIER}", effect: NoSchedule }
      containers:
        - name: activedb
          image: ${ECR_REGISTRY}/activedb-${INSTANCE_ID}:${IMAGE_TAG}
          ports: [{ containerPort: 6969 }]
          resources:
            requests: { cpu: ${CPU_REQ}, memory: ${MEM_REQ} }
            limits:   { cpu: ${CPU_LIM}, memory: ${MEM_LIM} }
          volumeMounts: [{ name: data, mountPath: /data }]
          env:
            - { name: ADB_DATA_DIR, value: /data }
            - name: ADB_API_KEY
              valueFrom:
                secretKeyRef: { name: "inst-${INSTANCE_ID}-secret", key: api-key }
          livenessProbe:
            httpGet: { path: /health, port: 6969 }
            initialDelaySeconds: 10
            periodSeconds: 30
          readinessProbe:
            httpGet: { path: /ready, port: 6969 }
            initialDelaySeconds: 5
            periodSeconds: 10
      volumes:
        - name: data
          persistentVolumeClaim: { claimName: "inst-${INSTANCE_ID}-pvc" }
```

### PVC
```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: inst-${INSTANCE_ID}-pvc
spec:
  accessModes: [ReadWriteOnce]
  storageClassName: gp3
  resources:
    requests:
      storage: ${STORAGE_SIZE}  # Free: 500Mi, Pro: 10Gi, Ent: 100Gi
```

### Service + Ingress
```yaml
apiVersion: v1
kind: Service
metadata: { name: "inst-${INSTANCE_ID}-svc" }
spec:
  selector: { instance-id: ${INSTANCE_ID} }
  ports: [{ port: 6969, targetPort: 6969 }]
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: inst-${INSTANCE_ID}-ingress
  annotations:
    alb.ingress.kubernetes.io/scheme: internet-facing
    alb.ingress.kubernetes.io/target-type: ip
    external-dns.alpha.kubernetes.io/hostname: ${INSTANCE_ID}.db.activedb.dev
spec:
  ingressClassName: alb
  rules:
    - host: ${INSTANCE_ID}.db.activedb.dev
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service: { name: "inst-${INSTANCE_ID}-svc", port: { number: 6969 } }
```

### NetworkPolicy (인스턴스 격리)
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
spec:
  podSelector: { matchLabels: { instance-id: ${INSTANCE_ID} } }
  policyTypes: [Ingress, Egress]
  ingress:
    - from: [{ namespaceSelector: { matchLabels: { name: ingress-system } } }]
      ports: [{ port: 6969 }]
  egress:
    - to: [{ ipBlock: { cidr: "0.0.0.0/0" } }]
      ports: [{ port: 443 }]
```

---

## 4. 프로비저닝 엔진 (Phase 1: Next.js 내장)

```typescript
// src/lib/provisioner.ts
export class Provisioner {
  async createInstance(params: CreateInstanceParams): Promise<void> {
    // 순서: Secret → PVC → Deployment → Service → Ingress → NetworkPolicy
  }
  async deleteInstance(id: string): Promise<void> {
    // 역순 삭제
  }
  async getInstanceStatus(id: string): Promise<"running" | "provisioning"> {
    const deploy = await this.k8s.apps.readNamespacedDeployment(name, NS);
    return (deploy.body.status?.readyReplicas ?? 0) > 0 ? "running" : "provisioning";
  }
}
```

---

## 5. 프로비저닝 엔진 (Phase 2: Rust 에이전트)

```
SQS Queue ← Next.js API
    ↓
Rust Provisioner (ECS Fargate)
  ├── kube-rs (K8s API)
  ├── aws-sdk-sqs (큐 소비)
  ├── aws-sdk-ecr (이미지)
  └── sqlx (Neon DB 상태 업데이트)
```

큐 메시지: `{ action: "create|delete|restart|scale", instance_id, params: { tier, region, image_tag, ... } }`

---

## 6. 이미지 빌드 파이프라인

```
CLI push → API (S3 업로드) → CodeBuild:
  1. S3에서 쿼리 다운로드
  2. AQL 컴파일 (activedb-core)
  3. Docker 이미지 빌드 → ECR push
  4. K8s Rolling Update (maxSurge:1, maxUnavailable:0)
```

ECR 네이밍: `activedb-${INSTANCE_ID}:${VERSION}` (기존 CLI 패턴 재사용)

---

## 7. 배포 전략

### Rolling Update
- maxSurge: 1, maxUnavailable: 0
- readinessProbe 통과 → 이전 Pod 종료
- 실패 시 자동 롤백

### Auto-Scaling (Phase 3)
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
spec:
  scaleTargetRef: { kind: Deployment, name: "inst-${INSTANCE_ID}" }
  minReplicas: 1
  maxReplicas: ${MAX}  # Pro: 3, Enterprise: 10
  metrics:
    - type: Resource
      resource: { name: cpu, target: { type: Utilization, averageUtilization: 70 } }
```

---

## 8. 장애 복구

- **Pod 자동 복구**: restartPolicy: Always + livenessProbe
- **데이터 복구**: PVC → EBS Snapshot (매 시간) → 복원 시 스냅샷에서 새 PVC 생성
- **알림**: CloudWatch Alarm → SNS → Slack/Email
  - CrashLoopBackOff, PVC 90%+, 노드 NotReady
