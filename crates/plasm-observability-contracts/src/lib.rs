//! Serde wire contracts for Plasm audit/trace observability, independent of sink or storage backends.
//!
//! Product-only row types (e.g. Iceberg projection rows) live in `plasm-trace-sink`, not here.

mod model;
mod run_artifact;

pub use model::{
    AuditEvent, DurableTraceDetail, IngestBatchRequest, IngestBatchResponse, TraceDetailRecord,
    TraceDetailResponse, TraceGetResponse, TraceListResponse, TraceSummary, TraceTotals,
    AUDIT_EVENT_KIND_MCP_TRACE_SEGMENT, SCHEMA_VERSION,
};
pub use run_artifact::RunArtifactArchiveRef;
