// graphql-client generate --schema-path ./graphql/github.schema.graphql --custom-scalars-module crate::gql_types --output-directory ./src/ --response-derives Debug ./graphql/github_queries.graphql

mod convert;
mod github_queries;
pub(crate) mod gql_types {
    #[allow(clippy::upper_case_acronyms)]
    pub(crate) type URI = String;
    pub(crate) type DateTime = String; //chrono::DateTime<chrono::Utc>;
}

use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use conv::ValueFrom;
use github_queries::{
    issues_and_prs_query, organization_repos_query, user_contributed_repos_query,
    user_contributed_repos_query::{
        UserContributedReposQueryUserRepositoriesContributedToNodes,
        // Using this very long name in the code causes rustfmt to not format
        // any block containing this name.
        UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTarget as ContributedRefTarget,
    },
    user_repos_query::{self, ReposNodes, ReposNodesLanguages},
    IssuesAndPrsQuery, OrganizationReposQuery, UserContributedReposQuery, UserReposQuery,
};
use graphql_client::Response;
use human_bytes::human_bytes;
use itertools::{EitherOrBoth, Itertools};
use once_cell::sync::Lazy;
use reqwest::Client;
use rss::Channel;
use serde_derive::Serialize;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Write,
    path::PathBuf,
};
use tinytemplate::TinyTemplate;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const MY_LOGIN: &str = "autarch";
const MY_ORG: &str = "houseabsolute";
const MY_EMAIL: &str = "autarch@urth.org";
static WORK_REPOS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut h = HashSet::new();
    h.insert("10gen");
    h.insert("10gen-archive");
    h.insert("mongodb");
    h.insert("mongodb-forks");
    h.insert("mongodb-labs");
    h
});
// These are repos in other orgs that are my projects. There are others I
// could add but this is just the ones that I've worked on the last few years.
static MY_EXTERNAL_REPOS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut h = HashSet::new();
    h.insert("moose/Dist-Zilla-Plugin-Conflicts");
    h.insert("moose/Package-DeprecationManager");
    h
});

const DATE_FORMAT: &str = "%Y-%m-%d";

const API_URL: &str = "https://api.github.com/graphql";

#[derive(Debug, Serialize)]
struct OneRepo {
    full_name: String,
    url: String,
    fork_count: i64,
    stargazer_count: i64,
    committed_date: String,
}

#[derive(Debug, Default, Serialize)]
struct UserAndRepoStats {
    created_at: String,
    total_repos: i64,
    owned_repos: i64,
    forked_repos: i64,
    live_repos: i64,
    all_time_languages: HashMap<String, (String, i64)>,
    recent_languages: HashMap<String, (String, i64)>,
    my_repos: Vec<OneRepo>,
    other_repos: Vec<OneRepo>,
}

#[derive(Debug, Serialize)]
struct TopRepos<'a> {
    my_most_recent: Vec<&'a OneRepo>,
    other_most_recent: Vec<&'a OneRepo>,
    most_starred: Vec<&'a OneRepo>,
    most_forked: Vec<&'a OneRepo>,
}

#[derive(Debug, Serialize)]
struct LanguageStat<'a> {
    name: &'a str,
    color: &'a str,
    percentage: f64,
    bytes: String,
}

#[derive(Debug, Default, Serialize)]
struct IssueAndPrStats {
    issues_created: i64,
    issues_closed: i64,
    prs_created: i64,
    prs_merged: i64,
}

#[derive(Debug, Serialize)]
struct BlogPost {
    title: String,
    date: String,
    url: String,
}

#[derive(Serialize)]
struct Context<'a> {
    blog_posts: Vec<BlogPost>,
    user_and_repo_stats: &'a UserAndRepoStats,
    recent_commits: Vec<(String, String)>,
    top_repos: TopRepos<'a>,
    issue_and_pr_stats: IssueAndPrStats,
    top_languages: Vec<(String, String)>,
    top_artists: Vec<String>,
    generated_at: String,
}

impl ReposNodes {
    fn committed_date(&self) -> Option<&str> {
        let default_target = self
            .default_branch_ref
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Could not get default branch ref for repo {}",
                    self.name_with_owner
                )
            })
            .target
            .as_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Could not get target of default branch ref for repo {}",
                    self.name_with_owner
                )
            });
        match default_target {
            user_repos_query::ReposNodesDefaultBranchRefTarget::Commit(c) => {
                let nodes = c.history.nodes.as_ref().unwrap_or_else(|| {
                    panic!(
                        "Could not get history nodes for default target of repo {}",
                        self.name_with_owner
                    )
                });
                if nodes.is_empty() {
                    return None;
                }
                Some(
                    nodes[0]
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!(
                                "Could not get a commit from nodes for default target of repo {}",
                                self.name_with_owner
                            )
                        })
                        .committed_date
                        .as_ref(),
                )
            }
            _ => None,
        }
    }

    fn is_mine(&self) -> bool {
        repo_is_mine(&self.owner.login, &self.name_with_owner)
    }

    fn is_fork_of_mine(&self) -> bool {
        if !self.is_fork {
            return false;
        }
        let parent = self
            .parent
            .as_ref()
            .unwrap_or_else(|| panic!("Repo {} is a fork but has no parent", self.name_with_owner));
        repo_is_mine(&parent.owner.login, "")
    }
}

impl UserContributedReposQueryUserRepositoriesContributedToNodes {
    fn is_mine(&self) -> bool {
        repo_is_mine(&self.owner.login, &self.name_with_owner)
    }

    fn is_fork_of_mine(&self) -> bool {
        if !self.is_fork {
            return false;
        }
        let parent = self
            .parent
            .as_ref()
            .unwrap_or_else(|| panic!("Repo {} is a fork but has no parent", self.name_with_owner));
        repo_is_mine(&parent.owner.login, "")
    }
}

fn repo_is_mine(login: &str, name_with_owner: &str) -> bool {
    login == MY_LOGIN || login == MY_ORG || MY_EXTERNAL_REPOS.contains(name_with_owner)
}

impl OneRepo {
    fn string_for_template(&self) -> String {
        format!(
            "[{}]({}) - {}",
            self.full_name, self.url, self.committed_date
        )
    }
}

impl From<ReposNodes> for OneRepo {
    fn from(repo: ReposNodes) -> Self {
        let date_str = repo.committed_date().unwrap_or_else(|| {
            panic!(
                "Could not get committed date for repo {}",
                repo.name_with_owner,
            )
        });
        let committed_date = DateTime::parse_from_rfc3339(date_str)
            .unwrap_or_else(|e| panic!("Could not parse '{date_str}' as RFC3339 datetime: {e}"))
            .with_timezone(&Utc)
            .format(DATE_FORMAT)
            .to_string();
        OneRepo {
            full_name: repo.name_with_owner,
            url: repo.url,
            fork_count: repo.fork_count,
            stargazer_count: repo.stargazer_count,
            committed_date,
        }
    }
}

impl LanguageStat<'_> {
    fn string_for_template(&self) -> String {
        format!("{}: {}%, {}", self.name, self.percentage, self.bytes)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("GITHUB_TOKEN")
        .expect("You must set the GITHUB_TOKEN env var when running this program");
    let bearer = format!("Bearer {token}");
    let client = Client::builder()
        .user_agent(format!("autarch-profile-generator/{VERSION}"))
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&bearer)
                    .unwrap_or_else(|e| panic!("Could not parse header from '{bearer}': {e}")),
            ))
            .collect(),
        )
        .build()?;

    let blog_posts = blog_posts().await?;
    tracing::debug!("{blog_posts:#?}");
    let user_and_repo_stats = user_and_repo_stats(&client).await?;
    tracing::debug!("{user_and_repo_stats:#?}");
    let top_repos = top_repos(&user_and_repo_stats);
    tracing::debug!("{top_repos:#?}");
    let top_all_time_languages = top_languages(&user_and_repo_stats.all_time_languages)?;
    tracing::debug!("{top_all_time_languages:#?}");
    let top_recent_languages = top_languages(&user_and_repo_stats.recent_languages)?;
    tracing::debug!("{top_recent_languages:#?}");
    let issue_and_pr_stats = issue_and_pr_stats(&client).await?;
    tracing::debug!("{issue_and_pr_stats:#?}");
    let top_artists = top_artists().await?;
    tracing::debug!("{top_artists:#?}");

    let mut tt = TinyTemplate::new();
    tt.add_template("readme", README_TEMPLATE)?;
    let context = Context {
        blog_posts,
        user_and_repo_stats: &user_and_repo_stats,
        recent_commits: repo_pairs_for_template(
            &top_repos.my_most_recent,
            &top_repos.other_most_recent,
        ),
        top_repos,
        issue_and_pr_stats,
        top_languages: language_pairs_for_template(top_recent_languages, top_all_time_languages),
        top_artists,
        generated_at: Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
    };

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("README.md");
    let mut file = File::create(path)?;
    file.write_all(tt.render("readme", &context)?.as_bytes())?;

    Ok(())
}

async fn blog_posts() -> Result<Vec<BlogPost>> {
    if !should_get("blog_posts") {
        tracing::info!("Skipping blog posts");
        return Ok(vec![]);
    }
    tracing::info!("Getting blog feed");
    let content = reqwest::get("https://blog.urth.org/index.xml")
        .await?
        .bytes()
        .await?;
    let mut channel = Channel::read_from(&content[..])?;
    channel
        .items
        .splice(0..5, None)
        .map(|i| {
            let title = i
                .title()
                .unwrap_or_else(|| panic!("Blog post has no title"));
            let dt = DateTime::parse_from_rfc2822(
                i.pub_date()
                    .as_ref()
                    .unwrap_or_else(|| panic!("Blog post '{title}', has no publication date")),
            )?;
            Ok(BlogPost {
                title: title.to_string(),
                date: dt.date_naive().format(DATE_FORMAT).to_string(),
                url: i
                    .link()
                    .unwrap_or_else(|| panic!("Blog post '{title}', has no link"))
                    .to_string(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

async fn user_and_repo_stats(client: &Client) -> Result<UserAndRepoStats> {
    let mut stats = UserAndRepoStats::default();

    get_my_user_repos(client, &mut stats).await?;
    get_my_org_repos(client, &mut stats).await?;
    get_other_repos(client, &mut stats).await?;
    stats.other_repos = stats
        .other_repos
        .into_iter()
        .unique_by(|r| r.full_name.clone())
        .collect();

    Ok(stats)
}

async fn get_my_user_repos(client: &Client, stats: &mut UserAndRepoStats) -> Result<()> {
    if !should_get("user_repos") {
        tracing::info!("Skipping user repos");
        return Ok(());
    }

    let mut after = None;
    loop {
        tracing::info!("Getting user repos after {after:?}");
        let resp = user_query(client, after).await?;
        tracing::debug!("{resp:#?}");

        if let Some(errors) = resp.errors {
            tracing::error!("Errors in user repos query: {:?}", errors);
            return Err(anyhow::anyhow!("user repos query returned errors"));
        }

        let user = resp
            .data
            .unwrap_or_else(|| panic!("Response for user repos has no data"))
            .user
            .unwrap_or_else(|| panic!("Response data for user repos has no user"));
        if stats.created_at.is_empty() {
            stats.created_at = user.created_at;
        }

        collect_user_repo_stats(
            stats,
            // This query returns some of my houseabsolute org repos, but not
            // all. So we filter those out from this response and get the full
            // list of repos by querying the org later.
            user.repositories
                .nodes
                .unwrap_or_else(|| panic!("Response for user repos has no nodes for repositories"))
                .into_iter()
                .filter(|r| {
                    r.as_ref()
                        .unwrap_or_else(|| {
                            panic!("Response for user repos has an empty repositories node")
                        })
                        .owner
                        .login
                        != MY_ORG
                })
                .collect(),
        )?;

        if user.repositories.page_info.has_next_page {
            after = user.repositories.page_info.end_cursor;
        } else {
            break;
        }
    }

    Ok(())
}

async fn user_query(
    client: &Client,
    after: Option<String>,
) -> Result<Response<user_repos_query::ResponseData>> {
    for i in 1..5 {
        let vars = user_repos_query::Variables {
            login: MY_LOGIN.to_string(),
            email: MY_EMAIL.to_string(),
            after: after.clone(),
        };
        let resp = post_graphql::<UserReposQuery, _>(client, API_URL, vars).await?;
        if let Some(errors) = resp.errors {
            eprintln!("user query attempt #{i}: {}", errors[0].message);
        } else {
            return Ok(resp);
        }
    }
    panic!("Could not get results for user query after 5 attempts");
}

async fn get_my_org_repos(client: &Client, stats: &mut UserAndRepoStats) -> Result<()> {
    if !should_get("organization_repos") {
        tracing::info!("Skipping organization repos");
        return Ok(());
    }

    let mut after = None;
    loop {
        tracing::info!("Getting organization repos after {after:?}");
        let resp = organization_query(client, after).await?;
        tracing::debug!("{resp:#?}");

        if let Some(errors) = resp.errors {
            tracing::error!("Errors in organization repos query: {:?}", errors);
            return Err(anyhow::anyhow!("organization repos query returned errors"));
        }

        let organization = resp
            .data
            .unwrap_or_else(|| panic!("Response for org repos has no data"))
            .organization
            .unwrap_or_else(|| panic!("Response data for user repos has no organization"));
        collect_user_repo_stats(
            stats,
            organization
                .repositories
                .nodes
                .unwrap_or_else(|| panic!("Response for org repos has no nodes for repositories"))
                .into_iter()
                .map(|n| n.map(std::convert::Into::into))
                .collect(),
        )?;

        if organization.repositories.page_info.has_next_page {
            after = organization.repositories.page_info.end_cursor;
        } else {
            break;
        }
    }

    Ok(())
}

async fn organization_query(
    client: &Client,
    after: Option<String>,
) -> Result<Response<organization_repos_query::ResponseData>> {
    for i in 1..5 {
        let vars = organization_repos_query::Variables {
            login: MY_ORG.to_string(),
            email: MY_EMAIL.to_string(),
            after: after.clone(),
        };
        let resp = post_graphql::<OrganizationReposQuery, _>(client, API_URL, vars).await?;
        if let Some(errors) = resp.errors {
            eprintln!("organization query attempt #{i}: {}", errors[0].message);
        } else {
            return Ok(resp);
        }
    }
    panic!("Could not get results for organization query after 5 attempts");
}

static FILTER_DATE: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = chrono::Utc::now();
    // The chrono::Duration struct cannot represent 2 years, only multiple of
    // weeks, but two years is not 104 weeks.  let two_years_ago =
    let two_years_ago = format!("{}-{}", now.year() - 2, now.format("%m-%dT%H:%M:%SZ"),);
    chrono::DateTime::parse_from_rfc3339(&two_years_ago)
        .unwrap_or_else(|e| panic!("Could not parse `{two_years_ago}` as an RFC3339 date: {e}"))
        .with_timezone(&Utc)
});

fn collect_user_repo_stats(
    stats: &mut UserAndRepoStats,
    repos: Vec<Option<user_repos_query::ReposNodes>>,
) -> Result<()> {
    for repo in repos.into_iter().map(|r| r.unwrap()) {
        if repo.is_archived || repo.is_disabled || repo.is_empty || repo.is_private {
            continue;
        }
        if WORK_REPOS.contains(repo.owner.login.as_str()) {
            continue;
        }

        // Only repos I own are counted for stats, but I want to save others
        // to show recent commits I made to other repos.
        if !repo.is_mine() {
            if repo.is_fork_of_mine() {
                tracing::debug!("Skipping fork of my own repo: {}", repo.name_with_owner);
                continue;
            }

            let committed_date = repo.committed_date();
            if committed_date.is_none() {
                continue;
            }

            let committed_date =
                DateTime::parse_from_rfc3339(committed_date.unwrap())?.with_timezone(&Utc);
            if committed_date < *FILTER_DATE {
                continue;
            }
            stats.other_repos.push(repo.into());
            continue;
        }

        stats.total_repos += 1;
        if repo.is_fork {
            stats.forked_repos += 1;
            continue;
        }

        stats.owned_repos += 1;

        let languages = repo
            .languages
            .as_ref()
            .unwrap_or_else(|| panic!("Repo {} has no languages", repo.name_with_owner));
        collect_language_stats(
            &mut stats.all_time_languages,
            repo.name_with_owner.as_str(),
            languages,
        );

        let committed_date = repo.committed_date();
        if committed_date.is_none() {
            continue;
        }

        let committed_date =
            DateTime::parse_from_rfc3339(committed_date.unwrap())?.with_timezone(&Utc);
        if committed_date >= *FILTER_DATE {
            collect_language_stats(
                &mut stats.recent_languages,
                repo.name_with_owner.as_str(),
                languages,
            );

            stats.live_repos += 1;
        }
        stats.my_repos.push(repo.into());
    }

    Ok(())
}

async fn get_other_repos(client: &Client, stats: &mut UserAndRepoStats) -> Result<()> {
    if !should_get("other_repos") {
        tracing::info!("Skipping other repos");
        return Ok(());
    }

    let mut after = None;
    loop {
        tracing::info!("Getting other repos with recent contributions after {after:?}");
        let resp = post_graphql::<UserContributedReposQuery, _>(
            client,
            API_URL,
            user_contributed_repos_query::Variables {
                login: MY_LOGIN.to_string(),
                email: MY_EMAIL.to_string(),
                after,
            },
        )
        .await?;
        tracing::debug!("{resp:#?}");

        if let Some(errors) = resp.errors {
            tracing::error!("Errors in other repos query: {:?}", errors);
            return Err(anyhow::anyhow!("other repos query returned errors"));
        }

        let contributions = resp
            .data
            .unwrap_or_else(|| panic!("Response for other repos has no data"))
            .user
            .unwrap_or_else(|| panic!("Response for other repos has no user"))
            .repositories_contributed_to;
        for repo in contributions
            .nodes
            .expect("Contributions response has no nodes")
            .into_iter()
            .flatten()
        {
            if should_skip_other_repo(&repo) {
                continue;
            }

            let Some(committed_date) = committed_date_for_repo(&repo) else {
                continue;
            };
            let committed_date = DateTime::parse_from_rfc3339(&committed_date)
                .unwrap_or_else(|e| {
                    panic!("Could not parse '{committed_date}' as RFC3339 datetime: {e}")
                })
                .with_timezone(&Utc)
                .format(DATE_FORMAT)
                .to_string();
            stats.other_repos.push(OneRepo {
                full_name: repo.name_with_owner,
                url: repo.url,
                fork_count: 0,
                stargazer_count: 0,
                committed_date,
            });
        }

        if contributions.page_info.has_next_page {
            after = contributions.page_info.end_cursor;
        } else {
            break;
        }
    }

    Ok(())
}

fn should_skip_other_repo(
    repo: &UserContributedReposQueryUserRepositoriesContributedToNodes,
) -> bool {
    if repo.is_mine() {
        tracing::debug!(
            "Skipping my own repo returned in other repos query: {}",
            repo.name_with_owner,
        );
        true
    } else if WORK_REPOS.contains(repo.owner.login.as_str()) {
        tracing::debug!("Skipping work repo owned by {}", repo.owner.login);
        true
    } else if repo.is_fork_of_mine() {
        tracing::debug!("Skipping fork of my own repo: {}", repo.name_with_owner);
        true
    } else {
        false
    }
}

fn committed_date_for_repo(
    repo: &UserContributedReposQueryUserRepositoriesContributedToNodes,
) -> Option<&str> {
    match repo
        .default_branch_ref
        .as_ref()
        .unwrap_or_else(|| {
            panic!(
                "Could not get default branch ref for repo {}",
                repo.name_with_owner
            )
        })
        .target
        .as_ref()
    {
        Some(ContributedRefTarget::Commit(c)) => {
            let nodes = c.history.nodes.as_ref().unwrap_or_else(|| {
                panic!(
                    "Could not get history nodes for default target of repo {}",
                    repo.name_with_owner
                )
            });
            if nodes.is_empty() {
                return None;
            }
            Some(
                nodes
                    .last()
                    .unwrap_or_else(|| {
                        panic!(
                            "History nodes for default target of repo {} is empty",
                            repo.name_with_owner
                        )
                    })
                    .as_ref()
                    .unwrap_or_else(|| {
                        panic!(
                            "Could not get a commit from nodes for default target of repo {}",
                            repo.name_with_owner
                        )
                    })
                    .committed_date
                    .as_str(),
            )
        }
        _ => None,
    }
}

const REPOS_TO_IGNORE_FOR_LANGUAGE_STATS: &[&str] = &[
    // The presentations repo has a ton of HTML and JS I didn't write
    // and this distorts the stats.
    "autarch/presentations",
    // The mason book is HTML, but it's just the HTMl from the old dynamic
    // site which I crawled, so it's not interesting for these stats.
    "autarch/masonbook.houseabsolute.com",
];

fn collect_language_stats(
    stats: &mut HashMap<String, (String, i64)>,
    repo_name: &str,
    languages: &ReposNodesLanguages,
) {
    let lang_sizes = languages
        .edges
        .as_ref()
        .unwrap()
        .iter()
        .map(|e| e.as_ref().unwrap().size)
        .collect::<Vec<_>>();
    let lang_names_and_colors = languages
        .nodes
        .as_ref()
        .unwrap()
        .iter()
        .map(|l| {
            let l = l.as_ref().unwrap();
            (l.name.as_str(), l.color.as_deref())
        })
        .collect::<Vec<_>>();

    assert!(
        (lang_sizes.len() == lang_names_and_colors.len()),
        "language sizes and names differ in length: {} != {} for {}",
        lang_sizes.len(),
        lang_names_and_colors.len(),
        repo_name,
    );
    if !lang_sizes.is_empty() && !REPOS_TO_IGNORE_FOR_LANGUAGE_STATS.contains(&repo_name) {
        for i in 0..lang_sizes.len() - 1 {
            let lang = match (repo_name, lang_names_and_colors[i].0) {
                // This is really XS, not C (although arguably, XS is just C).
                ("houseabsolute/File-LibMagic", "C") => "XS",
                (_, l) => l,
            };

            // The tidyall repo has a bunch of PHP and JS checked in for
            // testing, but none of it is code I've written or maintained.
            if repo_name == "houseabsolute/perl-code-tidyall" && lang != "Perl" {
                continue;
            }
            let color = language_color(lang, lang_names_and_colors[i].1);
            let size = lang_sizes[i];
            if let Some(v) = stats.get_mut(lang) {
                v.1 += size;
            } else {
                stats.insert(lang.to_string(), (color.to_string(), size));
            }
        }
    }
}

fn language_color<'a>(lang: &str, color: Option<&'a str>) -> &'a str {
    match color {
        Some(c) => c,
        None => match lang {
            "Perl 6" => "#00A9E0",
            "XS" => "#021c9e", // a darker blue than Perl,
            _ => panic!("No color for {lang}"),
        },
    }
}

fn top_repos(stats: &UserAndRepoStats) -> TopRepos<'_> {
    let my_most_recent = top_n(&stats.my_repos, 10, |a, b| {
        b.committed_date.cmp(&a.committed_date)
    });
    let other_most_recent = top_n(&stats.other_repos, 10, |a, b| {
        b.committed_date.cmp(&a.committed_date)
    });
    let most_forked = top_n(&stats.my_repos, 5, |a, b| b.fork_count.cmp(&a.fork_count));
    let most_starred = top_n(&stats.my_repos, 5, |a, b| {
        b.stargazer_count.cmp(&a.stargazer_count)
    });
    TopRepos {
        my_most_recent,
        other_most_recent,
        most_starred,
        most_forked,
    }
}

fn top_n<S>(repos: &[OneRepo], take: usize, sorter: S) -> Vec<&OneRepo>
where
    S: FnMut(&&OneRepo, &&OneRepo) -> Ordering,
{
    repos.iter().sorted_by(sorter).take(take).collect()
}

fn top_languages(languages: &HashMap<String, (String, i64)>) -> Result<Vec<LanguageStat>> {
    let total_size: i64 = languages.values().map(|v| v.1).sum();
    let colors: HashMap<&str, &str> = languages
        .iter()
        .map(|(k, v)| (k.as_str(), v.0.as_str()))
        .collect();

    let mut language_sums: HashMap<&str, i64> = HashMap::new();
    for (lang, (_, size)) in languages {
        if let Some(v) = language_sums.get_mut(lang.as_str()) {
            *v += *size;
        } else {
            language_sums.insert(lang, *size);
        }
    }

    let mut top = vec![];
    for (name, sum) in language_sums {
        let pct = (f64::value_from(sum)? / f64::value_from(total_size)?) * 100.0;
        if pct < 1.0 {
            tracing::debug!("Skipping language {name} with total percentage of {pct}");
            continue;
        }
        top.push(LanguageStat {
            name,
            color: colors.get(name).unwrap(),
            percentage: pct.round(),
            bytes: human_bytes(f64::value_from(sum)?),
        });
    }

    top.sort_by(|a, b| b.percentage.total_cmp(&a.percentage));
    Ok(top)
}

async fn issue_and_pr_stats(client: &Client) -> Result<IssueAndPrStats> {
    if !should_get("issues_and_prs") {
        tracing::info!("Skipping issue and pr data");
        return Ok(IssueAndPrStats::default());
    }
    tracing::info!("Getting issue and pr data");
    let resp =
        post_graphql::<IssuesAndPrsQuery, _>(client, API_URL, issues_and_prs_query::Variables {})
            .await?;
    tracing::debug!("{resp:#?}");

    if let Some(errors) = resp.errors {
        tracing::error!("Errors in issue and pr query: {:?}", errors);
        return Err(anyhow::anyhow!("issue and pr query returned errors"));
    }

    let data = resp.data.unwrap();
    Ok(IssueAndPrStats {
        issues_created: data.issues_created.issue_count,
        issues_closed: data.issues_closed.issue_count,
        prs_created: data.prs_created.issue_count,
        prs_merged: data.prs_merged.issue_count,
    })
}

fn repo_pairs_for_template(mine: &[&OneRepo], others: &[&OneRepo]) -> Vec<(String, String)> {
    mine.iter()
        .zip_longest(others)
        .map(|l| match l {
            EitherOrBoth::Both(a, b) => (a.string_for_template(), b.string_for_template()),
            EitherOrBoth::Left(a) => (a.string_for_template(), String::new()),
            EitherOrBoth::Right(b) => (String::new(), b.string_for_template()),
        })
        .collect()
}

fn language_pairs_for_template(
    top_recent_languages: Vec<LanguageStat<'_>>,
    top_all_time_languages: Vec<LanguageStat<'_>>,
) -> Vec<(String, String)> {
    top_recent_languages
        .into_iter()
        .zip_longest(top_all_time_languages)
        .map(|l| match l {
            EitherOrBoth::Both(a, b) => (a.string_for_template(), b.string_for_template()),
            EitherOrBoth::Left(a) => (a.string_for_template(), String::new()),
            EitherOrBoth::Right(b) => (String::new(), b.string_for_template()),
        })
        .collect()
}

async fn top_artists() -> Result<Vec<String>> {
    if !should_get("top_artists") {
        tracing::info!("Skipping top artists");
        return Ok(vec![]);
    }
    tracing::info!("Getting top artists from last.fm");
    let api_key = env::var("LAST_FM_API_KEY")
        .expect("You must set the LAST_FM_API_KEY env var when running this program");
    let mut client = lastfm_rs::Client::new(&api_key);
    let artists = client
        .top_artists("autarch")
        .await
        .within_period(lastfm_rs::user::top_artists::Period::SevenDays)
        .with_limit(10)
        .send()
        .await?
        .artists;
    tracing::debug!("{artists:#?}");
    Ok(artists
        .iter()
        .sorted_by_key(|a| a.name.to_lowercase())
        .map(|a| {
            if a.mbid.is_empty() {
                format!(
                    "[{}](https://musicbrainz.org/search?query={}&type=artist&method=indexed)",
                    a.name,
                    urlencoding::encode(&a.name),
                )
            } else {
                format!("[{}](https://musicbrainz.org/artist/{})", a.name, a.mbid)
            }
        })
        .collect())
}

fn should_get(what: &str) -> bool {
    let only = env::var("PROFILE_GENERATOR_ONLY").unwrap_or_default();
    if only.is_empty() {
        return true;
    }
    only == what
}

use graphql_client::GraphQLQuery;

pub async fn post_graphql<Q: GraphQLQuery, U: reqwest::IntoUrl + Clone>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<crate::Response<Q::ResponseData>> {
    let body = Q::build_query(variables);
    for i in 1..5 {
        let reqwest_response = client.post(url.clone()).json(&body).send().await?;

        for (k, v) in reqwest_response.headers() {
            tracing::debug!("Response header: {}: {:?}", k, v);
        }

        if let Some(gmt) = reqwest_response.headers().get("x-github-media-type") {
            if gmt.to_str().unwrap_or_default() == "github.v3; format=json" {
                tracing::debug!("Got GitHub v3 media type, will retry request (attempt #{i})");
                let body = reqwest_response.text().await;
                tracing::debug!("Response body: {:?}", body);
                continue;
            }
        }

        return reqwest_response.json().await.map_err(Into::into);
    }

    Err(anyhow::anyhow!(
        "Could not get valid response after 5 attempts"
    ))
}

const README_TEMPLATE: &str = r"
# Dave Rolsky

See the [houseabsolute organization](https://github.com/houseabsolute) for the
bulk of my code. Using a GitHub organization makes it easier to collaborate
with others.

This file was generated by the Rust code in
https://github.com/autarch/autarch. It was last generated at {generated_at}.

My (mostly technical) blog lives at https://blog.urth.org/.

## Recent Blog Posts

{{ for post in blog_posts }}- [{post.title}]({post.url}) - {post.date}
{{ endfor }}

## Repo Stats
- **{user_and_repo_stats.live_repos} original repos with commits to the default branch in the last two years**
- {user_and_repo_stats.total_repos} total repos
  - {user_and_repo_stats.forked_repos} are forks

This excludes archived, disabled, empty, and private repos.

## Recent Commits
| My Repos | Others |
|----------|--------|
{{ for pair in recent_commits -}}
| {pair.0}              | {pair.1}                |
{{ endfor }}

## Most Starred
{{ for repo in top_repos.most_starred -}}
- [{repo.full_name}]({repo.url}) - {repo.stargazer_count} stars
{{ endfor }}

## Most Forked
{{ for repo in top_repos.most_forked -}}
- [{repo.full_name}]({repo.url}) - {repo.fork_count} forks
{{ endfor }}

## GitHub Activity Stats
- {issue_and_pr_stats.prs_created} PRs created
  - of which {issue_and_pr_stats.prs_merged} were merged
- {issue_and_pr_stats.issues_created} issues created
  - of which {issue_and_pr_stats.issues_closed} have been closed

## Language Stats
| Past Two Years        | All Time                |
|-----------------------|-------------------------|
{{ for pair in top_languages -}}
| {pair.0}              | {pair.1}                |
{{ endfor }}

## Top Artists for the Past Week
{{ for artist in top_artists -}}
* {artist}
{{ endfor }}
";
