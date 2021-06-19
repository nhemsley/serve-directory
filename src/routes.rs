//! Definitions for routes served by the program

use super::ARGUMENTS;
use build_html::*;
use std::fs::read_dir;
use std::path::PathBuf;
use warp::filters::BoxedFilter;
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
async fn path_to_html(route: warp::path::FullPath) -> Result<String, warp::reject::Rejection> {
    let path = PathBuf::from(&ARGUMENTS.folder).join(&route.as_str()[1..]);

    let links: Container = read_dir(&path)
        .map_err(|_| warp::reject::not_found())?
        .filter_map(|res| res.ok().map(|x| x.path()))
        .filter_map(format_path)
        .fold(
            Container::new(ContainerType::Div).add_link_attr("..", "..", [("class", "content")]),
            |a, (path, name)| a.add_link_attr(format!("/{}", path), name, [("class", "content")]),
        )
        .with_attributes([("id", "wrapper")]);

    let content = HtmlPage::new()
        .add_style(include_str!("styles.css"))
        .add_container(
            Container::new(ContainerType::Main)
                .with_attributes([("class", "border-box")])
                .add_preformatted_attr(route.as_str(), [("id", "header")])
                .add_container(links),
        )
        .to_html_string();

    Ok(content)
}

/// Converts the provided `PathBuf` into the partial path off of the root, and the filename
fn format_path(path: PathBuf) -> Option<(String, String)> {
    let file_path: String = path.to_str()?.into();
    let file_name: String = path.file_name()?.to_str()?.into();
    Some((file_path, file_name))
}
