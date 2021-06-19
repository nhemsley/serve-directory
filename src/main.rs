//! A simple web server to serve a directory of static files

use lazy_static::lazy_static;
use log::{error, info, warn, LevelFilter};
use routes::routes;
use structopt::StructOpt;
use tokio::signal;

mod routes;

lazy_static! {
    /// The command line arguments passed into the program
    static ref ARGUMENTS: Arguments = {
        match Arguments::from_args_safe() {
            Ok(opt) => opt,
            Err(e) => {
                error!("{}", e.to_string().trim());
                warn!("Program shutting down");
                std::process::exit(1);
            }
        }
    };
}

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Joseph Skubal",
    about = "Serve files in the specified directory and subdirectories"
)]
struct Arguments {
    /// The port that the server should bind to
    #[structopt(long, short, default_value = "8080")]
    pub port: u16,

    /// The root directory that should be served by the
    #[structopt(default_value = ".")]
    pub folder: String,
}

/// Program Entry Point
#[tokio::main]
async fn main() {
    initialize_logger();
    info!("Starting up");
    start_server().await;
}

/// Initialize the logger to print output
fn initialize_logger() {
    env_logger::builder()
        .format_module_path(false)
        .filter_level(LevelFilter::Info)
        .init()
}

/// Start the server
async fn start_server() {
    let shutdown = async {
        if signal::ctrl_c().await.is_err() {
            error!("Something went wrong getting Ctrl+C signal");
        } else {
            info!("Ctrl+C signal received. Shutting down");
        }
    };
    let binding = ([127, 0, 0, 1], ARGUMENTS.port);
    let (addr, server) = warp::serve(routes()).bind_with_graceful_shutdown(binding, shutdown);

    info!("Server listening on {}", addr);
    server.await;
}
