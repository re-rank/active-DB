/**
 * K8s Provisioner - Phase 1 (Next.js 내장)
 *
 * Phase 2에서 Rust 기반 SQS Worker로 분리 예정.
 * 현재는 @kubernetes/client-node를 사용한 직접 호출 인터페이스.
 */

const NAMESPACE = "activedb-instances";
const ECR_REGISTRY = process.env.ECR_REGISTRY ?? "";

interface CreateInstanceParams {
  instanceId: string;
  tier: "free" | "pro" | "enterprise";
  region: string;
  imageTag: string;
  apiKey: string;
}

interface TierConfig {
  cpu: { request: string; limit: string };
  memory: { request: string; limit: string };
  storage: string;
  nodeSelector: Record<string, string>;
}

const tierConfigs: Record<string, TierConfig> = {
  free: {
    cpu: { request: "250m", limit: "500m" },
    memory: { request: "256Mi", limit: "512Mi" },
    storage: "500Mi",
    nodeSelector: { role: "db", tier: "free" },
  },
  pro: {
    cpu: { request: "1000m", limit: "2000m" },
    memory: { request: "2Gi", limit: "4Gi" },
    storage: "10Gi",
    nodeSelector: { role: "db", tier: "pro" },
  },
  enterprise: {
    cpu: { request: "4000m", limit: "8000m" },
    memory: { request: "16Gi", limit: "32Gi" },
    storage: "100Gi",
    nodeSelector: { role: "db", tier: "enterprise" },
  },
};

export class Provisioner {
  /**
   * Create all K8s resources for a new instance.
   * Order: Secret → PVC → Deployment → Service → Ingress → NetworkPolicy
   */
  async createInstance(params: CreateInstanceParams): Promise<{ endpoint: string }> {
    const config = tierConfigs[params.tier] ?? tierConfigs.free;
    const name = `inst-${params.instanceId}`;

    // 1. Create Secret
    await this.applyManifest({
      apiVersion: "v1",
      kind: "Secret",
      metadata: { name: `${name}-secret`, namespace: NAMESPACE },
      type: "Opaque",
      stringData: { "api-key": params.apiKey },
    });

    // 2. Create PVC
    await this.applyManifest({
      apiVersion: "v1",
      kind: "PersistentVolumeClaim",
      metadata: { name: `${name}-pvc`, namespace: NAMESPACE },
      spec: {
        accessModes: ["ReadWriteOnce"],
        storageClassName: "gp3",
        resources: { requests: { storage: config.storage } },
      },
    });

    // 3. Create Deployment
    await this.applyManifest({
      apiVersion: "apps/v1",
      kind: "Deployment",
      metadata: { name, namespace: NAMESPACE },
      spec: {
        replicas: 1,
        selector: { matchLabels: { "instance-id": params.instanceId } },
        template: {
          metadata: { labels: { "instance-id": params.instanceId } },
          spec: {
            nodeSelector: config.nodeSelector,
            tolerations: [{ key: "dedicated", value: `db-${params.tier}`, effect: "NoSchedule" }],
            containers: [
              {
                name: "activedb",
                image: `${ECR_REGISTRY}/activedb-${params.instanceId}:${params.imageTag}`,
                ports: [{ containerPort: 6969 }],
                resources: {
                  requests: { cpu: config.cpu.request, memory: config.memory.request },
                  limits: { cpu: config.cpu.limit, memory: config.memory.limit },
                },
                volumeMounts: [{ name: "data", mountPath: "/data" }],
                env: [
                  { name: "ADB_DATA_DIR", value: "/data" },
                  { name: "ADB_API_KEY", valueFrom: { secretKeyRef: { name: `${name}-secret`, key: "api-key" } } },
                ],
                livenessProbe: { httpGet: { path: "/health", port: 6969 }, initialDelaySeconds: 10, periodSeconds: 30 },
                readinessProbe: { httpGet: { path: "/ready", port: 6969 }, initialDelaySeconds: 5, periodSeconds: 10 },
              },
            ],
            volumes: [{ name: "data", persistentVolumeClaim: { claimName: `${name}-pvc` } }],
          },
        },
      },
    });

    // 4. Create Service
    await this.applyManifest({
      apiVersion: "v1",
      kind: "Service",
      metadata: { name: `${name}-svc`, namespace: NAMESPACE },
      spec: {
        selector: { "instance-id": params.instanceId },
        ports: [{ port: 6969, targetPort: 6969 }],
      },
    });

    // 5. Create Ingress
    const hostname = `${params.instanceId}.db.activedb.dev`;
    await this.applyManifest({
      apiVersion: "networking.k8s.io/v1",
      kind: "Ingress",
      metadata: {
        name: `${name}-ingress`,
        namespace: NAMESPACE,
        annotations: {
          "alb.ingress.kubernetes.io/scheme": "internet-facing",
          "alb.ingress.kubernetes.io/target-type": "ip",
          "external-dns.alpha.kubernetes.io/hostname": hostname,
        },
      },
      spec: {
        ingressClassName: "alb",
        rules: [
          {
            host: hostname,
            http: {
              paths: [{ path: "/", pathType: "Prefix", backend: { service: { name: `${name}-svc`, port: { number: 6969 } } } }],
            },
          },
        ],
      },
    });

    // 6. Create NetworkPolicy
    await this.applyManifest({
      apiVersion: "networking.k8s.io/v1",
      kind: "NetworkPolicy",
      metadata: { name: `${name}-netpol`, namespace: NAMESPACE },
      spec: {
        podSelector: { matchLabels: { "instance-id": params.instanceId } },
        policyTypes: ["Ingress", "Egress"],
        ingress: [{ from: [{ namespaceSelector: { matchLabels: { name: "ingress-system" } } }], ports: [{ port: 6969 }] }],
        egress: [{ to: [{ ipBlock: { cidr: "0.0.0.0/0" } }], ports: [{ port: 443 }] }],
      },
    });

    return { endpoint: `https://${hostname}` };
  }

  /**
   * Delete all K8s resources for an instance (reverse order).
   */
  async deleteInstance(instanceId: string): Promise<void> {
    const name = `inst-${instanceId}`;
    const resources = [
      `networkpolicies/${name}-netpol`,
      `ingresses/${name}-ingress`,
      `services/${name}-svc`,
      `deployments/${name}`,
      `persistentvolumeclaims/${name}-pvc`,
      `secrets/${name}-secret`,
    ];

    for (const resource of resources) {
      await this.deleteResource(resource);
    }
  }

  /**
   * Get instance readiness status from K8s.
   */
  async getInstanceStatus(instanceId: string): Promise<"running" | "provisioning" | "error"> {
    // TODO: use @kubernetes/client-node
    // const deploy = await k8sAppsApi.readNamespacedDeployment(`inst-${instanceId}`, NAMESPACE);
    // return (deploy.body.status?.readyReplicas ?? 0) > 0 ? "running" : "provisioning";
    return "provisioning";
  }

  /**
   * Scale deployment replicas (for stop/start).
   */
  async scaleInstance(instanceId: string, replicas: number): Promise<void> {
    // TODO: use @kubernetes/client-node
    // await k8sAppsApi.patchNamespacedDeploymentScale(...)
  }

  // ── Internal ──

  private async applyManifest(manifest: Record<string, unknown>): Promise<void> {
    // TODO: Replace with @kubernetes/client-node calls
    // For now, this is a type-safe interface definition
    console.log(`[Provisioner] Apply ${manifest.kind}: ${(manifest.metadata as Record<string, string>)?.name}`);
  }

  private async deleteResource(resource: string): Promise<void> {
    console.log(`[Provisioner] Delete ${resource} in ${NAMESPACE}`);
  }
}

export const provisioner = new Provisioner();
