use std::path::Path;

use deno_ast::{MediaType, ParseParams, SourceTextInfo};
use deno_core::error::AnyError;
use deno_core::ExtensionFileSource;
use deno_core::ModuleCode;

pub fn transpile_ts_for_snapshotting(
  file_source: &ExtensionFileSource,
) -> Result<ModuleCode, AnyError> {
  let media_type = MediaType::from_path(Path::new(&file_source.specifier));

  let should_transpile = match media_type {
    MediaType::JavaScript => false,
    MediaType::Mjs => false,
    MediaType::TypeScript => true,
    _ => panic!(
      "Unsupported media type for snapshotting {media_type:?} for file {}",
      file_source.specifier
    ),
  };
  let code = file_source.load()?;

  if !should_transpile {
    return Ok(code);
  }

  let parsed = deno_ast::parse_module(ParseParams {
    specifier: file_source.specifier.to_string(),
    text_info: SourceTextInfo::from_string(code.take_as_string()),
    media_type,
    capture_tokens: false,
    scope_analysis: false,
    maybe_syntax: None,
  })?;
  let transpiled_source = parsed.transpile(&deno_ast::EmitOptions {
    imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Remove,
    inline_source_map: false,
    ..Default::default()
  })?;

  Ok(transpiled_source.text.into())
}
