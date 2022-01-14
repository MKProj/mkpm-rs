use git2::Repository;
/// Clone contains all related code to clone MKProject repos
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
/// REPO is the base gtihub link for the MKProject Repos
pub const REPO: &str = "http://github.com/MKProj/{}.git";
/// Repo is used to contain names and descriptions of the various repos
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repo {
    pub name: String,
    pub about: String,
}

impl Repo {
    pub fn clone(&self, path: &Option<PathBuf>) {
        // Set the path for the repo to be cloned in
        let path: PathBuf = match path {
            Some(p) => {
                Path::new(p).to_owned().join(&self.name)
            },
            None => {
                Path::new(&self.name).to_path_buf()
            },
        };
        // Create url
        let url = REPO.replace("{}", &self.name);
        // Clone repo
        Repository::clone(&url, path.clone()).expect("Coudln't clone repo");
    }
    pub fn load(p: PathBuf) -> Vec<Self> {
        let s = std::fs::read_to_string(p).unwrap();
        serde_json::from_str(&s).unwrap()
    }
}
