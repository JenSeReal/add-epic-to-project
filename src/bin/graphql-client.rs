use std::env;

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

  let operation = GetProjectUser::build_query(get_project_user::Variables {project_owner_name: "JenSeReal".to_string(), project_number: 3});

  dbg!(serde_json::to_string(&operation)?);

  let octocrab = octocrab::OctocrabBuilder::new()
    .personal_token(env::var("GITHUB_TOKEN")?)
    .build()
    .unwrap();
  let response: Response<get_project_user::ResponseData> = octocrab.post("graphql", Some(&operation)).await.unwrap();
  println!("response: {:#?}", response.data.and_then(|data|data.user).and_then(|user|user.project_v2).and_then(|project|Some(project.id)));
  Ok(())
}
