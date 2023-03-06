use std::{env, fs};

use octocrab::{etag::Etagged, models::IssueEvent, Page};

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

  dbg!(crab.clone());

  let event = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;

  // let event_crab: IssueEvent = crab.events().send().await?;

  let event: models::IssueEvent = serde_json::from_str(&event)?;

  dbg!(event.issue().id());

  // let error = &args[1];

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}
