use std::fmt::Display;

use git2::{Branch, Commit, Repository};

#[derive(Debug)]
struct Timestamp(i64);
impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn find_head_branch(repo: &Repository) -> Option<Branch> {
    let mut branches = repo.branches(Some(git2::BranchType::Local)).expect("Finding branches");
    branches.find_map(|branch_result|
            match branch_result {
                Ok((branch, _branch_type)) => {
                    if branch.is_head() {
                        println!("Found HEAD branch: {}", branch.name().expect("Getting branch name").expect("Decoding branch name"));
                        Some(branch)
                    }
                    else {
                        println!("branch {} is not HEAD", branch.name().expect("Getting branch name").expect("Decoding branch name"));
                        None
                    }
                },
                Err(e) => {
                    println!("WARN: Failed to open branch: {e}");
                    None
                },
            }
    )
}

fn extract_commit_time(commit: &Commit) -> Timestamp {
    Timestamp(commit.time().seconds())
}

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let branch = find_head_branch(&repo).expect("Finding HEAD branch");
    let branch_reference = branch.into_reference();
    println!("Found reference {}", branch_reference.name().expect("Decoding reference name"));

    let head_commit = branch_reference.peel_to_commit().expect("Finding latest commit");
    let head_commit_time = extract_commit_time(&head_commit);
    println!("Found commit {} at {head_commit_time}", head_commit.id());

    let mut commit_times: Vec<Timestamp> = Vec::with_capacity(head_commit.parent_count());
    commit_times.push(head_commit_time);

    for parent in head_commit.parents() {
        let time = extract_commit_time(&parent);
        println!("Found commit {} at {time}", parent.id());
        commit_times.push(time);
    }

    println!("{:?}", commit_times);
}
