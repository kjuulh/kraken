use std::{path::PathBuf, sync::Arc};

use eyre::ContextCompat;
use git2::{Cred, FetchOptions, PushOptions, RemoteCallbacks, Repository};
use tokio::sync::Mutex;

use crate::storage::DynStorageEngine;

use super::GitProvider;

#[derive(Clone, Debug)]
pub struct LocalGitProviderOptions {
    pub http_auth: Option<String>,
}

pub struct LocalGitProvider {
    storage_engine: DynStorageEngine,
    options: LocalGitProviderOptions,
}

impl LocalGitProvider {
    pub fn new(options: LocalGitProviderOptions, storage_engine: DynStorageEngine) -> Self {
        Self {
            storage_engine,
            options,
        }
    }

    fn fast_forward(
        repo: &Repository,
        lb: &mut git2::Reference,
        rc: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let name = match lb.name() {
            Some(s) => s.to_string(),
            None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
        };
        let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
        println!("{}", msg);
        lb.set_target(rc.id(), &msg)?;
        repo.set_head(&name)?;
        repo.checkout_head(Some(
            git2::build::CheckoutBuilder::default()
                // For some reason the force is required to make the working directory actually get updated
                // I suspect we should be adding some logic to handle dirty working directory states
                // but this is just an example so maybe not.
                .force(),
        ))?;
        Ok(())
    }

    fn normal_merge(
        repo: &Repository,
        local: &git2::AnnotatedCommit,
        remote: &git2::AnnotatedCommit,
    ) -> Result<(), git2::Error> {
        let local_tree = repo.find_commit(local.id())?.tree()?;
        let remote_tree = repo.find_commit(remote.id())?.tree()?;
        let ancestor = repo
            .find_commit(repo.merge_base(local.id(), remote.id())?)?
            .tree()?;
        let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

        if idx.has_conflicts() {
            println!("Merge conficts detected...");
            repo.checkout_index(Some(&mut idx), None)?;
            return Ok(());
        }
        let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
        // now create the merge commit
        let msg = format!("Merge: {} into {}", remote.id(), local.id());
        let sig = repo.signature()?;
        let local_commit = repo.find_commit(local.id())?;
        let remote_commit = repo.find_commit(remote.id())?;
        // Do our merge commit and set current branch head to that commit.
        let _merge_commit = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )?;
        // Set working tree to match head.
        repo.checkout_head(None)?;
        Ok(())
    }

    fn do_merge<'a>(
        repo: &'a Repository,
        remote_branch: &str,
        fetch_commit: git2::AnnotatedCommit<'a>,
    ) -> Result<(), git2::Error> {
        // 1. do a merge analysis
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        // 2. Do the appopriate merge
        if analysis.0.is_fast_forward() {
            println!("Doing a fast forward");
            // do a fast forward
            let refname = format!("refs/heads/{}", remote_branch);
            match repo.find_reference(&refname) {
                Ok(mut r) => {
                    Self::fast_forward(repo, &mut r, &fetch_commit)?;
                }
                Err(_) => {
                    // The branch doesn't exist so just set the reference to the
                    // commit directly. Usually this is because you are pulling
                    // into an empty repository.
                    repo.reference(
                        &refname,
                        fetch_commit.id(),
                        true,
                        &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                    )?;
                    repo.set_head(&refname)?;
                    repo.checkout_head(Some(
                        git2::build::CheckoutBuilder::default()
                            .allow_conflicts(true)
                            .conflict_style_merge(true)
                            .force(),
                    ))?;
                }
            };
        } else if analysis.0.is_normal() {
            // do a normal merge
            let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
            Self::normal_merge(&repo, &head_commit, &fetch_commit)?;
        } else {
            println!("Nothing to do...");
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl GitProvider for LocalGitProvider {
    async fn clone_from_url(&self, url: &String) -> eyre::Result<(PathBuf, Repository)> {
        let url = url.clone();
        tracing::debug!(url, "allocating dir");
        let dir = self.storage_engine.allocate_dir().await?;
        let options = self.options.clone();

        let dirpath = dir.clone().path();
        let repo = tokio::task::spawn_blocking(move || {
            let mut callbacks = RemoteCallbacks::new();
            callbacks.credentials(|url, username_from_url, _allowed_types| {
                tracing::debug!(username_from_url, url, "pulling key from ssh-agent");

                if let Some(auth) = &options.http_auth {
                    tracing::trace!(auth, "authenticating");
                    let (user, pass) = auth
                        .split_once(":")
                        .ok_or("http_auth is not formatted correctly")
                        .unwrap();

                    Cred::userpass_plaintext(user, pass)
                } else {
                    let username = username_from_url
                        .context("could not find username_from_url")
                        .unwrap();
                    Cred::ssh_key_from_agent(username)
                }
            });

            let mut fo = git2::FetchOptions::new();
            fo.remote_callbacks(callbacks);

            let checkout_builder = git2::build::CheckoutBuilder::new();

            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fo).with_checkout(checkout_builder);

            tracing::debug!(
                path = dirpath.as_os_str().to_string_lossy().to_string(),
                "clone git repo"
            );
            builder.clone(url.as_str(), dirpath.as_path())
        })
        .await??;

        tracing::debug!("done pulling repo");

        Ok((dir.path(), repo))
    }

    async fn create_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        branch_name: &String,
    ) -> eyre::Result<()> {
        let repo = repo.lock().await;
        let branch_name = branch_name.to_lowercase().replace(" ", "-");

        let head_commit_oid = repo
            .head()?
            .target()
            .ok_or(eyre::anyhow!("could not get access to target commit"))?;
        let head_commit = repo.find_commit(head_commit_oid)?;
        let newbranch = repo.branch(&branch_name, &head_commit, true)?;

        repo.set_head(
            newbranch
                .into_reference()
                .name()
                .ok_or(eyre::anyhow!("could not get name of reference"))?,
        )?;

        tracing::trace!("pulling from origin");
        let options = self.options.clone();
        let remote = "origin";
        let mut cb = RemoteCallbacks::new();
        cb.credentials(|url, username_from_url, _allowed_types| {
            tracing::debug!(username_from_url, url, "pulling key from ssh-agent");

            if let Some(auth) = &options.http_auth {
                tracing::trace!(auth, "authenticating");
                let (user, pass) = auth
                    .split_once(":")
                    .ok_or("http_auth is not formatted correctly")
                    .unwrap();

                Cred::userpass_plaintext(user, pass)
            } else {
                let username = username_from_url.unwrap();
                Cred::ssh_key_from_agent(username)
            }
        });
        let mut remote = repo
            .find_remote(remote)
            .or_else(|_| repo.remote_anonymous(remote))?;

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        let head = repo.head()?;
        let refspec = &[head
            .name()
            .ok_or(eyre::anyhow!("could not find head.name"))?];

        remote.fetch(refspec, Some(&mut fo), None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let commit = repo.reference_to_annotated_commit(&fetch_head)?;
        Self::do_merge(&repo, &branch_name, commit)?;

        Ok(())
    }

    async fn push_branch(
        &self,
        repo: Arc<Mutex<Repository>>,
        branch_name: &String,
    ) -> eyre::Result<()> {
        let repo = repo.lock().await;
        let options = self.options.clone();

        tracing::trace!("pulling signature from local git");
        let signature = repo.signature()?;

        tracing::trace!("fetching index and adding changed files to working tree");
        let mut index = repo.index()?;
        index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        let tree = index.write_tree()?;
        let tree = repo.find_tree(tree)?;

        let parents = repo.head().map(|h| {
            h.target()
                .ok_or(eyre::anyhow!("could not fetch target"))
                .map(|t| repo.find_commit(t))
        })???;

        tracing::trace!("writing commit object");
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            branch_name.to_lowercase().replace(" ", "-").as_str(),
            &tree,
            &[&parents],
        )?;

        let mut remote = repo.find_remote("origin")?;
        let head = repo.head()?;
        let refspec = &[head
            .name()
            .ok_or(eyre::anyhow!("could not find head.name"))?];

        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(|url, username_from_url, _allowed_types| {
            tracing::debug!(username_from_url, url, "pulling key from ssh-agent");

            if let Some(auth) = &options.http_auth {
                tracing::trace!(auth, "authenticating");
                let (user, pass) = auth
                    .split_once(":")
                    .ok_or("http_auth is not formatted correctly")
                    .unwrap();

                Cred::userpass_plaintext(user, pass)
            } else {
                let username = username_from_url.unwrap();
                Cred::ssh_key_from_agent(username)
            }
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(remote_callbacks);

        tracing::trace!("pushing to remote");
        remote.push(refspec, Some(&mut push_options))?;

        Ok(())
    }
}
