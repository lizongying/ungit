use clap::{Arg, Command};
use flate2::read::GzDecoder;
use reqwest;
use std::string::String;
use std::{env, process};
use tar::Archive;

#[tokio::main]
async fn main() {
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

    // TODO
    // match fetch_repo(repo, spec, target).await {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         process::exit(1);
    //     }
    // };

    match clone_repo(repo, spec, target) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
}

fn fetch_refs(repo: &str, spec: &str) -> Result<String, Box<dyn std::error::Error>> {
    let repo_url = format!("https://github.com/{repo}.git"); // 替换为实际的仓库 URL

    let output = process::Command::new("git")
        .arg("ls-remote")
        .arg(repo_url)
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let lines: Vec<&str> = stdout.lines().collect();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 2 {
                continue;
            }

            let hash = parts[0];
            let ref_name = parts[1];

            if ref_name == "HEAD" {
                if spec == "" {
                    return Ok(hash.to_string());
                }
                continue;
            }

            if ref_name.ends_with(spec) {
                return Ok(hash.to_string());
            }
        }

        Err("Not find".into())
    } else {
        let stderr = String::from_utf8(output.stderr)?;
        eprintln!("Error:\n{}", stderr);
        Err(format!("stderr {}", stderr).into())
    }
}

fn get_client() -> Result<reqwest::Client, reqwest::Error> {
    let mut builder = reqwest::Client::builder();

    let proxy_all = env::var("ALL_PROXY")
        .or_else(|_| env::var("all_proxy"))
        .ok();

    if let Some(proxy) = proxy_all {
        println!("proxy_all: {}", proxy);
        builder = builder.proxy(reqwest::Proxy::all(&proxy)?);
    }

    let proxy_https = env::var("HTTPS_PROXY")
        .or_else(|_| env::var("https_proxy"))
        .ok();

    if let Some(proxy) = proxy_https {
        println!("proxy_https: {}", proxy);
        builder = builder.proxy(reqwest::Proxy::https(&proxy)?);
    }

    let proxy_http = env::var("HTTP_PROXY")
        .or_else(|_| env::var("http_proxy"))
        .ok();

    if let Some(proxy) = proxy_http {
        println!("proxy_http: {}", proxy);
        builder = builder.proxy(reqwest::Proxy::http(&proxy)?);
    }

    builder.build()
}

async fn fetch_repo(
    repo: &str,
    spec: &str,
    target: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let hash = fetch_refs(repo, spec)?;

    let proxy_github = match env::var("GITHUB_PROXY") {
        Ok(proxy) => format!("{}/", proxy.trim_end_matches('/')),
        _ => String::new(),
    };
    println!("proxy: {}", proxy_github);

    let zip_url = format!("{proxy_github}https://github.com/{repo}/archive/{hash}.tar.gz");
    println!("zip_url: {zip_url}");

    let resp = get_client()?
        .get(&zip_url)
        .header("User-Agent", "")
        .send()
        .await?
        .bytes()
        .await?;

    let archive = GzDecoder::new(resp.as_ref());
    let mut archive = Archive::new(archive);
    archive.unpack(target)?;

    println!("Repository downloaded successfully.");
    Ok(())
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
