//! A simple web server to serve a directory of static files

use lazy_static::lazy_static;
use log::{info, LevelFilter};
use routes::routes;
use std::net::{IpAddr, SocketAddr};
use structopt::StructOpt;

mod routes;

lazy_static! {
    /// The command line arguments passed into the program
    static ref ARGUMENTS: Arguments = Arguments::from_args();
}

/// Command line arguments for the program
#[derive(Debug, StructOpt, Clone, PartialEq)]
#[structopt(
    author = "Joseph Skubal",
    about = "Serve files in the specified directory and subdirectories"
)]
struct Arguments {
    /// The port that the server should bind to
    #[structopt(long, short, default_value = "8080")]
    pub port: u16,

    /// The root directory that should be served by the program
    #[structopt(default_value = ".")]
    pub folder: String,

    /// Whether additional data should be logged to the console
    #[structopt(long, short)]
    pub verbose: bool,

    /// Deploy on localhost, rather than attempting to bind to the external interface
    #[structopt(long, short)]
    pub localhost: bool,
}

/// Program Entry Point
#[tokio::main]
async fn main() {
    let ip_addr: IpAddr = if ARGUMENTS.localhost {
        IpAddr::from([127, 0, 0, 1])
    } else {
        local_ipaddress::get()
            .expect("No local IP address found")
            .parse()
            .expect("Found IP Address is invalid")
    };

    initialize_logger();
    let socket_addr = SocketAddr::from((ip_addr, ARGUMENTS.port));
    info!("Server listening on http://{}", socket_addr);
    warp::serve(routes()).bind(socket_addr).await;
}

/// Initialize the logger to print output
fn initialize_logger() {
    env_logger::builder()
        .format_module_path(false)
        .filter_level(LevelFilter::Info)
        .init()
}
