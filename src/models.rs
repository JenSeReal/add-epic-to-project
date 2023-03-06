#![allow(dead_code)]

use std::str::FromStr;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::errors;

#[derive(Debug, Clone)]
pub struct Args(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct Params {
  project_url: String,
  github_token: String,
  labels: Vec<String>,
  operator: Operator,
}

impl TryFrom<Args> for Params {
  type Error = anyhow::Error;

  fn try_from(value: Args) -> Result<Self, Self::Error> {
    Ok(Self {
      project_url: value
        .0
        .get(1)
        .context("Missing project-url parameter")?
        .clone(),
      github_token: value
        .0
        .get(2)
        .context("Missing github-token parameter")?
        .clone(),
      labels: value
        .0
        .get(3)
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or(vec![]),
      operator: value
        .0
        .get(4)
        .and_then(|s| Operator::from_str(s).ok())
        .unwrap_or_default(),
    })
  }
}

impl Params {
  pub fn github_token(&self) -> &str {
    &self.github_token
  }

  pub fn operator(&self) -> &Operator {
    &self.operator
  }

  pub fn labels(&self) -> &Vec<String> {
    &self.labels
  }
}

#[derive(Debug, Clone)]
pub enum Operator {
  And,
  Or,
  Not,
}

impl FromStr for Operator {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "AND" => Ok(Self::And),
      "OR" => Ok(Self::Or),
      "NOT" => Ok(Self::Not),
      _ => Err(errors::Errors::CouldNotParseOperator(s.to_string()).into()),
    }
  }
}

impl Default for Operator {
  fn default() -> Self {
    Self::Or
  }
}

#[derive(Serialize, Deserialize)]
pub struct IssueEvent {
  issue: Issue,
}

impl IssueEvent {
  pub fn issue(&self) -> &Issue {
    &self.issue
  }
}

#[derive(Serialize, Deserialize)]
pub struct Issue {
  id: u32,
  labels: Vec<Label>,
  number: u32,
}

impl Issue {
  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn labels(&self) -> impl Iterator<Item = &Label> + '_ {
    self.labels.iter()
  }

  pub fn number(&self) -> u32 {
    self.number
  }
}

#[derive(Serialize, Deserialize)]
pub struct Label {
  id: u32,
  name: String,
}

impl Label {
  pub fn name(&self) -> &str {
    &self.name
  }
}
