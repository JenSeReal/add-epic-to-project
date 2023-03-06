use std::{env, fs};

use crate::models::{Args, Params};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let args = Args(env::args().collect());
  dbg!(args.clone());

  let params = Params::try_from(args)?;

  dbg!(params.clone());

  let crab = octocrab::OctocrabBuilder::new()
    .personal_token(params.github_token().to_string())
    .build()?;

  let event = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;

  dbg!(event);

  for (key, value) in env::vars() {
    dbg!(format!("{key}: {value}"));
  }

  // let error = &args[1];

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}
