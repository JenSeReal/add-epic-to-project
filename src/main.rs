use std::{env, fs};

use crate::models::{Args, Label, Operator, Params};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let params = Params::try_from(Args(env::args().collect()))?;
  let _crab = octocrab::OctocrabBuilder::new()
    .personal_token(params.github_token().to_string())
    .build()?;

  let event_string = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;
  let event: models::IssueEvent = serde_json::from_str(&event_string)?;

  let mut labels = event.issue().labels();
  let contains = |l: &Label| params.labels().contains(&l.name().to_string());

  let is_epic = match params.operator() {
    Operator::And => labels.all(contains),
    Operator::Or => labels.any(contains),
    Operator::Not => !labels.any(contains),
  };

  dbg!(event.issue().labels().collect::<Vec<_>>());

  dbg!(is_epic);

  Ok(())
}
