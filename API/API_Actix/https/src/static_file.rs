use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
pub async fn load(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "website/web.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn load2(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "website/log.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}