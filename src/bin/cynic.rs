use std::env;

use crate::queries::GetProjectUser;

#[cynic::schema_for_derives(file = r#"schemas/github.graphql"#, module = "schema")]
mod queries {
  use super::schema;

  #[derive(cynic::QueryVariables, Debug)]
  pub struct GetProjectUserVariables {
    pub project_number: i32,
    pub project_owner_name: String,
  }

  #[derive(cynic::QueryFragment, Debug)]
  #[cynic(graphql_type = "Query", variables = "GetProjectUserVariables")]
  pub struct GetProjectUser {
    #[arguments(login: $project_owner_name)]
    pub user: Option<User>,
  }

  #[derive(cynic::QueryFragment, Debug)]
  #[cynic(variables = "GetProjectUserVariables")]
  pub struct User {
    #[arguments(number: $project_number)]
    pub project_v2: Option<ProjectV2>,
  }

  #[derive(cynic::QueryFragment, Debug)]
  pub struct ProjectV2 {
    pub id: cynic::Id,
  }
}

#[allow(non_snake_case, non_camel_case_types)]
mod schema {
  cynic::use_schema!(r#"schemas/github.graphql"#);
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  use cynic::{GraphQlResponse, QueryBuilder};

  dotenv::dotenv().ok();

  let operation = queries::GetProjectUser::build(queries::GetProjectUserVariables {
    project_number: 3,
    project_owner_name: "JenSeReal".to_string(),
  });

  dbg!(serde_json::to_string(&operation)?);

  let octocrab = octocrab::OctocrabBuilder::new()
    .personal_token(env::var("GITHUB_TOKEN")?)
    .build()
    .unwrap();
  let response: GraphQlResponse<GetProjectUser> =
    octocrab.post("graphql", Some(&operation)).await.unwrap();

  println!(
    "response: {:#?}",
    response
      .data
      .and_then(|data| data.user)
      .and_then(|user| user.project_v2)
      .map(|project| project.id)
  );
  Ok(())
}
