//! Markdown rendering handler

use super::super::ARGUMENTS;
use build_html::*;
use std::fs;
use std::path::PathBuf;
use warp::path::FullPath;
use warp::reject::not_found;
use warp::reply::{html, Reply};

/// Handle markdown file rendering
pub async fn handle_markdown_file(route: FullPath) -> Result<impl Reply, warp::reject::Rejection> {
    let path = PathBuf::from(&ARGUMENTS.folder).join(&route.as_str()[1..]);
    
    // Check if the path exists and is a markdown file
    if !path.exists() || !path.is_file() {
        return Err(not_found());
    }
    
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    if extension != "md" && extension != "markdown" {
        return Err(not_found());
    }
    
    // Read the markdown file
    let markdown_content = fs::read_to_string(&path)
        .map_err(|_| not_found())?;
    
    // Convert markdown to HTML
    let html_content = markdown::to_html(&markdown_content);
    
    // Create a styled HTML page
    let page = HtmlPage::new()
        .with_style(include_str!("../styles.css"))
        .with_style(markdown_styles())
        .with_container(
            Container::new(ContainerType::Main)
                .with_attributes([("class", "border-box markdown-content")])
                .with_raw(&html_content)
        )
        .to_html_string();
    
    Ok(html(page))
}

/// Additional CSS styles for markdown rendering
fn markdown_styles() -> &'static str {
    r#"
.markdown-content {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    line-height: 1.6;
    color: #333;
}

.markdown-content h1,
.markdown-content h2,
.markdown-content h3,
.markdown-content h4,
.markdown-content h5,
.markdown-content h6 {
    margin: 1.5em 0 0.5em 0;
    font-weight: bold;
}

.markdown-content h1 {
    font-size: 2em;
    border-bottom: 2px solid #eee;
    padding-bottom: 0.3em;
}

.markdown-content h2 {
    font-size: 1.5em;
    border-bottom: 1px solid #eee;
    padding-bottom: 0.2em;
}

.markdown-content p {
    margin: 1em 0;
}

.markdown-content pre {
    background: #f6f8fa;
    border-radius: 6px;
    padding: 16px;
    overflow: auto;
    font-family: 'Courier New', Courier, monospace;
}

.markdown-content code {
    background: #f6f8fa;
    padding: 0.2em 0.4em;
    border-radius: 3px;
    font-family: 'Courier New', Courier, monospace;
}

.markdown-content pre code {
    background: none;
    padding: 0;
}

.markdown-content blockquote {
    border-left: 4px solid #dfe2e5;
    padding-left: 16px;
    margin: 1em 0;
    color: #6a737d;
}

.markdown-content ul,
.markdown-content ol {
    margin: 1em 0;
    padding-left: 2em;
}

.markdown-content table {
    border-collapse: collapse;
    width: 100%;
    margin: 1em 0;
}

.markdown-content th,
.markdown-content td {
    border: 1px solid #dfe2e5;
    padding: 6px 13px;
    text-align: left;
}

.markdown-content th {
    background: #f6f8fa;
    font-weight: bold;
}

.markdown-content a {
    color: #0366d6;
    text-decoration: none;
}

.markdown-content a:hover {
    text-decoration: underline;
}
"#
}