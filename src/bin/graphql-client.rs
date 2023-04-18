use std::env;

use anyhow::Context;
use graphql_client::{GraphQLQuery, Response};

macro_rules! generate_query {
  ($query:ident) => {
    #[derive(GraphQLQuery)]
    #[graphql(
      schema_path = "schemas/github.graphql",
      query_path = "schemas/queries.graphql",
      response_derives = "Debug"
    )]
    struct $query;
  };
}

generate_query!(GetProjectUser);
generate_query!(GetProjectOrg);

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  dotenv::dotenv().ok();

  let octocrab = octocrab::OctocrabBuilder::new()
    .personal_token(env::var("GITHUB_TOKEN")?)
    .build()?;

  let get_project_id = GetProjectUser::build_query(get_project_user::Variables {
    project_owner_name: "JenSeReal".to_string(),
    project_number: 3,
  });

  let get_project_id_res: Response<get_project_user::ResponseData> =
    octocrab.post("graphql", Some(&get_project_id)).await?;

  let project_id = get_project_id_res
    .data
    .and_then(|data| Some(data.user?.project_v2?.id))
    .context("Could not get project id")?;

  dbg!(project_id);

  Ok(())
}
