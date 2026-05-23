//! Small documented surfaces shared by Divergent's binary and tests.
//!
//! Most of Divergent is terminal application code, so it intentionally stays
//! behind the binary. The pieces exposed here are the parts worth treating as
//! stable contracts: compact, parseable inputs that other code can reason
//! about without booting the UI.

/// Revision expression parsing shared by the CLI and documented examples.
pub mod commit_reference;
