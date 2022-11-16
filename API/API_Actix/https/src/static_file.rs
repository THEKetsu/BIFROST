use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
/* HOME */
pub async fn load(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "website/web.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/* LOGIN  */
pub async fn load2(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "website/log.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
/* ERROR 404 */
pub async fn not_found(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "error/error.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}