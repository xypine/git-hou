use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let mut branches = repo.branches(Some(git2::BranchType::Local)).expect("Finding branches");
    let default_branch = branches.find_map(|branch_result|
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
    ).expect("Finding HEAD branch");
}
