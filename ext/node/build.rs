// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

fn main() {
  // we use a changing variable name to make it harder to depend on this
  let crate_version = env!("CARGO_PKG_VERSION");
  println!(
    "cargo:rustc-env=NODE_GLOBAL_THIS_NAME=__DENO_NODE_GLOBAL_THIS_{}__ TARGET={}",
    crate_version.replace('.', "_"),
    std::env::var("TARGET").unwrap()
  );
  println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
}
