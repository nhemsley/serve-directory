//! Definitions for routes served by the program

use super::ARGUMENTS;
use build_html::*;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use warp::filters::BoxedFilter;
use warp::path::FullPath;
use warp::reject::not_found;
use warp::reply::{html, Reply};
use warp::Filter;

/// The set of routes used by the program
pub fn routes() -> BoxedFilter<(impl Reply,)> {
    let handle_files = warp::fs::dir(&ARGUMENTS.folder);
    let handle_directories = warp::get()
        .and(warp::path::full())
        .and_then(path_to_html)
        .map(html);

    handle_files.or(handle_directories).boxed()
}

/// Converts the URL route of a folder to an HTML string of the contents
async fn path_to_html(route: FullPath) -> Result<String, warp::reject::Rejection> {
    let path = PathBuf::from(&ARGUMENTS.folder).join(&route.as_str()[1..]);

    let content = HtmlPage::new()
        .with_style(include_str!("styles.css"))
        .with_container(
            Container::new(ContainerType::Main)
                .with_attributes([("class", "border-box")])
                .with_preformatted_attr(route.as_str(), [("id", "header")])
                .with_container(links_container(path.as_path(), &route).ok_or_else(not_found)?),
        )
        .to_html_string();

    Ok(content)
}

/// Get the container that the links will be contained within
fn links_container(path: &Path, route: &FullPath) -> Option<Container> {
    let content_attrs = [("class", "content")];
    let mut links = Container::new(ContainerType::Div).with_attributes([("id", "wrapper")]);

    if route.as_str() != "/" {
        let parent = path
            .parent()
            .and_then(|path| path.strip_prefix(&ARGUMENTS.folder).ok())
            .and_then(Path::to_str)
            .map(|s| format!("/{}", s))?;
        links.add_link_attr(parent, "..", content_attrs);
    }

    read_dir(&path)
        .ok()?
        .filter_map(|res| res.ok().map(|x| x.path()))
        .filter_map(format_path)
        .for_each(|(path, name)| links.add_link_attr(path, name, content_attrs));

    Some(links)
}

/// Converts the provided `PathBuf` into the partial path off of the root, and the filename
fn format_path(path: PathBuf) -> Option<(String, String)> {
    Some((
        format!("/{}", path.strip_prefix(&ARGUMENTS.folder).ok()?.to_str()?), // Net path
        path.file_name()?.to_str()?.into(),                                   // File name
    ))
}
