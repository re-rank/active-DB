"use client";

import { signIn } from "next-auth/react";
import { Github } from "lucide-react";

export default function LoginPage() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-background">
      <div className="w-full max-w-sm space-y-6 rounded-lg border p-8">
        <div className="text-center space-y-2">
          <h1 className="text-2xl font-bold">ActiveDB Cloud</h1>
          <p className="text-sm text-muted-foreground">
            Sign in to manage your graph-vector databases
          </p>
        </div>
        <button
          onClick={() => signIn("github", { callbackUrl: "/dashboard" })}
          className="flex w-full items-center justify-center gap-2 rounded-md bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        >
          <Github className="h-5 w-5" />
          Continue with GitHub
        </button>
        <p className="text-center text-xs text-muted-foreground">
          By continuing, you agree to our{" "}
          <a href="/terms" className="underline hover:text-foreground">
            Terms of Service
          </a>
        </p>
      </div>
    </main>
  );
}
