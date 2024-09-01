use git2::{Branch, Repository, TreeWalkResult};

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

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let branch = find_head_branch(&repo).expect("Finding HEAD branch");
    let branch_reference = branch.into_reference();
    println!("Found reference {}", branch_reference.name().expect("Decoding reference name"));
    let head_commit = branch_reference.peel_to_commit().expect("Finding latest commit");
    println!("Found commit {}", head_commit.id());
    for parent in head_commit.parents() {
        println!("Found parent {}", parent.id());
    }
}
