#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod args;
pub mod auth_tokens;
pub mod cache;
pub mod cdp;
pub mod emit;
pub mod errors;
pub mod factory;
pub mod file_fetcher;
pub mod graph_util;
pub mod http_util;
#[cfg(feature = "tools")]
pub mod js;
#[cfg(feature = "tools")]
pub mod lsp;
pub mod module_loader;
pub mod napi;
pub mod node;
pub mod npm;
pub mod ops;
pub mod resolver;
#[cfg(feature = "tools")]
pub mod standalone;
pub mod tools;
#[cfg(feature = "tools")]
pub mod tsc;
pub mod util;
pub mod version;
pub mod worker;

pub use crate::args::flags_from_vec;
pub use crate::args::DenoSubcommand;
pub use crate::args::Flags;
pub use crate::util::display;
pub use crate::util::v8::get_v8_flags_from_env;
pub use crate::util::v8::init_v8_flags;

pub use deno_core::anyhow::Context;
pub use deno_core::error::AnyError;
pub use deno_core::error::JsError;
pub use deno_core::futures::FutureExt;
pub use deno_core::unsync::JoinHandle;
pub use deno_terminal::colors;
pub use deno_runtime::fmt_errors::format_js_error;
pub use deno_runtime::tokio_util::create_and_run_current_thread_with_maybe_metrics;
pub use factory::CliFactory;
pub use std::env;
pub use std::env::current_exe;
pub use std::future::Future;
pub use std::path::PathBuf;

pub use deno_runtime::UNSTABLE_GRANULAR_FLAGS;

pub(crate) fn unstable_exit_cb(feature: &str, api_name: &str) {
  eprintln!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  std::process::exit(70);
}

#[allow(dead_code)]
pub(crate) fn unstable_warn_cb(feature: &str) {
  eprintln!(
    "The `--unstable` flag is deprecated, use --unstable-{feature} instead."
  );
}
