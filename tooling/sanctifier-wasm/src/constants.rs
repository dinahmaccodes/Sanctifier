//! Compile-time constants for the WASM package.
//!
//! All tuneable limits and namespace strings live here so they can be
//! referenced by both the implementation modules and the test suite without
//! coupling either to internal details of the other.

/// Analysis output schema version (independent of tool version).
///
/// Increment only when the JSON output format changes in a breaking way.
/// See `docs/wasm-versioning-alignment.md` for the full versioning policy.
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Maximum allowed source code size (10 MB).
pub const MAX_SOURCE_SIZE: usize = 10 * 1024 * 1024;

/// Minimum required source code size (1 byte).
pub const MIN_SOURCE_SIZE: usize = 1;

/// Conservative per-invocation memory budget (32 MB).
///
/// WASM32 has a 4 GB virtual address space but the default linear memory
/// grows in 64 KB pages.  Capping working-set estimation to 32 MB prevents
/// runaway allocations from stalling the browser tab.
pub const MEMORY_BUDGET_BYTES: usize = 32 * 1024 * 1024;

/// Conservative overhead multiplier for memory budget estimation.
///
/// The analyser expands source into several internal representations (tokens,
/// AST nodes, finding lists).  A factor of 8 covers the worst-case observed
/// peak without live heap profiling.
pub const MEMORY_OVERHEAD_FACTOR: usize = 8;

/// Maximum allowed configuration JSON size (1 MB).
pub const MAX_CONFIG_SIZE: usize = 1024 * 1024;

/// Namespace prefix for browser-side wasm asset caches.
pub const CACHE_NAMESPACE: &str = "sanctifier-wasm";
