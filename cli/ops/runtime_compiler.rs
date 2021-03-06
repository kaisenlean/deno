// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{Deserialize, JsonOp, Value};
use crate::compilers::runtime_compile;
use crate::compilers::runtime_transpile;
use crate::futures::FutureExt;
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ZeroCopyBuf;
use std::collections::HashMap;

pub fn init(i: &mut CoreIsolate, s: &State) {
  i.register_op("op_compile", s.stateful_json_op(op_compile));
  i.register_op("op_transpile", s.stateful_json_op(op_transpile));
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CompileArgs {
  root_name: String,
  sources: Option<HashMap<String, String>>,
  bundle: bool,
  options: Option<String>,
}

fn op_compile(
  state: &State,
  args: Value,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<JsonOp, OpError> {
  state.check_unstable("Deno.compile");
  let args: CompileArgs = serde_json::from_value(args)?;
  let global_state = state.borrow().global_state.clone();
  let fut = async move {
    runtime_compile(
      global_state,
      &args.root_name,
      &args.sources,
      args.bundle,
      &args.options,
    )
    .await
  }
  .boxed_local();
  Ok(JsonOp::Async(fut))
}

#[derive(Deserialize, Debug)]
struct TranspileArgs {
  sources: HashMap<String, String>,
  options: Option<String>,
}

fn op_transpile(
  state: &State,
  args: Value,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<JsonOp, OpError> {
  state.check_unstable("Deno.transpile");
  let args: TranspileArgs = serde_json::from_value(args)?;
  let global_state = state.borrow().global_state.clone();
  let fut = async move {
    runtime_transpile(global_state, &args.sources, &args.options).await
  }
  .boxed_local();
  Ok(JsonOp::Async(fut))
}
