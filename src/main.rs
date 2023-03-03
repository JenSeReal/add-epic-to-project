use std::env;

use crate::params::{Args, Params};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod params;

fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let args = Args(env::args().collect());
  dbg!(args.clone());

  let params = Params::try_from(args)?;

  dbg!(params);

  // let error = &args[1];

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}
