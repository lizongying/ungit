use clap::{Arg, Command};
use std::process;
use std::string::String;

fn main() {
    let cmd = Command::new("ungit")
        .version("0.1")
        .author("Li ZongYing <lizongying@msn.com>")
        .about("Clone a Git repository to a local directory")
        .arg(
            Arg::new("repo")
                .help("GitHub repository URL")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target")
                .help("Target directory")
                .required(false)
                .index(2),
        )
        .get_matches();

    let repo = match cmd.get_one::<String>("repo") {
        Some(repo) => repo,
        None => {
            eprintln!("Error: The 'repo' argument is required.");
            process::exit(1);
        }
    };

    let parts: Vec<&str> = repo.split('#').collect();

    let repo = if parts.len() > 1 {
        &parts[0].to_string()
    } else {
        repo
    };
    println!("repo: {}", repo);

    let spec = if parts.len() > 1 { parts[1] } else { "" };
    println!("spec: {}", spec);

    let target = match cmd.get_one::<String>("target") {
        Some(target) => target,
        None => {
            // If target is not provided, extract the part after the last '/' in the repo
            let parts: Vec<&str> = repo.split('/').collect();

            match parts.last() {
                Some(extracted_target) => &extracted_target.to_string(),
                None => {
                    eprintln!("Error: Could not extract target from repo URL.");
                    process::exit(1);
                }
            }
        }
    };
    println!("target: {}", target);

    match clone_repo(repo, spec, target) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
}

fn clone_repo(repo: &str, spec: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo_url = format!("git@github.com:{repo}.git");

    let mut cmd = process::Command::new("git");
    cmd.arg("clone")
        .arg(repo_url)
        .arg(target)
        .arg("--depth")
        .arg("1");

    if !spec.is_empty() {
        cmd.arg("--branch").arg(spec);
    }

    let exit_status = cmd.status()?;

    if !exit_status.success() {
        return Err(format!("Failed to clone repository. Exit status: {}", exit_status).into());
    }

    let git = format!("{target}/.git");
    let exit_status = process::Command::new("rm")
        .arg("-r")
        .arg("-f")
        .arg(git)
        .status()?;

    if !exit_status.success() {
        return Err(format!("Failed to clone repository. Exit status: {}", exit_status).into());
    }

    println!("Repository cloned successfully.");
    Ok(())
}
