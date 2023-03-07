use std::{env, fs};

use serde_json::json;

use crate::{
  models::{Args, Label, Operator, Params},
  queries::{get_project_user, GetProjectOrg, GetProjectUser},
};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod models;
mod queries;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let params = Params::try_from(Args(env::args().collect()))?;

  let _crab = octocrab::OctocrabBuilder::new()
    .personal_token(params.github_token().to_string())
    .build()?;

  let event = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;

  let event: models::IssueEvent = serde_json::from_str(&event)?;

  let mut labels = event.issue().labels();
  let contains = |l: &Label| params.labels().contains(&l.name().to_string());

  let is_epic = match params.operator() {
    Operator::And => labels.all(contains),
    Operator::Or => labels.any(contains),
    Operator::Not => !labels.any(contains),
  };

  dbg!(event.issue().labels().collect::<Vec<_>>());

  dbg!(is_epic);

  let project_variables = get_project_user::Variables {
    project_number: params.project_url().number(),
    project_owner_name: params.project_url().owner_name().to_string(),
  };

  let body = json!({
    "query": GetProjectUser {},
    "variables": project_variables
  });

  dbg!(body.to_string());

  // let response = crab.events().send().await?;

  // dbg!(response);

  // if event.issue().labels().

  // let error = &args[1];

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}
