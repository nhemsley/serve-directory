//! This module contains utilities that provide functionality that is used for the
//! various warp routes

use build_html::*;
use std::fs::read_dir;
use std::path::PathBuf;

/// Converts the URL route of a folder to an HTML string of the contents
pub fn path_to_html(route: warp::path::FullPath) -> Result<String, warp::reject::Rejection> {
    let mut path: String = match PathBuf::from(format!(".{}", route.as_str())).to_str() {
        Some(path) => path.into(),
        None => return Err(warp::reject::not_found()),
    };

    if !(path.ends_with('/')) {
        path.push('/')
    }

    let links: Container = read_dir(&path)
        .map_err(|_| warp::reject::not_found())?
        .filter_map(|res| res.ok())
        .filter_map(|file| file.file_name().to_str().map(String::from))
        .fold(
            Container::new(ContainerType::UnorderedList).add_link("..", ".."),
            |a, n| a.add_link(&format!("{}{}", path, n), &n),
        );

    Ok(HtmlPage::new().add_container(links).to_html_string())
}
