use std::{env, fs};

use crate::models::{Args, Params};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let params = Params::try_from(Args(env::args().collect()))?;

  let crab = octocrab::OctocrabBuilder::new()
    .personal_token(params.github_token().to_string())
    .build()?;

  let event = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;

  dbg!(&event);

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
