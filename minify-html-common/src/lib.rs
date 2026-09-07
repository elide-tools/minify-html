pub mod gen;
pub mod pattern;
pub mod spec;
// Shared fixtures for the dependent crates' test suites; off by default so they don't land in
// release builds (`#[cfg(test)]` can't work here, as the consumers are other crates).
#[cfg(feature = "test-data")]
pub mod tests;
pub mod whitespace;
