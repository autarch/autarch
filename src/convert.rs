use crate::github_queries::{organization_repos_query, user_repos_query};

// The repos portion of the user and organization responses contain identical
// fields but with different types. Converting the org types to user types
// makes it possible to have one fn for collecting stats instead of two.

impl From<organization_repos_query::ReposNodes> for user_repos_query::ReposNodes {
    fn from(repo: organization_repos_query::ReposNodes) -> Self {
        Self {
            created_at: repo.created_at,
            default_branch_ref: repo.default_branch_ref.map(std::convert::Into::into),
            fork_count: repo.fork_count,
            is_archived: repo.is_archived,
            is_disabled: repo.is_disabled,
            is_empty: repo.is_empty,
            is_fork: repo.is_fork,
            is_mirror: repo.is_mirror,
            is_private: repo.is_private,
            name_with_owner: repo.name_with_owner,
            languages: repo.languages.map(std::convert::Into::into),
            license_info: repo.license_info.map(std::convert::Into::into),
            owner: repo.owner.into(),
            parent: repo.parent.map(std::convert::Into::into),
            stargazer_count: repo.stargazer_count,
            url: repo.url,
        }
    }
}

impl From<organization_repos_query::ReposNodesDefaultBranchRef>
    for user_repos_query::ReposNodesDefaultBranchRef
{
    fn from(ref_: organization_repos_query::ReposNodesDefaultBranchRef) -> Self {
        Self {
            target: ref_.target.map(std::convert::Into::into),
        }
    }
}

impl From<organization_repos_query::ReposNodesDefaultBranchRefTarget>
    for user_repos_query::ReposNodesDefaultBranchRefTarget
{
    fn from(target: organization_repos_query::ReposNodesDefaultBranchRefTarget) -> Self {
        match target {
            organization_repos_query::ReposNodesDefaultBranchRefTarget::Blob => {
                user_repos_query::ReposNodesDefaultBranchRefTarget::Blob
            }
            organization_repos_query::ReposNodesDefaultBranchRefTarget::Commit(c) => {
                user_repos_query::ReposNodesDefaultBranchRefTarget::Commit(
                    user_repos_query::ReposNodesDefaultBranchRefTargetOnCommit {
                        history: c.history.into(),
                    },
                )
            }
            organization_repos_query::ReposNodesDefaultBranchRefTarget::Tag => {
                user_repos_query::ReposNodesDefaultBranchRefTarget::Tag
            }
            organization_repos_query::ReposNodesDefaultBranchRefTarget::Tree => {
                user_repos_query::ReposNodesDefaultBranchRefTarget::Tree
            }
        }
    }
}

impl From<organization_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistory>
    for user_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistory
{
    fn from(
        history: organization_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistory,
    ) -> Self {
        Self {
            nodes: history.nodes.map(|vn| {
                vn.into_iter()
                    .map(|n| n.map(std::convert::Into::into))
                    .collect()
            }),
        }
    }
}

impl From<organization_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes>
    for user_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes
{
    fn from(
        nodes: organization_repos_query::ReposNodesDefaultBranchRefTargetOnCommitHistoryNodes,
    ) -> Self {
        Self {
            committed_date: nodes.committed_date,
        }
    }
}

impl From<organization_repos_query::ReposNodesLanguages> for user_repos_query::ReposNodesLanguages {
    fn from(langs: organization_repos_query::ReposNodesLanguages) -> Self {
        Self {
            edges: langs.edges.map(|ve| {
                ve.into_iter()
                    .map(|e| e.map(std::convert::Into::into))
                    .collect()
            }),
            nodes: langs.nodes.map(|vn| {
                vn.into_iter()
                    .map(|n| n.map(std::convert::Into::into))
                    .collect()
            }),
            total_size: langs.total_size,
        }
    }
}

impl From<organization_repos_query::ReposNodesLanguagesEdges>
    for user_repos_query::ReposNodesLanguagesEdges
{
    fn from(edges: organization_repos_query::ReposNodesLanguagesEdges) -> Self {
        Self { size: edges.size }
    }
}

impl From<organization_repos_query::ReposNodesLanguagesNodes>
    for user_repos_query::ReposNodesLanguagesNodes
{
    fn from(nodes: organization_repos_query::ReposNodesLanguagesNodes) -> Self {
        Self {
            color: nodes.color,
            name: nodes.name,
        }
    }
}

impl From<organization_repos_query::ReposNodesLicenseInfo>
    for user_repos_query::ReposNodesLicenseInfo
{
    fn from(license: organization_repos_query::ReposNodesLicenseInfo) -> Self {
        Self {
            name: license.name,
            nickname: license.nickname,
            spdx_id: license.spdx_id,
        }
    }
}

impl From<organization_repos_query::ReposNodesOwner> for user_repos_query::ReposNodesOwner {
    fn from(owner: organization_repos_query::ReposNodesOwner) -> Self {
        Self {
            login: owner.login,
            on: owner.on.into(),
        }
    }
}

impl From<organization_repos_query::ReposNodesOwnerOn> for user_repos_query::ReposNodesOwnerOn {
    fn from(on: organization_repos_query::ReposNodesOwnerOn) -> Self {
        match on {
            organization_repos_query::ReposNodesOwnerOn::Organization => {
                user_repos_query::ReposNodesOwnerOn::Organization
            }
            organization_repos_query::ReposNodesOwnerOn::User => {
                user_repos_query::ReposNodesOwnerOn::User
            }
        }
    }
}

impl From<organization_repos_query::ReposNodesParent> for user_repos_query::ReposNodesParent {
    fn from(parent: organization_repos_query::ReposNodesParent) -> Self {
        Self {
            owner: parent.owner.into(),
        }
    }
}

impl From<organization_repos_query::ReposNodesParentOwner>
    for user_repos_query::ReposNodesParentOwner
{
    fn from(owner: organization_repos_query::ReposNodesParentOwner) -> Self {
        Self {
            login: owner.login,
            on: owner.on.into(),
        }
    }
}

impl From<organization_repos_query::ReposNodesParentOwnerOn>
    for user_repos_query::ReposNodesParentOwnerOn
{
    fn from(on: organization_repos_query::ReposNodesParentOwnerOn) -> Self {
        match on {
            organization_repos_query::ReposNodesParentOwnerOn::Organization => {
                user_repos_query::ReposNodesParentOwnerOn::Organization
            }
            organization_repos_query::ReposNodesParentOwnerOn::User => {
                user_repos_query::ReposNodesParentOwnerOn::User
            }
        }
    }
}
