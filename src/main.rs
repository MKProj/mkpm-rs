mod commands;
use commands::Repo;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use commands::{Book, Result};
use std::process::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "mkpm", about = "Package manager for MKProject Projects")]
enum CLI {
    #[structopt(about = "Read/Get MKProject books")]
    Read {
        #[structopt(short, long)]
        title: String,
        #[structopt(short, long)]
        format: Option<String>,
    },
    #[structopt(about = "Install MKProject Applications")]
    /*Install {
        #[structopt(short, long)]
        app_name: String,
    }, */
    #[structopt(about = "Update MKPM")]
    Update,
    #[structopt(about = "Clone MKProjects Repos")]
    Clone {
        #[structopt(short, long)]
        repo: String,
        #[structopt(short, long, parse(from_os_str))]
        path: Option<PathBuf>,
    },
}
#[tokio::main]
async fn main() -> Result<()>{
    let cli = CLI::from_args();
    match cli {
        CLI::Read { title, format } => {
            let books = Book::load(Path::new("book.json").to_path_buf());
            for i in books{
                if &i.title == &title{
                    match &format{
                        Some(f) => match f.as_str(){
                            "pdf" => i.get_pdf().await?,
                            "web" => i.open_web(),
                                _ => i.get_pdf().await?
                        }
                        None => i.get_pdf().await?
                    }
                }
            }
        }
       // CLI::Install { app_name } => {}
        CLI::Update => {
            Command::new("cargo")
                .arg("install")
                .arg("mkpm")
                .spawn()?;
        }
        CLI::Clone { repo, path } => {
            let repos = Repo::load(Path::new("repo.json").to_path_buf());
            for i in repos {
                if &i.name == &repo {
                    println!("Cloning {}.......", &i.name);
                    i.clone(&path)
                }
            }
        }
    }
    Ok(())
}
