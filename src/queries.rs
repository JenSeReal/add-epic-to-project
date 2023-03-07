use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "schemas/github.graphql",
  query_path = "schemas/get_project_org.graphql"
)]
pub struct GetProjectOrg;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
  schema_path = "schemas/github.graphql",
  query_path = "schemas/get_project_user.graphql"
)]
pub struct GetProjectUser;

impl GetProjectUser {
  pub fn new() -> Self {
    Self {}
  }
}
