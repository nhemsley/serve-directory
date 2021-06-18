//! This application is a simple web server that serves the current working directory.
//!
//! Above all, this application is meant to be simple. Serving the working directory can
//! be accomplished by using the following command:
//! ```bash
//! $ serve-directory
//! ```

use clap::{App, Arg};
use log::{error, info, LevelFilter};
use tokio::signal;
use warp::{path, Filter};

mod route_utils;

/// Program Entry Point
#[tokio::main]
async fn main() {
    let matches = App::new("serve-directory")
        .version("0.1.0")
        .author("Joseph Skubal")
        .about("Serves files in the current working directory and subdirectories")
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .help("The port to which the server should attempt to bind")
                .takes_value(true),
        )
        .get_matches();

    initialize_logger();
    info!("Starting up");
    let port = matches
        .value_of("port")
        .map(|x| x.parse::<u16>())
        .unwrap_or(Ok(8080));

    match port {
        Ok(port) => start_server(port).await,
        Err(_) => error!("Invalid port specified")
    }
}

/// Initialize the logger to print output
fn initialize_logger() {
    env_logger::builder()
        .format_module_path(false)
        .filter_level(LevelFilter::Info)
        .init()
}

/// Start the server
async fn start_server(port: u16) {
    let shutdown = async {
        if signal::ctrl_c().await.is_err() {
            error!("Something went wrong getting Ctrl+C signal")
        }
    };
    #[rustfmt::skip]
    let (addr, server) = warp::serve(routes())
        .bind_with_graceful_shutdown(([127, 0, 0, 1], port), shutdown);

    info!("Server listening on {}", addr);
    server.await;
}

/// Provides the Filters specifying the different routes that the server can
/// respond to
fn routes() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let handle_files = warp::fs::dir(".");

    let handle_directories = warp::get()
        .and(path::full())
        .and_then(|fp| async move { route_utils::path_to_html(fp) })
        .map(|value| warp::reply::html(value));

    handle_files.or(handle_directories).boxed()
}
