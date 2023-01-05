// We are using a lib.rs file here because we want to
// benefit from the criterion.rs benchmarking framework.
// The framework does not currently allow benchmarks to depend
// from crates from a binary crate.
// Check out this [issues comment](https://github.com/bheisler/criterion.rs/issues/353#issuecomment-553205894) for more

pub mod cmd;
pub mod config;
pub mod converters;
pub mod extractor;
pub mod hash;
pub mod languages;
pub mod players;
pub mod repo;
pub mod shapes;
pub mod utils;
