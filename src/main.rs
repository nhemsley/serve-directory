use tokio::signal;
use tokio::sync::oneshot;
use warp::{path, Filter};

use std::path::PathBuf;

mod route_utils;

/// Program Entry Point
///
/// Starts the server on `127.0.0.1:8080` and waits for the `Ctrl` + `C` shutdown
/// signal
#[tokio::main]
async fn main() {
    println!("Server starting up...");

    // Routes
    let files = warp::fs::dir("."); // Allow access to all files in the current directory
    let dirs = warp::get()
        .and(path::full().map(route_utils::route_to_file_path))
        .and_then(|path: PathBuf| async move { route_utils::dir_to_http(&path) })
        .map(|value| warp::reply::html(value));
    let routes = files.or(dirs);

    // Set up trigger to shutdown gracefully
    let (tx, rx) = oneshot::channel::<()>();

    // Start Server
    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
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
