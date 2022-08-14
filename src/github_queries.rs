#![allow(clippy::all, warnings)]
pub struct UserReposQuery;
pub mod user_repos_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserReposQuery";
    pub const QUERY : & str = "fragment repos on RepositoryConnection {\n  pageInfo {\n    endCursor\n    hasNextPage\n  }\n  nodes {\n    createdAt\n    defaultBranchRef {\n      target {\n        __typename\n        ... on Commit {\n          history(\n            author: {\n              emails: [$email],\n            },\n            first: 1,\n          ) {\n            nodes {\n              committedDate\n            }\n          }\n        }\n      }\n    }\n    forkCount\n    isArchived\n    isDisabled\n    isEmpty\n    isFork\n    isMirror\n    isPrivate\n    nameWithOwner\n    languages(first: 100) {\n      edges {\n        size\n      }\n      nodes {\n        color\n        name\n      }\n      totalSize\n    }\n    licenseInfo {\n      nickname\n      spdxId\n      name\n    }\n    owner {\n      __typename\n      login\n    }\n    stargazerCount\n    url\n  }\n}\n\nquery UserReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    createdAt\n    repositories(\n      affiliations: [COLLABORATOR, OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery OrganizationReposQuery($login: String!, $email: String!, $after: String) {\n  organization(login: $login) {\n    repositories(\n      affiliations: [OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery UserContributedReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    repositoriesContributedTo(\n      after: $after,\n      contributionTypes: [COMMIT],\n      first: 100,\n      includeUserRepositories: true,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      pageInfo {\n        endCursor\n        hasNextPage\n      }\n      nodes {\n        defaultBranchRef {\n          target {\n            __typename\n            ... on Commit {\n              history(\n                author: {\n                  emails: [$email],\n                },\n                first: 1,\n              ) {\n                nodes {\n                  committedDate\n                }\n              }\n            }\n          }\n        }\n        nameWithOwner\n        owner {\n          __typename\n          login\n        }\n        url\n      }\n    }\n  }\n}\n\n# Adapted from queries in\n# https://github.com/lowlighter/metrics/blob/master/source/plugins/followup/querie/s\nquery IssuesAndPrsQuery {\n  issues_created:search(query: \"author:autarch is:issue is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  issues_closed:search(query: \"author:autarch is:issue is:closed is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_created:search(query: \"author:autarch is:pr is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_merged:search(query: \"author:autarch is:pr is:merged is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = crate::gql_types::DateTime;
    type URI = crate::gql_types::URI;
    #[derive(Serialize)]
    pub struct Variables {
        pub login: String,
        pub email: String,
        pub after: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct repos {
        #[serde(rename = "pageInfo")]
        pub page_info: ReposPageInfo,
        pub nodes: Option<Vec<Option<ReposNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposPageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodes {
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "defaultBranchRef")]
        pub default_branch_ref: Option<ReposNodesDefaultBranchRef>,
        #[serde(rename = "forkCount")]
        pub fork_count: Int,
        #[serde(rename = "isArchived")]
        pub is_archived: Boolean,
        #[serde(rename = "isDisabled")]
        pub is_disabled: Boolean,
        #[serde(rename = "isEmpty")]
        pub is_empty: Boolean,
        #[serde(rename = "isFork")]
        pub is_fork: Boolean,
        #[serde(rename = "isMirror")]
        pub is_mirror: Boolean,
        #[serde(rename = "isPrivate")]
        pub is_private: Boolean,
        #[serde(rename = "nameWithOwner")]
        pub name_with_owner: String,
        pub languages: Option<ReposNodesLanguages>,
        #[serde(rename = "licenseInfo")]
        pub license_info: Option<ReposNodesLicenseInfo>,
        pub owner: ReposNodesOwner,
        #[serde(rename = "stargazerCount")]
        pub stargazer_count: Int,
        pub url: URI,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRef {
        pub target: Option<ReposNodesDefaultBranchRefTarget>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum ReposNodesDefaultBranchRefTarget {
        Blob,
        Commit(ReposNodesDefaultBranchRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommit {
        pub history: ReposNodesDefaultBranchRefTargetOnCommitHistory,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommitHistory {
        pub nodes: Option<Vec<Option<ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes {
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguages {
        pub edges: Option<Vec<Option<ReposNodesLanguagesEdges>>>,
        pub nodes: Option<Vec<Option<ReposNodesLanguagesNodes>>>,
        #[serde(rename = "totalSize")]
        pub total_size: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguagesEdges {
        pub size: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguagesNodes {
        pub color: Option<String>,
        pub name: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLicenseInfo {
        pub nickname: Option<String>,
        #[serde(rename = "spdxId")]
        pub spdx_id: Option<String>,
        pub name: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesOwner {
        pub login: String,
        #[serde(flatten)]
        pub on: ReposNodesOwnerOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum ReposNodesOwnerOn {
        Organization,
        User,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub user: Option<UserReposQueryUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserReposQueryUser {
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        pub repositories: UserReposQueryUserRepositories,
    }
    pub type UserReposQueryUserRepositories = repos;
}
impl graphql_client::GraphQLQuery for UserReposQuery {
    type Variables = user_repos_query::Variables;
    type ResponseData = user_repos_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_repos_query::QUERY,
            operation_name: user_repos_query::OPERATION_NAME,
        }
    }
}
pub struct OrganizationReposQuery;
pub mod organization_repos_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "OrganizationReposQuery";
    pub const QUERY : & str = "fragment repos on RepositoryConnection {\n  pageInfo {\n    endCursor\n    hasNextPage\n  }\n  nodes {\n    createdAt\n    defaultBranchRef {\n      target {\n        __typename\n        ... on Commit {\n          history(\n            author: {\n              emails: [$email],\n            },\n            first: 1,\n          ) {\n            nodes {\n              committedDate\n            }\n          }\n        }\n      }\n    }\n    forkCount\n    isArchived\n    isDisabled\n    isEmpty\n    isFork\n    isMirror\n    isPrivate\n    nameWithOwner\n    languages(first: 100) {\n      edges {\n        size\n      }\n      nodes {\n        color\n        name\n      }\n      totalSize\n    }\n    licenseInfo {\n      nickname\n      spdxId\n      name\n    }\n    owner {\n      __typename\n      login\n    }\n    stargazerCount\n    url\n  }\n}\n\nquery UserReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    createdAt\n    repositories(\n      affiliations: [COLLABORATOR, OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery OrganizationReposQuery($login: String!, $email: String!, $after: String) {\n  organization(login: $login) {\n    repositories(\n      affiliations: [OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery UserContributedReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    repositoriesContributedTo(\n      after: $after,\n      contributionTypes: [COMMIT],\n      first: 100,\n      includeUserRepositories: true,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      pageInfo {\n        endCursor\n        hasNextPage\n      }\n      nodes {\n        defaultBranchRef {\n          target {\n            __typename\n            ... on Commit {\n              history(\n                author: {\n                  emails: [$email],\n                },\n                first: 1,\n              ) {\n                nodes {\n                  committedDate\n                }\n              }\n            }\n          }\n        }\n        nameWithOwner\n        owner {\n          __typename\n          login\n        }\n        url\n      }\n    }\n  }\n}\n\n# Adapted from queries in\n# https://github.com/lowlighter/metrics/blob/master/source/plugins/followup/querie/s\nquery IssuesAndPrsQuery {\n  issues_created:search(query: \"author:autarch is:issue is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  issues_closed:search(query: \"author:autarch is:issue is:closed is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_created:search(query: \"author:autarch is:pr is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_merged:search(query: \"author:autarch is:pr is:merged is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = crate::gql_types::DateTime;
    type URI = crate::gql_types::URI;
    #[derive(Serialize)]
    pub struct Variables {
        pub login: String,
        pub email: String,
        pub after: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct repos {
        #[serde(rename = "pageInfo")]
        pub page_info: ReposPageInfo,
        pub nodes: Option<Vec<Option<ReposNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposPageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodes {
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "defaultBranchRef")]
        pub default_branch_ref: Option<ReposNodesDefaultBranchRef>,
        #[serde(rename = "forkCount")]
        pub fork_count: Int,
        #[serde(rename = "isArchived")]
        pub is_archived: Boolean,
        #[serde(rename = "isDisabled")]
        pub is_disabled: Boolean,
        #[serde(rename = "isEmpty")]
        pub is_empty: Boolean,
        #[serde(rename = "isFork")]
        pub is_fork: Boolean,
        #[serde(rename = "isMirror")]
        pub is_mirror: Boolean,
        #[serde(rename = "isPrivate")]
        pub is_private: Boolean,
        #[serde(rename = "nameWithOwner")]
        pub name_with_owner: String,
        pub languages: Option<ReposNodesLanguages>,
        #[serde(rename = "licenseInfo")]
        pub license_info: Option<ReposNodesLicenseInfo>,
        pub owner: ReposNodesOwner,
        #[serde(rename = "stargazerCount")]
        pub stargazer_count: Int,
        pub url: URI,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRef {
        pub target: Option<ReposNodesDefaultBranchRefTarget>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum ReposNodesDefaultBranchRefTarget {
        Blob,
        Commit(ReposNodesDefaultBranchRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommit {
        pub history: ReposNodesDefaultBranchRefTargetOnCommitHistory,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommitHistory {
        pub nodes: Option<Vec<Option<ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes {
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguages {
        pub edges: Option<Vec<Option<ReposNodesLanguagesEdges>>>,
        pub nodes: Option<Vec<Option<ReposNodesLanguagesNodes>>>,
        #[serde(rename = "totalSize")]
        pub total_size: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguagesEdges {
        pub size: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLanguagesNodes {
        pub color: Option<String>,
        pub name: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesLicenseInfo {
        pub nickname: Option<String>,
        #[serde(rename = "spdxId")]
        pub spdx_id: Option<String>,
        pub name: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ReposNodesOwner {
        pub login: String,
        #[serde(flatten)]
        pub on: ReposNodesOwnerOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum ReposNodesOwnerOn {
        Organization,
        User,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub organization: Option<OrganizationReposQueryOrganization>,
    }
    #[derive(Deserialize, Debug)]
    pub struct OrganizationReposQueryOrganization {
        pub repositories: OrganizationReposQueryOrganizationRepositories,
    }
    pub type OrganizationReposQueryOrganizationRepositories = repos;
}
impl graphql_client::GraphQLQuery for OrganizationReposQuery {
    type Variables = organization_repos_query::Variables;
    type ResponseData = organization_repos_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: organization_repos_query::QUERY,
            operation_name: organization_repos_query::OPERATION_NAME,
        }
    }
}
pub struct UserContributedReposQuery;
pub mod user_contributed_repos_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "UserContributedReposQuery";
    pub const QUERY : & str = "fragment repos on RepositoryConnection {\n  pageInfo {\n    endCursor\n    hasNextPage\n  }\n  nodes {\n    createdAt\n    defaultBranchRef {\n      target {\n        __typename\n        ... on Commit {\n          history(\n            author: {\n              emails: [$email],\n            },\n            first: 1,\n          ) {\n            nodes {\n              committedDate\n            }\n          }\n        }\n      }\n    }\n    forkCount\n    isArchived\n    isDisabled\n    isEmpty\n    isFork\n    isMirror\n    isPrivate\n    nameWithOwner\n    languages(first: 100) {\n      edges {\n        size\n      }\n      nodes {\n        color\n        name\n      }\n      totalSize\n    }\n    licenseInfo {\n      nickname\n      spdxId\n      name\n    }\n    owner {\n      __typename\n      login\n    }\n    stargazerCount\n    url\n  }\n}\n\nquery UserReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    createdAt\n    repositories(\n      affiliations: [COLLABORATOR, OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery OrganizationReposQuery($login: String!, $email: String!, $after: String) {\n  organization(login: $login) {\n    repositories(\n      affiliations: [OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery UserContributedReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    repositoriesContributedTo(\n      after: $after,\n      contributionTypes: [COMMIT],\n      first: 100,\n      includeUserRepositories: true,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      pageInfo {\n        endCursor\n        hasNextPage\n      }\n      nodes {\n        defaultBranchRef {\n          target {\n            __typename\n            ... on Commit {\n              history(\n                author: {\n                  emails: [$email],\n                },\n                first: 1,\n              ) {\n                nodes {\n                  committedDate\n                }\n              }\n            }\n          }\n        }\n        nameWithOwner\n        owner {\n          __typename\n          login\n        }\n        url\n      }\n    }\n  }\n}\n\n# Adapted from queries in\n# https://github.com/lowlighter/metrics/blob/master/source/plugins/followup/querie/s\nquery IssuesAndPrsQuery {\n  issues_created:search(query: \"author:autarch is:issue is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  issues_closed:search(query: \"author:autarch is:issue is:closed is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_created:search(query: \"author:autarch is:pr is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_merged:search(query: \"author:autarch is:pr is:merged is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = crate::gql_types::DateTime;
    type URI = crate::gql_types::URI;
    #[derive(Serialize)]
    pub struct Variables {
        pub login: String,
        pub email: String,
        pub after: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub user: Option<UserContributedReposQueryUser>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUser {
        #[serde(rename = "repositoriesContributedTo")]
        pub repositories_contributed_to: UserContributedReposQueryUserRepositoriesContributedTo,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedTo {
        #[serde(rename = "pageInfo")]
        pub page_info: UserContributedReposQueryUserRepositoriesContributedToPageInfo,
        pub nodes: Option<Vec<Option<UserContributedReposQueryUserRepositoriesContributedToNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToPageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodes {
        #[serde(rename = "defaultBranchRef")]
        pub default_branch_ref:
            Option<UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRef>,
        #[serde(rename = "nameWithOwner")]
        pub name_with_owner: String,
        pub owner: UserContributedReposQueryUserRepositoriesContributedToNodesOwner,
        pub url: URI,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRef {
        pub target: Option<
            UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTarget,
        >,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTarget {
        Blob , Commit (UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommit) , Tag , Tree }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommit { pub history : UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommitHistory , }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommitHistory { pub nodes : Option < Vec < Option < UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommitHistoryNodes >> > , }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodesDefaultBranchRefTargetOnCommitHistoryNodes
    {
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct UserContributedReposQueryUserRepositoriesContributedToNodesOwner {
        pub login: String,
        #[serde(flatten)]
        pub on: UserContributedReposQueryUserRepositoriesContributedToNodesOwnerOn,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum UserContributedReposQueryUserRepositoriesContributedToNodesOwnerOn {
        Organization,
        User,
    }
}
impl graphql_client::GraphQLQuery for UserContributedReposQuery {
    type Variables = user_contributed_repos_query::Variables;
    type ResponseData = user_contributed_repos_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: user_contributed_repos_query::QUERY,
            operation_name: user_contributed_repos_query::OPERATION_NAME,
        }
    }
}
pub struct IssuesAndPrsQuery;
pub mod issues_and_prs_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssuesAndPrsQuery";
    pub const QUERY : & str = "fragment repos on RepositoryConnection {\n  pageInfo {\n    endCursor\n    hasNextPage\n  }\n  nodes {\n    createdAt\n    defaultBranchRef {\n      target {\n        __typename\n        ... on Commit {\n          history(\n            author: {\n              emails: [$email],\n            },\n            first: 1,\n          ) {\n            nodes {\n              committedDate\n            }\n          }\n        }\n      }\n    }\n    forkCount\n    isArchived\n    isDisabled\n    isEmpty\n    isFork\n    isMirror\n    isPrivate\n    nameWithOwner\n    languages(first: 100) {\n      edges {\n        size\n      }\n      nodes {\n        color\n        name\n      }\n      totalSize\n    }\n    licenseInfo {\n      nickname\n      spdxId\n      name\n    }\n    owner {\n      __typename\n      login\n    }\n    stargazerCount\n    url\n  }\n}\n\nquery UserReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    createdAt\n    repositories(\n      affiliations: [COLLABORATOR, OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery OrganizationReposQuery($login: String!, $email: String!, $after: String) {\n  organization(login: $login) {\n    repositories(\n      affiliations: [OWNER],\n      after: $after,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      ...repos\n    }\n  }\n}\n\nquery UserContributedReposQuery($login: String!, $email: String!, $after: String) {\n  user(login: $login) {\n    repositoriesContributedTo(\n      after: $after,\n      contributionTypes: [COMMIT],\n      first: 100,\n      includeUserRepositories: true,\n      orderBy: {\n        direction: ASC,\n        field: NAME,\n      },\n      privacy: PUBLIC,\n    ) {\n      pageInfo {\n        endCursor\n        hasNextPage\n      }\n      nodes {\n        defaultBranchRef {\n          target {\n            __typename\n            ... on Commit {\n              history(\n                author: {\n                  emails: [$email],\n                },\n                first: 1,\n              ) {\n                nodes {\n                  committedDate\n                }\n              }\n            }\n          }\n        }\n        nameWithOwner\n        owner {\n          __typename\n          login\n        }\n        url\n      }\n    }\n  }\n}\n\n# Adapted from queries in\n# https://github.com/lowlighter/metrics/blob/master/source/plugins/followup/querie/s\nquery IssuesAndPrsQuery {\n  issues_created:search(query: \"author:autarch is:issue is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  issues_closed:search(query: \"author:autarch is:issue is:closed is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_created:search(query: \"author:autarch is:pr is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n  prs_merged:search(query: \"author:autarch is:pr is:merged is:public -user:10gen -user:mongodb\", type: ISSUE, first: 0) {\n    issueCount\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub issues_created: IssuesAndPrsQueryIssuesCreated,
        pub issues_closed: IssuesAndPrsQueryIssuesClosed,
        pub prs_created: IssuesAndPrsQueryPrsCreated,
        pub prs_merged: IssuesAndPrsQueryPrsMerged,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesAndPrsQueryIssuesCreated {
        #[serde(rename = "issueCount")]
        pub issue_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesAndPrsQueryIssuesClosed {
        #[serde(rename = "issueCount")]
        pub issue_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesAndPrsQueryPrsCreated {
        #[serde(rename = "issueCount")]
        pub issue_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesAndPrsQueryPrsMerged {
        #[serde(rename = "issueCount")]
        pub issue_count: Int,
    }
}
impl graphql_client::GraphQLQuery for IssuesAndPrsQuery {
    type Variables = issues_and_prs_query::Variables;
    type ResponseData = issues_and_prs_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issues_and_prs_query::QUERY,
            operation_name: issues_and_prs_query::OPERATION_NAME,
        }
    }
}
