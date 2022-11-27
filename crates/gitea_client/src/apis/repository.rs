use std::sync::Arc;

use async_trait::async_trait;
use gitea_raw_client::{apis::Error, models};

pub use gitea_raw_client::apis::repository_api::*;

#[async_trait]
pub trait Repository {
    async fn accept_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<models::Repository, Error<AcceptRepoTransferError>>;
    async fn create_current_user_repo(
        &self,
        body: Option<models::CreateRepoOption>,
    ) -> Result<models::Repository, Error<CreateCurrentUserRepoError>>;
    async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateForkOption>,
    ) -> Result<models::Repository, Error<CreateForkError>>;
    async fn generate_repo(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: Option<models::GenerateRepoOption>,
    ) -> Result<models::Repository, Error<GenerateRepoError>>;
    async fn get_annotated_tag(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<models::AnnotatedTag, Error<GetAnnotatedTagError>>;
    async fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<models::GitBlobResponse, Error<GetBlobError>>;
    async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        recursive: Option<bool>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<models::GitTreeResponse, Error<GetTreeError>>;
    async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Repository>, Error<ListForksError>>;
    async fn reject_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<models::Repository, Error<RejectRepoTransferError>>;
    async fn add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
        body: Option<models::AddCollaboratorOption>,
    ) -> Result<(), Error<RepoAddCollaboratorError>>;
    async fn add_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<(), Error<RepoAddTeamError>>;
    async fn add_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> Result<(), Error<RepoAddTopicError>>;
    async fn apply_diff_patch(
        &self,
        owner: &str,
        repo: &str,
        body: models::UpdateFileOptions,
    ) -> Result<models::FileResponse, Error<RepoApplyDiffPatchError>>;
    async fn cancel_scheduled_auto_merge(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> Result<(), Error<RepoCancelScheduledAutoMergeError>>;
    async fn check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<(), Error<RepoCheckCollaboratorError>>;
    async fn check_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<models::Team, Error<RepoCheckTeamError>>;
    async fn create_branch(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateBranchRepoOption>,
    ) -> Result<models::Branch, Error<RepoCreateBranchError>>;
    async fn create_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateBranchProtectionOption>,
    ) -> Result<models::BranchProtection, Error<RepoCreateBranchProtectionError>>;
    async fn create_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: models::CreateFileOptions,
    ) -> Result<models::FileResponse, Error<RepoCreateFileError>>;
    async fn create_hook(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateHookOption>,
    ) -> Result<models::Hook, Error<RepoCreateHookError>>;
    async fn create_key(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateKeyOption>,
    ) -> Result<models::DeployKey, Error<RepoCreateKeyError>>;
    async fn create_pull_request(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreatePullRequestOption>,
    ) -> Result<models::PullRequest, Error<RepoCreatePullRequestError>>;
    async fn create_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: models::CreatePullReviewOptions,
    ) -> Result<models::PullReview, Error<RepoCreatePullReviewError>>;
    async fn create_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: models::PullReviewRequestOptions,
    ) -> Result<Vec<models::PullReview>, Error<RepoCreatePullReviewRequestsError>>;
    async fn create_release(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateReleaseOption>,
    ) -> Result<models::Release, Error<RepoCreateReleaseError>>;
    async fn create_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        attachment: std::path::PathBuf,
        name: Option<&str>,
    ) -> Result<models::Attachment, Error<RepoCreateReleaseAttachmentError>>;
    async fn create_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: Option<models::CreateStatusOption>,
    ) -> Result<models::CommitStatus, Error<RepoCreateStatusError>>;
    async fn create_tag(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateTagOption>,
    ) -> Result<models::Tag, Error<RepoCreateTagError>>;
    async fn create_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::CreateWikiPageOptions>,
    ) -> Result<models::WikiPage, Error<RepoCreateWikiPageError>>;
    async fn delete(&self, owner: &str, repo: &str) -> Result<(), Error<RepoDeleteError>>;
    async fn delete_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<(), Error<RepoDeleteBranchError>>;
    async fn delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<(), Error<RepoDeleteBranchProtectionError>>;
    async fn delete_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<(), Error<RepoDeleteCollaboratorError>>;
    async fn delete_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: models::DeleteFileOptions,
    ) -> Result<models::FileDeleteResponse, Error<RepoDeleteFileError>>;
    async fn delete_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<(), Error<RepoDeleteGitHookError>>;
    async fn delete_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<(), Error<RepoDeleteHookError>>;
    async fn delete_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<(), Error<RepoDeleteKeyError>>;
    async fn delete_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> Result<(), Error<RepoDeletePullReviewError>>;
    async fn delete_pull_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: models::PullReviewRequestOptions,
    ) -> Result<(), Error<RepoDeletePullReviewRequestsError>>;
    async fn delete_release(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<(), Error<RepoDeleteReleaseError>>;
    async fn delete_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        attachment_id: i64,
    ) -> Result<(), Error<RepoDeleteReleaseAttachmentError>>;
    async fn delete_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), Error<RepoDeleteReleaseByTagError>>;
    async fn delete_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<(), Error<RepoDeleteTagError>>;
    async fn delete_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> Result<(), Error<RepoDeleteTeamError>>;
    async fn delete_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> Result<(), Error<RepoDeleteTopicError>>;
    async fn delete_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> Result<(), Error<RepoDeleteWikiPageError>>;
    async fn dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        body: models::DismissPullReviewOptions,
    ) -> Result<models::PullReview, Error<RepoDismissPullReviewError>>;
    async fn download_commit_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        diff_type: &str,
    ) -> Result<String, Error<RepoDownloadCommitDiffOrPatchError>>;
    async fn download_pull_diff_or_patch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        diff_type: &str,
        binary: Option<bool>,
    ) -> Result<String, Error<RepoDownloadPullDiffOrPatchError>>;
    async fn edit(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::EditRepoOption>,
    ) -> Result<models::Repository, Error<RepoEditError>>;
    async fn edit_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: Option<models::EditBranchProtectionOption>,
    ) -> Result<models::BranchProtection, Error<RepoEditBranchProtectionError>>;
    async fn edit_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        body: Option<models::EditGitHookOption>,
    ) -> Result<models::GitHook, Error<RepoEditGitHookError>>;
    async fn edit_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        body: Option<models::EditHookOption>,
    ) -> Result<models::Hook, Error<RepoEditHookError>>;
    async fn edit_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: Option<models::EditPullRequestOption>,
    ) -> Result<models::PullRequest, Error<RepoEditPullRequestError>>;
    async fn edit_release(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        body: Option<models::EditReleaseOption>,
    ) -> Result<models::Release, Error<RepoEditReleaseError>>;
    async fn edit_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        attachment_id: i64,
        body: Option<models::EditAttachmentOptions>,
    ) -> Result<models::Attachment, Error<RepoEditReleaseAttachmentError>>;
    async fn edit_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
        body: Option<models::CreateWikiPageOptions>,
    ) -> Result<models::WikiPage, Error<RepoEditWikiPageError>>;
    async fn get(&self, owner: &str, repo: &str)
        -> Result<models::Repository, Error<RepoGetError>>;
    async fn get_all_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: Option<&str>,
        path: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Commit>, Error<RepoGetAllCommitsError>>;
    async fn get_archive(
        &self,
        owner: &str,
        repo: &str,
        archive: &str,
    ) -> Result<(), Error<RepoGetArchiveError>>;
    async fn get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::User>, Error<RepoGetAssigneesError>>;
    async fn get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> Result<models::Branch, Error<RepoGetBranchError>>;
    async fn get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> Result<models::BranchProtection, Error<RepoGetBranchProtectionError>>;
    async fn get_by_id(&self, id: i64) -> Result<models::Repository, Error<RepoGetByIdError>>;
    async fn get_combined_status_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<models::CombinedStatus, Error<RepoGetCombinedStatusByRefError>>;
    async fn get_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        r#ref: Option<&str>,
    ) -> Result<models::ContentsResponse, Error<RepoGetContentsError>>;
    async fn get_contents_list(
        &self,
        owner: &str,
        repo: &str,
        r#ref: Option<&str>,
    ) -> Result<Vec<models::ContentsResponse>, Error<RepoGetContentsListError>>;
    async fn get_editor_config(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        r#ref: Option<&str>,
    ) -> Result<(), Error<RepoGetEditorConfigError>>;
    async fn get_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> Result<models::GitHook, Error<RepoGetGitHookError>>;
    async fn get_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<models::Hook, Error<RepoGetHookError>>;
    async fn get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::IssueTemplate>, Error<RepoGetIssueTemplatesError>>;
    async fn get_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<models::DeployKey, Error<RepoGetKeyError>>;
    async fn get_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<::std::collections::HashMap<String, i64>, Error<RepoGetLanguagesError>>;
    async fn get_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<models::Note, Error<RepoGetNoteError>>;
    async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> Result<models::PullRequest, Error<RepoGetPullRequestError>>;
    async fn get_pull_request_commits(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Commit>, Error<RepoGetPullRequestCommitsError>>;
    async fn get_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> Result<models::PullReview, Error<RepoGetPullReviewError>>;
    async fn get_pull_review_comments(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> Result<Vec<models::PullReviewComment>, Error<RepoGetPullReviewCommentsError>>;
    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        r#ref: Option<&str>,
    ) -> Result<(), Error<RepoGetRawFileError>>;
    async fn get_raw_file_or_lfs(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        r#ref: Option<&str>,
    ) -> Result<(), Error<RepoGetRawFileOrLfsError>>;
    async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<models::Release, Error<RepoGetReleaseError>>;
    async fn get_release_attachment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        attachment_id: i64,
    ) -> Result<models::Attachment, Error<RepoGetReleaseAttachmentError>>;
    async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<models::Release, Error<RepoGetReleaseByTagError>>;
    async fn get_repo_permissions(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> Result<models::RepoCollaboratorPermission, Error<RepoGetRepoPermissionsError>>;
    async fn get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::User>, Error<RepoGetReviewersError>>;
    async fn get_single_commit(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<models::Commit, Error<RepoGetSingleCommitError>>;
    async fn get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<models::Tag, Error<RepoGetTagError>>;
    async fn get_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
    ) -> Result<models::WikiPage, Error<RepoGetWikiPageError>>;
    async fn get_wiki_page_revisions(
        &self,
        owner: &str,
        repo: &str,
        page_name: &str,
        page: Option<i32>,
    ) -> Result<models::WikiCommitList, Error<RepoGetWikiPageRevisionsError>>;
    async fn get_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::WikiPageMetaData>, Error<RepoGetWikiPagesError>>;
    async fn list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::Reference>, Error<RepoListAllGitRefsError>>;
    async fn list_branch_protection(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::BranchProtection>, Error<RepoListBranchProtectionError>>;
    async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Branch>, Error<RepoListBranchesError>>;
    async fn list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::User>, Error<RepoListCollaboratorsError>>;
    async fn list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::GitHook>, Error<RepoListGitHooksError>>;
    async fn list_git_refs(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
    ) -> Result<Vec<models::Reference>, Error<RepoListGitRefsError>>;
    async fn list_hooks(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Hook>, Error<RepoListHooksError>>;
    async fn list_keys(
        &self,
        owner: &str,
        repo: &str,
        key_id: Option<i32>,
        fingerprint: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::DeployKey>, Error<RepoListKeysError>>;
    async fn list_pull_requests(
        &self,
        owner: &str,
        repo: &str,
        state: Option<&str>,
        sort: Option<&str>,
        milestone: Option<i64>,
        labels: Option<Vec<i64>>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::PullRequest>, Error<RepoListPullRequestsError>>;
    async fn list_pull_reviews(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::PullReview>, Error<RepoListPullReviewsError>>;
    async fn list_release_attachments(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> Result<Vec<models::Attachment>, Error<RepoListReleaseAttachmentsError>>;
    async fn list_releases(
        &self,
        owner: &str,
        repo: &str,
        draft: Option<bool>,
        pre_release: Option<bool>,
        per_page: Option<i32>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Release>, Error<RepoListReleasesError>>;
    async fn list_stargazers(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::User>, Error<RepoListStargazersError>>;
    async fn list_statuses(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        sort: Option<&str>,
        state: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::CommitStatus>, Error<RepoListStatusesError>>;
    async fn list_statuses_by_ref(
        &self,
        owner: &str,
        repo: &str,
        r#ref: &str,
        sort: Option<&str>,
        state: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::CommitStatus>, Error<RepoListStatusesByRefError>>;
    async fn list_subscribers(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::User>, Error<RepoListSubscribersError>>;
    async fn list_tags(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::Tag>, Error<RepoListTagsError>>;
    async fn list_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<models::Team>, Error<RepoListTeamsError>>;
    async fn list_topics(
        &self,
        owner: &str,
        repo: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<models::TopicName, Error<RepoListTopicsError>>;
    async fn merge_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        body: Option<models::MergePullRequestOption>,
    ) -> Result<(), Error<RepoMergePullRequestError>>;
    async fn migrate(
        &self,
        body: Option<models::MigrateRepoOptions>,
    ) -> Result<models::Repository, Error<RepoMigrateError>>;
    async fn mirror_sync(&self, owner: &str, repo: &str) -> Result<(), Error<RepoMirrorSyncError>>;
    async fn pull_request_is_merged(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> Result<(), Error<RepoPullRequestIsMergedError>>;
    async fn search(
        &self,
        q: Option<&str>,
        topic: Option<bool>,
        include_desc: Option<bool>,
        uid: Option<i64>,
        priority_owner_id: Option<i64>,
        team_id: Option<i64>,
        starred_by: Option<i64>,
        private: Option<bool>,
        is_private: Option<bool>,
        template: Option<bool>,
        archived: Option<bool>,
        mode: Option<&str>,
        exclusive: Option<bool>,
        sort: Option<&str>,
        order: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<models::SearchResults, Error<RepoSearchError>>;
    async fn signing_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<String, Error<RepoSigningKeyError>>;
    async fn submit_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        body: models::SubmitPullReviewOptions,
    ) -> Result<models::PullReview, Error<RepoSubmitPullReviewError>>;
    async fn test_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        r#ref: Option<&str>,
    ) -> Result<(), Error<RepoTestHookError>>;
    async fn tracked_times(
        &self,
        owner: &str,
        repo: &str,
        user: Option<&str>,
        since: Option<String>,
        before: Option<String>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::TrackedTime>, Error<RepoTrackedTimesError>>;
    async fn transfer(
        &self,
        owner: &str,
        repo: &str,
        body: models::TransferRepoOption,
    ) -> Result<models::Repository, Error<RepoTransferError>>;
    async fn un_dismiss_pull_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> Result<models::PullReview, Error<RepoUnDismissPullReviewError>>;
    async fn update_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        body: models::UpdateFileOptions,
    ) -> Result<models::FileResponse, Error<RepoUpdateFileError>>;
    async fn update_pull_request(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        style: Option<&str>,
    ) -> Result<(), Error<RepoUpdatePullRequestError>>;
    async fn update_topics(
        &self,
        owner: &str,
        repo: &str,
        body: Option<models::RepoTopicOptions>,
    ) -> Result<(), Error<RepoUpdateTopicsError>>;
    async fn topic_search(
        &self,
        q: &str,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<models::TopicResponse>, Error<TopicSearchError>>;
    async fn user_current_check_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<models::WatchInfo, Error<UserCurrentCheckSubscriptionError>>;
    async fn user_current_delete_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<(), Error<UserCurrentDeleteSubscriptionError>>;
    async fn user_current_put_subscription(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<models::WatchInfo, Error<UserCurrentPutSubscriptionError>>;
    async fn user_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        user: &str,
    ) -> Result<Vec<models::TrackedTime>, Error<UserTrackedTimesError>>;
}

pub type DynRepository = Arc<dyn Repository + Send + Sync>;
