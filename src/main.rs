use std::{collections::BTreeSet, fmt::Display};
use git2::{Branch, Commit, Repository};

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let branch = find_head_branch(&repo).expect("Finding HEAD branch");
    let commit_times = get_branch_commit_times(branch);

    let estimate = estimate_hours(commit_times);
    println!("\nESTIMATE");
    println!("   {estimate}");
    println!(" ≈ {} hours", estimate.round());
}

#[derive(Debug, PartialEq, Eq)]
struct CommitWithTimestamp(git2::Oid, i64);
impl Display for CommitWithTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}
impl Ord for CommitWithTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}
impl PartialOrd for CommitWithTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const MAX_COMMIT_DIFF_MINUTES: f64 = 120.0;
const FIRST_COMMIT_ADDITION_MINUTES: f64 = 120.0;
fn estimate_hours(mut dates: Vec<CommitWithTimestamp>) -> f64 {
    let dates_total = dates.len();
    if dates_total < 2 {
        return 0.0;
    }

    dates.reverse();

    let hours = dates.iter().enumerate().map(|(index, date)| {
        if index == dates_total - 1 {
            return 0f64;
        }

        let next_date = &dates[index + 1];
        let diff_minutes = (next_date.1 - date.1) as f64 / 60.0;

        if diff_minutes < MAX_COMMIT_DIFF_MINUTES {
            return diff_minutes / 60.0;
        }
        
        FIRST_COMMIT_ADDITION_MINUTES / 60.0
    }).reduce(|acc, e| acc + e).unwrap_or_default();

    hours
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

fn extract_commit_time(commit: &Commit) -> CommitWithTimestamp {
    CommitWithTimestamp(commit.id(), commit.time().seconds())
}

fn get_commit_parent_times(commit: Commit) -> BTreeSet<CommitWithTimestamp> {
    let mut parents = BTreeSet::new();
    for parent in commit.parents() {
        let commit_info = extract_commit_time(&parent);
        if parents.contains(&commit_info) {
            continue;
        }
        parents.insert(commit_info);
        for gparent in get_commit_parent_times(parent) {
            parents.insert(gparent);
        }
    }
    parents
}

fn get_branch_commit_times(branch: Branch) -> Vec<CommitWithTimestamp> {
    let branch_reference = branch.into_reference();
    println!("Found reference {}", branch_reference.name().expect("Decoding reference name"));

    let head_commit = branch_reference.peel_to_commit().expect("Finding latest commit");
    let head_commit_time = extract_commit_time(&head_commit);
    println!("Found commit {} at {head_commit_time}", head_commit.id());

    let mut commit_times: Vec<CommitWithTimestamp> = Vec::with_capacity(head_commit.parent_count());
    commit_times.push(head_commit_time);

    for commit in get_commit_parent_times(head_commit) {
        let hash = commit.0;
        let time = commit.1;
        println!("Found commit {hash} at {time}");
        commit_times.push(commit);
    }

    commit_times
}

