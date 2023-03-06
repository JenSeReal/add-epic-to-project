use std::{env, fs};

use crate::models::{Args, Operator, Params};
// use std::fs::write;
// use std::process::exit;

mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
  // let mut github_output_path = env::var("GITHUB_OUTPUT").unwrap();

  let params = Params::try_from(Args(env::args().collect()))?;

  dbg!(&params);

  let crab = octocrab::OctocrabBuilder::new()
    .personal_token(params.github_token().to_string())
    .build()?;

  let event = fs::read_to_string(env::var("GITHUB_EVENT_PATH")?)?;

  let event: models::IssueEvent = serde_json::from_str(&event)?;

  dbg!(event.issue().id());

  let mut labels = event.issue().labels();

  let is_epic = match params.operator() {
    Operator::And => labels.all(|s| params.labels().contains(s)),
    Operator::Or => labels.any(|s| params.labels().contains(s)),
    Operator::Not => labels.all(|s| !params.labels().contains(s)),
  };

  dbg!(is_epic);

  // if event.issue().labels().

  // let error = &args[1];

  // if !error.is_empty() {
  //   eprintln!("Error: {error}");
  //   write(github_output_path, format!("error={error}")).unwrap();
  //   exit(1);
  // }

  Ok(())
}

#[cfg(test)]
mod test {
  use crate::models;

  const ISSUE_EVENT: &str = "{\n  \"action\": \"opened\",\n  \"issue\": {\n    \"active_lock_reason\": null,\n    \"assignee\": null,\n    \"assignees\": [],\n    \"author_association\": \"OWNER\",\n    \"body\": null,\n    \"closed_at\": null,\n    \"comments\": 0,\n    \"comments_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19/comments\",\n    \"created_at\": \"2023-03-06T13:14:03Z\",\n    \"events_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19/events\",\n    \"html_url\": \"https://github.com/JenSeReal/example-add-epic-issue-to-project/issues/19\",\n    \"id\": 1611401201,\n    \"labels\": [],\n    \"labels_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19/labels{/name}\",\n    \"locked\": false,\n    \"milestone\": null,\n    \"node_id\": \"I_kwDOJEXPh85gDAfx\",\n    \"number\": 19,\n    \"performed_via_github_app\": null,\n    \"reactions\": {\n      \"+1\": 0,\n      \"-1\": 0,\n      \"confused\": 0,\n      \"eyes\": 0,\n      \"heart\": 0,\n      \"hooray\": 0,\n      \"laugh\": 0,\n      \"rocket\": 0,\n      \"total_count\": 0,\n      \"url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19/reactions\"\n    },\n    \"repository_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project\",\n    \"state\": \"open\",\n    \"state_reason\": null,\n    \"timeline_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19/timeline\",\n    \"title\": \"Test19\",\n    \"updated_at\": \"2023-03-06T13:14:03Z\",\n    \"url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/19\",\n    \"user\": {\n      \"avatar_url\": \"https://avatars.githubusercontent.com/u/33718194?v=4\",\n      \"events_url\": \"https://api.github.com/users/JenSeReal/events{/privacy}\",\n      \"followers_url\": \"https://api.github.com/users/JenSeReal/followers\",\n      \"following_url\": \"https://api.github.com/users/JenSeReal/following{/other_user}\",\n      \"gists_url\": \"https://api.github.com/users/JenSeReal/gists{/gist_id}\",\n      \"gravatar_id\": \"\",\n      \"html_url\": \"https://github.com/JenSeReal\",\n      \"id\": 33718194,\n      \"login\": \"JenSeReal\",\n      \"node_id\": \"MDQ6VXNlcjMzNzE4MTk0\",\n      \"organizations_url\": \"https://api.github.com/users/JenSeReal/orgs\",\n      \"received_events_url\": \"https://api.github.com/users/JenSeReal/received_events\",\n      \"repos_url\": \"https://api.github.com/users/JenSeReal/repos\",\n      \"site_admin\": false,\n      \"starred_url\": \"https://api.github.com/users/JenSeReal/starred{/owner}{/repo}\",\n      \"subscriptions_url\": \"https://api.github.com/users/JenSeReal/subscriptions\",\n      \"type\": \"User\",\n      \"url\": \"https://api.github.com/users/JenSeReal\"\n    }\n  },\n  \"repository\": {\n    \"allow_forking\": true,\n    \"archive_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/{archive_format}{/ref}\",\n    \"archived\": false,\n    \"assignees_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/assignees{/user}\",\n    \"blobs_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/git/blobs{/sha}\",\n    \"branches_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/branches{/branch}\",\n    \"clone_url\": \"https://github.com/JenSeReal/example-add-epic-issue-to-project.git\",\n    \"collaborators_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/collaborators{/collaborator}\",\n    \"comments_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/comments{/number}\",\n    \"commits_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/commits{/sha}\",\n    \"compare_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/compare/{base}...{head}\",\n    \"contents_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/contents/{+path}\",\n    \"contributors_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/contributors\",\n    \"created_at\": \"2023-03-02T09:03:40Z\",\n    \"default_branch\": \"main\",\n    \"deployments_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/deployments\",\n    \"description\": null,\n    \"disabled\": false,\n    \"downloads_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/downloads\",\n    \"events_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/events\",\n    \"fork\": false,\n    \"forks\": 0,\n    \"forks_count\": 0,\n    \"forks_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/forks\",\n    \"full_name\": \"JenSeReal/example-add-epic-issue-to-project\",\n    \"git_commits_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/git/commits{/sha}\",\n    \"git_refs_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/git/refs{/sha}\",\n    \"git_tags_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/git/tags{/sha}\",\n    \"git_url\": \"git://github.com/JenSeReal/example-add-epic-issue-to-project.git\",\n    \"has_discussions\": false,\n    \"has_downloads\": true,\n    \"has_issues\": true,\n    \"has_pages\": false,\n    \"has_projects\": true,\n    \"has_wiki\": false,\n    \"homepage\": null,\n    \"hooks_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/hooks\",\n    \"html_url\": \"https://github.com/JenSeReal/example-add-epic-issue-to-project\",\n    \"id\": 608554887,\n    \"is_template\": false,\n    \"issue_comment_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/comments{/number}\",\n    \"issue_events_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues/events{/number}\",\n    \"issues_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/issues{/number}\",\n    \"keys_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/keys{/key_id}\",\n    \"labels_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/labels{/name}\",\n    \"language\": null,\n    \"languages_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/languages\",\n    \"license\": {\n      \"key\": \"mit\",\n      \"name\": \"MIT License\",\n      \"node_id\": \"MDc6TGljZW5zZTEz\",\n      \"spdx_id\": \"MIT\",\n      \"url\": \"https://api.github.com/licenses/mit\"\n    },\n    \"merges_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/merges\",\n    \"milestones_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/milestones{/number}\",\n    \"mirror_url\": null,\n    \"name\": \"example-add-epic-issue-to-project\",\n    \"node_id\": \"R_kgDOJEXPhw\",\n    \"notifications_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/notifications{?since,all,participating}\",\n    \"open_issues\": 19,\n    \"open_issues_count\": 19,\n    \"owner\": {\n      \"avatar_url\": \"https://avatars.githubusercontent.com/u/33718194?v=4\",\n      \"events_url\": \"https://api.github.com/users/JenSeReal/events{/privacy}\",\n      \"followers_url\": \"https://api.github.com/users/JenSeReal/followers\",\n      \"following_url\": \"https://api.github.com/users/JenSeReal/following{/other_user}\",\n      \"gists_url\": \"https://api.github.com/users/JenSeReal/gists{/gist_id}\",\n      \"gravatar_id\": \"\",\n      \"html_url\": \"https://github.com/JenSeReal\",\n      \"id\": 33718194,\n      \"login\": \"JenSeReal\",\n      \"node_id\": \"MDQ6VXNlcjMzNzE4MTk0\",\n      \"organizations_url\": \"https://api.github.com/users/JenSeReal/orgs\",\n      \"received_events_url\": \"https://api.github.com/users/JenSeReal/received_events\",\n      \"repos_url\": \"https://api.github.com/users/JenSeReal/repos\",\n      \"site_admin\": false,\n      \"starred_url\": \"https://api.github.com/users/JenSeReal/starred{/owner}{/repo}\",\n      \"subscriptions_url\": \"https://api.github.com/users/JenSeReal/subscriptions\",\n      \"type\": \"User\",\n      \"url\": \"https://api.github.com/users/JenSeReal\"\n    },\n    \"private\": true,\n    \"pulls_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/pulls{/number}\",\n    \"pushed_at\": \"2023-03-06T08:52:13Z\",\n    \"releases_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/releases{/id}\",\n    \"size\": 6,\n    \"ssh_url\": \"git@github.com:JenSeReal/example-add-epic-issue-to-project.git\",\n    \"stargazers_count\": 0,\n    \"stargazers_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/stargazers\",\n    \"statuses_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/statuses/{sha}\",\n    \"subscribers_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/subscribers\",\n    \"subscription_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/subscription\",\n    \"svn_url\": \"https://github.com/JenSeReal/example-add-epic-issue-to-project\",\n    \"tags_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/tags\",\n    \"teams_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/teams\",\n    \"topics\": [],\n    \"trees_url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project/git/trees{/sha}\",\n    \"updated_at\": \"2023-03-02T09:03:40Z\",\n    \"url\": \"https://api.github.com/repos/JenSeReal/example-add-epic-issue-to-project\",\n    \"visibility\": \"private\",\n    \"watchers\": 0,\n    \"watchers_count\": 0,\n    \"web_commit_signoff_required\": false\n  },\n  \"sender\": {\n    \"avatar_url\": \"https://avatars.githubusercontent.com/u/33718194?v=4\",\n    \"events_url\": \"https://api.github.com/users/JenSeReal/events{/privacy}\",\n    \"followers_url\": \"https://api.github.com/users/JenSeReal/followers\",\n    \"following_url\": \"https://api.github.com/users/JenSeReal/following{/other_user}\",\n    \"gists_url\": \"https://api.github.com/users/JenSeReal/gists{/gist_id}\",\n    \"gravatar_id\": \"\",\n    \"html_url\": \"https://github.com/JenSeReal\",\n    \"id\": 33718194,\n    \"login\": \"JenSeReal\",\n    \"node_id\": \"MDQ6VXNlcjMzNzE4MTk0\",\n    \"organizations_url\": \"https://api.github.com/users/JenSeReal/orgs\",\n    \"received_events_url\": \"https://api.github.com/users/JenSeReal/received_events\",\n    \"repos_url\": \"https://api.github.com/users/JenSeReal/repos\",\n    \"site_admin\": false,\n    \"starred_url\": \"https://api.github.com/users/JenSeReal/starred{/owner}{/repo}\",\n    \"subscriptions_url\": \"https://api.github.com/users/JenSeReal/subscriptions\",\n    \"type\": \"User\",\n    \"url\": \"https://api.github.com/users/JenSeReal\"\n  }\n}";

  #[test]
  fn test_deserealizazion_issue_event() -> anyhow::Result<(), anyhow::Error> {
    let event: models::IssueEvent = serde_json::from_str(ISSUE_EVENT)?;
    let labels: Vec<String> = vec![];

    assert_eq!(event.issue().id(), 1611401201);
    assert_eq!(event.issue().labels().cloned().collect::<Vec<_>>(), labels);
    assert_eq!(event.issue().number(), 19);

    Ok(())
  }
}
