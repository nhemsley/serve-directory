//! This application is a simple web server that serves the current working directory.
//! 
//! Above all, this application is meant to be simple. Serving the working directory can
//! be accomplished by using the following command:
//! ```bash
//! $ serve-directory
//! ```

use tokio::signal;
use tokio::sync::oneshot;
use warp::{path, Filter};

mod route_utils;

/// Program Entry Point
///
/// Starts the server on `127.0.0.1:8080` and waits for the `Ctrl` + `C` shutdown
/// signal
#[tokio::main]
async fn main() {
    println!("Server starting up...");

    // Set up trigger to shutdown gracefully
    let (tx, rx) = oneshot::channel::<()>();

    // Start Server
    let (_addr, server) =
        warp::serve(routes()).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
            rx.await.ok();
        });
    tokio::task::spawn(server);
    println!("Now serving on http://localhost:8080/");

    // Send the shutdown signal when Ctrl + C is pressed
    signal::ctrl_c().await.unwrap();
    println!("Shutting down gracefully...");
    if tx.send(()).is_err() {
        eprintln!("Unable to shut down gracefully!");
    };
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
