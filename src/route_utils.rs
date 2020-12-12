//! This module contains utilities that provide functionality that is used for the
//! various warp routes

use std::fmt::{self, Display};
use std::fs::read_dir;
use std::path::PathBuf;

/// Represents an HTTP document that can be converted to
///
/// This struct is designed to allow the construction of HTTP documents
/// using the Builder Pattern
#[derive(Debug, PartialEq, Clone)]
pub struct HttpPage {
    title: String,
    content: Vec<String>,
}

impl Default for HttpPage {
    fn default() -> Self {
        HttpPage::new("")
    }
}

impl Display for HttpPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = self
            .content
            .iter()
            .fold(String::new(), |acc, next| format!("{}\n{}", acc, next));

        write!(
            f,
            r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>{}
            </body>
        </html>
        "#,
            self.title, content
        )
    }
}

impl HttpPage {
    /// Creates a new HTTP page with the given title and no content
    pub fn new(page_title: &str) -> Self {
        HttpPage {
            title: page_title.into(),
            content: Vec::new(),
        }
    }

    /// Returns a copy of this HTTP document with a list appended
    ///
    /// This function can be chained.
    ///
    /// # Example
    /// ```
    /// let mut page = HttpPage::new("Zoo");
    /// page.add_list(["Lions", "Tigers", "Bears"])
    ///     .add_list(vec!["Oh", "my!"]);
    /// ```
    pub fn add_list<T: Display>(&mut self, items: &[T]) -> Self {
        let list_items = items
            .iter()
            .map(|item| format!("<li>{}</li>", item))
            .fold(String::new(), |acc, next| format!("{}\n{}", acc, next));
        let list = format!(
            r"
        <ul>
            {}
        </ul>",
            list_items
        );
        let mut new_http_page = self.clone();
        new_http_page.content.push(list);
        new_http_page
    }
}

/// Converts the route specified in the URL into a path relative to the
/// current working directory on disc
///
/// This function is sensitive to the target operating system. On Windows,
/// the file path will use the `\` character, while on other systems, the
/// path will use `/`
///
/// Because this function uses the [`warp::path::FullPath`] struct, it is
/// assumed that the path starts with '/'
///
/// # Example
/// On Linux,
/// ```
/// warp_path; // "/hello/world"
/// assert_eq!(route_to_file_path(warp_path), PathBuf::from("./hello/world"));
/// ```
///
/// On Winodws,
/// ```
/// warp_path; // "/hello/world"
/// assert_eq!(route_to_file_path(warp_path), PathBuf::from(r".\hello\world"));
/// ```
pub fn route_to_file_path(warp_path: warp::path::FullPath) -> PathBuf {
    let file_path: String = if cfg!(target_os = "windows") {
        warp_path.as_str().replace('/', r"\")
    } else {
        warp_path.as_str().into()
    };

    PathBuf::from(format!(".{}", file_path))
}

/// Converts the specified path into an http document with the list of files
/// in that path
pub fn dir_to_http(path: &PathBuf) -> Result<String, warp::reject::Rejection> {
    let entries = match read_dir(path) {
        Ok(result) => result,
        Err(_) => return Err(warp::reject::not_found()),
    };

    let file_names: Vec<String> = entries
        .filter_map(|res| res.ok())
        .filter_map(|file| file.file_name().to_str().map(String::from))
        .collect();

    let mut path: String = path.to_str().map(|path| &path[1..]).unwrap().into();
    if !(path.ends_with('/') || path.ends_with('\\')) {
        path.push('/')
    }

    let links: Vec<String> = file_names
        .iter()
        .map(|name| format!("{}{}", path, name))
        .zip(file_names.iter())
        .map(|(addr, name)| format!(r#"<a href="{}">{}</a>"#, addr, name))
        .collect();

    let http_page = HttpPage::new("Directory Contents").add_list(&links);

    Ok(format!("{}", http_page))
}
