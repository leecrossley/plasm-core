# plasm-core

CGS schema loading, shared types, expression AST, type checking, discovery, and prompt rendering—**catalog-agnostic** building blocks for Plasm.

## Design boundary: no domain leakage

Plasm is a **general-purpose language and runtime for API mapping** (schema, expressions, CML, execution). **Domain-specific knowledge is forbidden in this crate:** no branches on particular CGS entity or capability names from `apis/…`, no field-alias or env-key hacks for one vendor’s HTTP templates, and no special transport cases tied to a single product.

Catalog behavior belongs in **`apis/<name>/`**, fixtures, and optional **plugins**—expressed as data and schema-driven rules. Code here stays **agnostic**, driven only by loaded CGS and generic IR/types.

## Prompt surface

The **TSV** default (`Expression` / `Meaning` columns) and the **compact markdown DOMAIN** layout share the same slot metadata. On method and query lines, the renderer can add a compact

`args: p# <wire> <type> <req|opt>; …`

fragment to the `Meaning` cell (TSV) or the `;;` tail (markdown DOMAIN), using types from CGS and [`IdentMetadata`](src/symbol_tuning.rs). When that inline text is not enough, standalone `p#` gloss lines still appear (e.g. long `select+` / `multiselect+` / `array+`, projection lists, relations, block headings).

**Measuring size:** `cargo run -p plasm-core --bin dump_prompt -- <path/to/schema-dir> >/dev/null` prints `chars | ~tok (heuristic) | …` on stderr. A measured table of example catalogs is in the root [Plasm README](../README.md).

See [AGENTS.md](../../AGENTS.md) for workspace layout and commands.
