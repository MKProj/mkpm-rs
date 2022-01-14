use open::that;
/// This contains all code related to the read command
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::path::{PathBuf};
use std::io::{self, Cursor};
/// Result type to help download from web
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
/// DL is the base Download link for the pdf formats
pub const DL: &str = "http://mkproj.com/download/";
/// WEB_L is the base for the web formats
pub const WEB_L: &str = "http://{}.mkproj.com";
/// The Book struct that is deserialized to function for the read command
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub title: String,
    short: String,
}
/// The formats that the books follow:
/// - PDF: Donwload locally
/// - Web: Open using `open::that()`
impl Book {
    /// Open web link
    pub fn open_web(&self) {
        let wl = WEB_L.to_owned().replace("{}", self.short.as_str());
        that(&wl).unwrap()
    }
    /// Download pdf from mkproj.com
    pub async fn get_pdf(&self) -> Result<()>{
        let url  = format!("{}{}.zip", DL, &self.short);
        let resp = reqwest::get(url).await?;
        let mut file = std::fs::File::create(format!("{}.zip", &self.short))?;
        let mut content = Cursor::new(resp.bytes().await?);
        io::copy(&mut content, &mut file)?;
        Ok(())

    }
    /// Load all books from book.json
    pub fn load(p: PathBuf) -> Vec<Self> {
        let s = std::fs::read_to_string(p).unwrap();
        from_str(&s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Book;
    use std::path::Path;
    #[test]
    fn test_load() {
        let b = Book::load(Path::new("book.json").to_path_buf());
        println!("{:?}", b);
    }
}
