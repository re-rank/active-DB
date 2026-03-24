import {
  pgTable,
  text,
  timestamp,
  bigint,
  boolean,
  jsonb,
} from "drizzle-orm/pg-core";
import { generateId } from "@/lib/utils";

// ── Users & Auth ──

export const users = pgTable("users", {
  id: text("id").primaryKey().$defaultFn(() => generateId("usr")),
  githubId: bigint("github_id", { mode: "number" }).unique().notNull(),
  email: text("email").unique().notNull(),
  name: text("name"),
  avatarUrl: text("avatar_url"),
  stripeCustomerId: text("stripe_customer_id").unique(),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
  updatedAt: timestamp("updated_at", { withTimezone: true }).defaultNow(),
});

export const accounts = pgTable("accounts", {
  id: text("id").primaryKey(),
  userId: text("user_id").notNull().references(() => users.id, { onDelete: "cascade" }),
  type: text("type").notNull(),
  provider: text("provider").notNull(),
  providerAccountId: text("provider_account_id").notNull(),
  accessToken: text("access_token"),
  refreshToken: text("refresh_token"),
  expiresAt: bigint("expires_at", { mode: "number" }),
});

export const sessions = pgTable("sessions", {
  id: text("id").primaryKey(),
  sessionToken: text("session_token").unique().notNull(),
  userId: text("user_id").notNull().references(() => users.id, { onDelete: "cascade" }),
  expires: timestamp("expires", { withTimezone: true }).notNull(),
});

export const apiKeys = pgTable("api_keys", {
  id: text("id").primaryKey().$defaultFn(() => generateId("ak")),
  userId: text("user_id").notNull().references(() => users.id, { onDelete: "cascade" }),
  name: text("name").notNull(),
  keyHash: text("key_hash").notNull(),
  keyPrefix: text("key_prefix").notNull(),
  lastUsed: timestamp("last_used", { withTimezone: true }),
  expiresAt: timestamp("expires_at", { withTimezone: true }),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
});

// ── Organizations ──

export const organizations = pgTable("organizations", {
  id: text("id").primaryKey().$defaultFn(() => generateId("org")),
  name: text("name").notNull(),
  slug: text("slug").unique().notNull(),
  ownerId: text("owner_id").notNull().references(() => users.id),
  stripeCustomerId: text("stripe_customer_id").unique(),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
  updatedAt: timestamp("updated_at", { withTimezone: true }).defaultNow(),
});

export const orgMembers = pgTable("org_members", {
  orgId: text("org_id").notNull().references(() => organizations.id, { onDelete: "cascade" }),
  userId: text("user_id").notNull().references(() => users.id, { onDelete: "cascade" }),
  role: text("role").notNull().default("developer"),
  joinedAt: timestamp("joined_at", { withTimezone: true }).defaultNow(),
});

// ── Instances ──

export const instances = pgTable("instances", {
  id: text("id").primaryKey().$defaultFn(() => generateId("inst")),
  name: text("name").notNull(),
  ownerType: text("owner_type").notNull(),
  ownerId: text("owner_id").notNull(),
  region: text("region").notNull().default("us-east-1"),
  tier: text("tier").notNull().default("free"),
  status: text("status").notNull().default("provisioning"),
  endpoint: text("endpoint"),
  imageTag: text("image_tag"),
  cpuLimit: text("cpu_limit").default("500m"),
  memoryLimit: text("memory_limit").default("512Mi"),
  storageSize: text("storage_size").default("500Mi"),
  config: jsonb("config").default({}),
  errorMessage: text("error_message"),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
  updatedAt: timestamp("updated_at", { withTimezone: true }).defaultNow(),
  deletedAt: timestamp("deleted_at", { withTimezone: true }),
});

// ── Deployments ──

export const deployments = pgTable("deployments", {
  id: text("id").primaryKey().$defaultFn(() => generateId("dpl")),
  instanceId: text("instance_id").notNull().references(() => instances.id),
  triggeredBy: text("triggered_by").notNull().references(() => users.id),
  imageTag: text("image_tag").notNull(),
  status: text("status").notNull().default("pending"),
  buildLogUrl: text("build_log_url"),
  startedAt: timestamp("started_at", { withTimezone: true }).defaultNow(),
  completedAt: timestamp("completed_at", { withTimezone: true }),
  errorMessage: text("error_message"),
});

// ── Billing ──

export const subscriptions = pgTable("subscriptions", {
  id: text("id").primaryKey().$defaultFn(() => generateId("sub")),
  ownerType: text("owner_type").notNull(),
  ownerId: text("owner_id").notNull(),
  stripeSubscriptionId: text("stripe_subscription_id").unique().notNull(),
  stripePriceId: text("stripe_price_id").notNull(),
  plan: text("plan").notNull(),
  status: text("status").notNull().default("active"),
  currentPeriodStart: timestamp("current_period_start", { withTimezone: true }),
  currentPeriodEnd: timestamp("current_period_end", { withTimezone: true }),
  cancelAt: timestamp("cancel_at", { withTimezone: true }),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
  updatedAt: timestamp("updated_at", { withTimezone: true }).defaultNow(),
});

// ── Backups ──

export const backups = pgTable("backups", {
  id: text("id").primaryKey().$defaultFn(() => generateId("bkp")),
  instanceId: text("instance_id").notNull().references(() => instances.id),
  type: text("type").notNull(),
  status: text("status").notNull().default("creating"),
  snapshotId: text("snapshot_id"),
  s3Key: text("s3_key"),
  sizeBytes: bigint("size_bytes", { mode: "number" }),
  createdAt: timestamp("created_at", { withTimezone: true }).defaultNow(),
  expiresAt: timestamp("expires_at", { withTimezone: true }),
});
