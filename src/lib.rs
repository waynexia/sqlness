// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

//! **SQL** integration test har**NESS**
//!
//! ## Usage
//!
//! The entrypoint of this crate is [Runner] struct. It runs your test cases and
//! compare the result. There are three things you need to do to complete an
//! integration test:
//! - Prepare the test case (of course!)
//! - Implement [`Environment`] and [`Database`]. They provide methods to start
//!   the server, submit the query and clean up etc.
//! - Format the result. Implement [`Display`] for your query result to make them
//!   comparable.
//!
//! And then all you need is to run the runner!
//!
//! ```rust, no_run
//! async fn run_integration_test() {
//!     let runner = Runner::new(root_path, env).await;
//!     runner.run().await;
//! }
//! ```
//!
//! [`Display`]: std::fmt::Display
//!
//! ## Directory organization
//!
//! An example directory tree is:
//!
//! ```plaintext
//! sqlness
//! ├── local
//! │   ├── config.toml
//! │   ├── dml
//! │   │   └── basic.sql
//! │   ├── ddl
//! │   └── system
//! └── remote
//!     ├── config.toml
//!     ├── dml
//!     ├── ddl
//!     └── system
//! ```
//!
//! Here the root dir is `sqlness`, it contains two sub-directories for different
//! [`Environment`]s `local` and `remote`. Each environment has an env-specific
//! configuration `config.toml`. All the test cases are placed under corresponding
//! environment directory.
//!
//! Note that only the first layer of sub-directory is special (for distinguash
//! different environments). All deeper layers are treated as the same. E.g.,
//! both `sqlness/local/dml/basic.sql` and `sqlness/local/dml/another-dir/basic.sql`
//! will be run under the `local` in the same pass.

mod case;
mod config;
mod database;
mod environment;
mod error;
mod runner;

pub use config::{Config, ConfigBuilder};
pub use database::Database;
pub use environment::Environment;
pub use error::SqlnessError;
pub use runner::Runner;
