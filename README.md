# Plasm

**Compiled tools for AI agents.** Most agent stacks hand the model a wall of JSON schemas, ask it to synthesize a valid payload, then pay again when malformed calls need validation, retries, and repair prompts. Plasm takes a different route: compile real API surfaces into a small, typed instruction layer that both the model and runtime understand.

For example, `dump_prompt apis/github` renders a compact GitHub instruction surface like this:

```tsv
Expression	Meaning
p304	str · owner
p319	str · repo
p297	int · number
p350	str · title
p336	select · allowed: open, closed
e5(p304=$, p319=$, p297=$)	returns e5 · projection [p273,p304,p319,p297,p350,p237,p339,p336,...] · A GitHub issue in a repository
e5{p39=e17(p304=$, p319=$), p42=$, p35=e19("octocat"), p37=$, p38=$, p40=$}	returns [e5] · optional params: p42, p35, p37, p38, p40 · List issues and pull requests in a repository
e17(p304=$, p319=$)	returns e17 · projection [p273,p304,p319,p267,p253,p309,...] · A GitHub repository
e17~$	returns [e17] · Search GitHub repositories.
```

So the model does not need to manufacture a REST payload for “list open bug issues in `plasm/plasm`”; it can emit the constrained Plasm form:

```plasm
e5{p39=e17(p304="plasm", p319="plasm"), p42="open", p37="bug", p40="updated"}
```

On the current GitHub catalog, that full Plasm instruction surface is **20,282 characters** (about **5,070 tokens** by the renderer’s chars/4 heuristic) for 91 capabilities plus navigation forms. By comparison, the official `github-mcp-server` **v0.15.0** exposes 93 tools and its compact serialized `tools/list` schema surface is **64,129 characters** (about **16,032 tokens** by the same heuristic).

That difference gets larger once an agent needs more than one API. Plasm supports **intent-based catalog queries** (`discover_capabilities`) and **federated sessions**: the agent can ask for capabilities by goal, add only the relevant entities from GitHub, Linear, Slack, or another catalog, and keep one typed symbol space across them. In practice this is closer to a small query planner over API domains than a conventional MCP server that exposes a fixed list of independent tools.

That means the agent works with **entities, relations, projections, and capabilities** instead of a flat pile of tool names and argument objects. The runtime can reject impossible calls before they hit a backend, batch graph-shaped work, attach multiple APIs to one session, and keep prompt volume lower as the tool surface grows. The goal is not “more connectors”; it is a tighter tool layer where schema compliance stops being the model’s job.

This repository contains the language/runtime pieces behind that layer:

- **CGS** describes API domains as entities, relations, and capabilities.
- **CML** describes executable transport mappings.
- `**plasm-runtime`** validates and executes against real HTTP/EVM backends.
- `**plasm-mcp` / HTTP** exposes discovery, execute sessions, traces, and MCP to agents.

This workspace ships `plasm-agent` and `plasm-mcp` for local use: HTTP discovery, execute sessions, and unauthenticated Streamable HTTP MCP. The sections below target this workspace only.

## API catalog

The `apis/` directory contains curated CGS/CML packages you can load directly with `--schema apis/<name>` or pack into plugin catalogs with `plasm-pack-plugins`.


| API                                        | What it covers                                                                 |
| ------------------------------------------ | ------------------------------------------------------------------------------ |
| `[clickup](apis/clickup/)`                 | ClickUp workspaces, tasks, lists, and related project-management objects       |
| `[discord](apis/discord/)`                 | Discord guild/channel/message style API surface                                |
| `[dnd5e](apis/dnd5e/)`                     | D&D 5e SRD public API                                                          |
| `[evm-erc20](apis/evm-erc20/)`             | EVM ERC-20 reads                                                               |
| `[figma](apis/figma/)`                     | Figma API surface                                                              |
| `[github](apis/github/)`                   | GitHub repositories, issues, sub-issues, issue types, PRs, reviews, Actions, files, and users |
| `[gitlab](apis/gitlab/)`                   | GitLab projects, issues, and merge requests                                    |
| `[gmail](apis/gmail/)`                     | Gmail mailbox operations                                                       |
| `[google-calendar](apis/google-calendar/)` | Google Calendar events and calendars                                           |
| `[google-docs](apis/google-docs/)`         | Google Docs get/create/batch update operations                                 |
| `[google-drive](apis/google-drive/)`       | Google Drive files, sharing, comments, drives, and changes                     |
| `[google-sheets](apis/google-sheets/)`     | Google Sheets values, batches, and metadata                                    |
| `[graphqlzero](apis/graphqlzero/)`         | GraphQLZero / JSONPlaceholder-style GraphQL                                    |
| `[hackernews](apis/hackernews/)`           | Hacker News Firebase and Algolia search                                        |
| `[jira](apis/jira/)`                       | Jira Cloud REST                                                                |
| `[linkedin](apis/linkedin/)`               | LinkedIn profile and posting/query surfaces                                    |
| `[linear](apis/linear/)`                   | Linear GraphQL issues and comments                                             |
| `[microsoft-teams](apis/microsoft-teams/)` | Microsoft Teams via Microsoft Graph                                            |
| `[musixmatch](apis/musixmatch/)`           | Musixmatch lyrics and related entities                                         |
| `[notion](apis/notion/)`                   | Notion bearer-auth reads/search and database rows                              |
| `[nytimes](apis/nytimes/)`                 | New York Times developer APIs                                                  |
| `[omdb](apis/omdb/)`                       | OMDb movie data                                                                |
| `[openbrewerydb](apis/openbrewerydb/)`     | Open Brewery DB                                                                |
| `[openmeteo](apis/openmeteo/)`             | Open-Meteo weather                                                             |
| `[outlook](apis/outlook/)`                 | Outlook mail folders, messages, and attachments                                |
| `[pokeapi](apis/pokeapi/)`                 | PokéAPI full surface                                                           |
| `[rawg](apis/rawg/)`                       | RAWG games                                                                     |
| `[reddit](apis/reddit/)`                   | Reddit OAuth identity, subreddits, posts, comments, and search                 |
| `[rickandmorty](apis/rickandmorty/)`       | Rick and Morty API                                                             |
| `[slack](apis/slack/)`                     | Slack Web API                                                                  |
| `[spotify](apis/spotify/)`                 | Spotify Web API                                                                |
| `[tau2_retail](apis/tau2_retail/)`         | Tau2 retail test domain                                                        |
| `[tavily](apis/tavily/)`                   | Tavily search, extract, and research                                           |
| `[themealdb](apis/themealdb/)`             | TheMealDB                                                                      |
| `[twitter](apis/twitter/)`                 | X API v2 posts, users, lists, and OAuth scope map                              |
| `[vultr](apis/vultr/)`                     | Vultr public HTTP v2                                                           |
| `[xkcd](apis/xkcd/)`                       | xkcd JSON API                                                                  |


## Prerequisites

- **Rust** (stable), **Cargo**
- A shell with **environment variables** available to the `cargo run` process (and optionally a `**.env`** file in the working tree or a parent—see [dotenv handling](crates/plasm-agent-core/src/dotenv_safe.rs) used by `plasm_agent::init_agent_runtime`).

All commands assume the current directory is the root of this repository.

## Build

**Agent and CLI (`plasm-mcp`, `plasm-cgs`, `plasm-pack-plugins`)** — no codegen step:

```bash
cargo build -p plasm-agent
```

**Full workspace** (includes crates that do not all need to be on the critical path for MCP):

```bash
cargo build --workspace
```

`**plasm-repl**` depends on `[plasm-eval](crates/plasm-eval)`, which includes a **generated** Rust `baml_client` (gitignored). With `[baml-cli](https://github.com/BoundaryML/baml)` on your `PATH` and a version compatible with `[baml_src/](baml_src/)`, run from this directory:

```bash
baml-cli generate
cargo build -p plasm-repl
```

If this repository is checked out as the `plasm-oss/` subdirectory of the full Plasm monorepo, you can run `scripts/ci/ensure-baml-codegen.sh` from the monorepo root; it pins a `baml-cli` version to match CI. Until codegen succeeds, use `**cargo build -p plasm-agent**` only; the quickstarts below list `plasm-repl` where a REPL is optional.

## Configuration (`.env` and environment)

**Default:** put values in a `**.env`** file at the workspace root (or a parent—`[dotenv_safe](crates/plasm-agent-core/src/dotenv_safe.rs)` walks up and merges) or `export` them in your shell. `plasm_agent::init_agent_runtime` loads dotenv on startup (see [lib.rs](crates/plasm-agent/src/lib.rs)).

- **Outbound API calls** (Vultr, Spotify, etc.) use CGS `auth: …` with `**env:`** and OAuth `**client_*_env`** in `[apis/](apis)*`—that reads `**std::env**` (including from `.env`). Avoid `hosted_kv` in local development here; that path targets platform KV in the hosted product.
- `**plasm-mcp` from this workspace** does **not** start `auth-framework`, does not require `PLASM_AUTH_JWT_SECRET`, and runs Streamable HTTP MCP **without** API-key or OAuth **transport** auth. Use `**plasm-mcp-app`** in the monorepo for tenant-scoped MCP, API keys, and control-plane features.
- **Operations / Kubernetes** may still use `PLASM_SECRETS_DIR` and the bootstrap materializer in `[bootstrap_secrets](crates/plasm-agent-core/src/bootstrap_secrets.rs)` for the **product** image—see deploy docs in the private repo—not the default path for day-to-day work in this tree.

## Quickstart: public API (no third-party secrets)

The `[apis/dnd5e](apis/dnd5e)` schema uses `auth: none` and a public `http_backend` in `[domain.yaml](apis/dnd5e/domain.yaml)`. No API keys are required to try read-only calls.

**Interactive REPL:**

```bash
cargo run -p plasm-repl -- --schema apis/dnd5e --backend https://www.dnd5eapi.co
```

**MCP + HTTP in one process:** no JWT or bootstrap flags:

```bash
cargo run -p plasm-agent --bin plasm-mcp -- \
  --schema apis/dnd5e --http --port 3000
```

Add `--mcp` for Streamable HTTP MCP on the same process (see `plasm-mcp --help` for `port` / `mcp-port` when using both).

**Non-interactive check** (only needs `plasm-cgs`, not `plasm-repl` or BAML):

```bash
cargo run -p plasm-agent --bin plasm-cgs -- --schema apis/dnd5e --backend https://www.dnd5eapi.co \
  abilityscore str
```

## Quickstart: API key and OAuth (outbound env vars)

These show **two** common patterns. Export the variables (or add them to `.env`), then run against the matching `apis/…` tree.

**Bearer / API key** — [Vultr](apis/vultr) declares `bearer_token` with `env: VULTR_API_KEY` in `[domain.yaml](apis/vultr/domain.yaml)`.

```bash
export VULTR_API_KEY="your-vultr-key"

cargo run -p plasm-agent --bin plasm-cgs -- --schema apis/vultr --backend https://api.vultr.com \
  region query --limit 5
```

**OAuth 2.0 client credentials** — [Spotify](apis/spotify) uses `oauth2_client_credentials` with env-backed client id/secret in `[domain.yaml](apis/spotify/domain.yaml)` (`SPOTIFY_CLIENT_ID`, `SPOTIFY_CLIENT_SECRET`).

```bash
export SPOTIFY_CLIENT_ID="…"
export SPOTIFY_CLIENT_SECRET="…"

cargo run -p plasm-repl -- --schema apis/spotify --backend https://api.spotify.com
```

`**plasm-mcp` with the same catalog:** the process still needs no `PLASM_AUTH_JWT_SECRET`; set only the outbound env vars (and optional `.env`).

## Add Plasm to Claude, Cursor, or another MCP client

Run `plasm-mcp` as a local Streamable HTTP MCP server, then point your client at its `/mcp` endpoint.

For a public API:

```bash
cargo run -p plasm-agent --bin plasm-mcp -- \
  --schema apis/dnd5e --backend https://www.dnd5eapi.co --mcp --port 3001
```

For an authenticated API, put the outbound credentials in `.env` or export them before starting the server:

```bash
export VULTR_API_KEY="your-vultr-key"

cargo run -p plasm-agent --bin plasm-mcp -- \
  --schema apis/vultr --backend https://api.vultr.com --mcp --port 3001
```

Then configure your MCP client:

```json
{
  "mcpServers": {
    "plasm": {
      "url": "http://127.0.0.1:3001/mcp"
    }
  }
}
```

Use the same URL in Claude Desktop, Cursor, or any client that supports Streamable HTTP MCP. This local server does not require an MCP transport API key; provider credentials are only for outbound calls to the API catalog you loaded.

## License

Plasm is licensed under the [Business Source License 1.1](LICENSE). The Change
License is GPLv3-or-later on the Change Date stated in the license.