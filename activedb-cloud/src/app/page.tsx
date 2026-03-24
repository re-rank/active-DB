export default function LandingPage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <h1 className="text-6xl font-bold mb-4">ActiveDB Cloud</h1>
      <p className="text-xl text-muted-foreground mb-8">
        Graph-Vector Database in the Cloud
      </p>
      <a
        href="/login"
        className="rounded-md bg-primary px-6 py-3 text-primary-foreground font-medium hover:bg-primary/90"
      >
        Get Started Free
      </a>
    </main>
  );
}
