<div align="center">

<picture>
  <img src="/assets/full_logo.png" alt="ActiveDB Logo">
</picture>

<b>ActiveDB</b>: an open-source graph-vector database engine built from scratch in Rust.

<h3>
  <a href="https://activedb.dev">Website</a> |
  <a href="https://docs.activedb.dev">Docs</a> |
  <a href="https://cloud.activedb.dev">Cloud</a>
</h3>

[![CI](https://img.shields.io/github/actions/workflow/status/ActiveDB/activedb-engine/ci.yml?label=CI)](https://github.com/ActiveDB/activedb-engine/actions)
[![License](https://img.shields.io/badge/license-AGPL--3.0-blue)](LICENSE)

</div>

<hr>

ActiveDB is a database that makes it easy to build all the components needed for an AI application in a single platform.

You no longer need a separate application DB, vector DB, graph DB, or application layers to manage the multiple storage locations. ActiveDB primarily operates with a graph + vector data model, but it can also support KV, documents, and relational data.

## Key Features

| | |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| **Built-in MCP tools** | ActiveDB has built-in MCP support to allow your agents to discover data and walk the graph. |
| **Built-in Embeddings** | No need to embed your data before sending it to ActiveDB, just use the `Embed` function to vectorize text. |
| **Tooling for RAG** | Built-in vector search, keyword search, and graph traversals for any type of RAG applications. |
| **Secure by Default** | Private by default. You can only access your data through your compiled ActiveQL queries. |
| **Ultra-Low Latency** | Built in Rust with LMDB as its storage engine for extremely low latencies. |
| **Type-Safe Queries** | ActiveQL is 100% type-safe, so your queries are guaranteed to execute correctly in production. |
| **Graph Algorithms** | 17 built-in graph algorithms: PageRank, Louvain, Betweenness Centrality, and more. |

## Getting Started

### Install CLI

```bash
curl -sSL "https://install.activedb.dev" | bash
```

### Initialize a project

```bash
mkdir my-project && cd my-project
activedb init
```

### Write queries

Open your newly created `.hx` files and start writing your schema and queries.
Head over to [our docs](https://docs.activedb.dev) for more information about writing queries.

```js
N::User {
   INDEX name: String,
   age: U32
}

QUERY getUser(user_name: String) =>
   user <- N<User>({name: user_name})
   RETURN user
```

### Check and deploy

```bash
activedb check
activedb push dev
```

### Query via SDK

```typescript
import ActiveDB from "activedb-ts";

const client = new ActiveDB();

await client.query("addUser", { name: "John", age: 20 });
const user = await client.query("getUser", { user_name: "John" });
console.log(user);
```

## Architecture

```
activedb-engine/
├── activedb-core/       # Core database engine (storage, graph, vector, BM25, compiler)
├── activedb-cli/        # CLI tool (init, build, push, auth, integrations)
├── activedb-container/  # Container runtime for deployed instances
├── activedb-macros/     # Procedural macros
├── aql-tests/           # ActiveQL integration tests
└── metrics/             # Metrics collection
```

## Cloud Service

ActiveDB is available as a managed cloud service at [cloud.activedb.dev](https://cloud.activedb.dev).

## License

ActiveDB is licensed under the [AGPL-3.0](LICENSE) (Affero General Public License).

## Commercial Support

For managed service or enterprise support, contact us at [cloud.activedb.dev](https://cloud.activedb.dev).
