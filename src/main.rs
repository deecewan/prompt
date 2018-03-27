extern crate colored;
extern crate git2;
extern crate time;

use std::{env, str};
use colored::*;
use git2::{ObjectType, Reference, ReferenceType, Repository, Status, StatusOptions};

const INSERT_SYMBOL: &str = "❯";
const INSERT_MODE: &str = "[INSERT]";
const NORMAL_MODE: &str = "[NORMAL]";

fn check_dirty_state(repo: &Repository) -> bool {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = match repo.statuses(Some(&mut status_options)) {
        Ok(s) => s,
        _ => return false,
    };

    statuses.iter().any(|x| match x.status() {
        Status::CURRENT => false,
        Status::IGNORED => false,
        Status::CONFLICTED => false,
        _ => true,
    })
}

fn process_non_standard_head(repo: &Repository, head: &Reference) -> String {
    let detached = repo.head_detached().unwrap_or(false);
    let name = match head.kind().unwrap() {
        ReferenceType::Oid => head.peel(ObjectType::Any)
            .and_then(|obj| obj.short_id())
            .and_then(|short_id| match short_id.as_str() {
                Some(id) => Ok(String::from(id)),
                None => Ok(String::from("Unknown")),
            })
            .unwrap_or(String::from("Unknown")),
        ReferenceType::Symbolic => String::from(head.symbolic_target().unwrap()),
    };

    if detached {
        format!("(detached {})", name)
    } else {
        name
    }
}

fn get_branch(repo: &Repository) -> String {
    match repo.head() {
        Ok(head) => match head.is_branch() {
            true => String::from(head.shorthand().unwrap()),
            false => process_non_standard_head(&repo, &head),
        },
        // this should fall back to looking at .git/HEAD
        Err(err) => {
            let unborn = match err.code() {
                git2::ErrorCode::UnbornBranch => true,
                _ => false,
            };
            if unborn {
                // this branch is present but doesn't exist
                // almost certainly going to be from empty repo (i hope)
                err.message()
                    .replace(r"reference 'refs/heads/", "")
                    .as_str()
                    .replace("' not found", "")
            } else {
                panic!(err);
            }
        }
    }
}

fn get_repo_info() -> (String, bool) {
    match Repository::open_from_env() {
        Ok(repo) => (get_branch(&repo), check_dirty_state(&repo)),
        Err(_) => /* probably not a repo */ (String::from(""), false),
    }
}

fn get_cwd() -> String {
    let cwd_path = env::current_dir().unwrap();
    let cwd = cwd_path.to_str().unwrap();

    let shortened = match env::home_dir() {
        Some(path) => str::replace(cwd, path.to_str().unwrap(), "~"),
        _ => String::from(""),
    };

    return shortened;
}

fn get_keymap_type(keymap: &str) -> String {
    let name = match keymap {
        "vicmd" => String::from(NORMAL_MODE).blue(),
        "main" => String::from(INSERT_MODE).green(),
        "" | _ => String::from("").normal(), // explicitly didn't set a keymap
    };

    format!("{}", name)
}

fn main() {
    let status = env::args().nth(1).unwrap();
    let keymap = env::args().nth(2).unwrap_or(String::from(""));
    let color = match status.as_str() {
        "0" => "magenta",
        _ => "red",
    };
    let cwd = get_cwd();
    let (branch, dirty) = get_repo_info();
    let status = if dirty {
        String::from("*")
    } else {
        String::from("")
    };
    let map = get_keymap_type(keymap.as_str());
    println!(
        "\n{} {}{} {}",
        cwd.blue(),
        branch.cyan(),
        status.cyan(),
        map
    );
    print!("{} ", INSERT_SYMBOL.color(color));
}
