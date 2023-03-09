use std::{env, fs};

use serde_json::json;

use crate::{
  models::{Args, Label, Operator, Params},
};
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

  let project_query = format!(
    r#"query getProject($projectOwnerName: String!, $projectNumber: Int!) {{
      {:#?}(login: $projectOwnerName) {{
        projectV2(number: $projectNumber) {{
          id
        }}
      }}
    }}
  "#,
    params.project().owner_type()
  );

  let project_request = json!({
    "query": project_query,
    "variables": {
      "project_number": params.project().number(),
      "project_owner_name": params.project().owner_name().to_string(),
    }
  });

  dbg!(project_query);

  let response = crab.post("graphql", Some(&project_request)).await?;

  dbg!(response);

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}
