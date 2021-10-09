//! `serve-directory` is a simple command line utility to serve static files from the command line. In
//! the general case, we really mean *simple*. To start a web server serving the contents of the current
//! directory and its subdirectories, just run the executable without arguments.
//!
//! ```bash
//! $ serve-directory
//! ```
//!
//! Files are accessed based on their relative position to the target folder. Consider the following
//! (abbreviated) rust project structure:
//! ```text
//! Project Structure:          Route:
//!  .                          /
//!  ├─ src/                    /src
//!  │  ├─ main.rs              /src/main.rs
//!  │  └─ stuff.rs             /src/stuff.rs
//!  ├─ target/                 /target
//!  │  └─ debug/               /target/debug
//!  │     └─ app.exe           /target/debug/app.exe
//!  └─ README.md               /README.md
//! ```
//!
//! As you can see, the correlation between the file structure and the route is quite natural.
//!
//! ## Why Should I Care?
//! Sometimes, configuring network folders is way more work than it's worth to simply pass a couple
//! files between machines, especially if you're moving files to or from a headless system. While users
//! could use a tool like SFTP, `serve-directory` provides a nice graphical way to browse the filesystem
//! in a web browser that should prove less daunting for new users. It also allows downloading files to
//! multiple clients without having to log in through SSH multiple times.
//!
//! It's also easy and doesn't require any additional setup (provided the user has a compiled
//! executable). Most users already have a browser on their machine. And, in the event that the user
//! does not, they can use `curl` or `wget` to download files over HTTP.
//!
//! While it's evident that there's a use case for a simple and erogonomic HTTP file server, why
//! would you use `serve-directory` instead of one of the more established options like the
//! [serve](https://www.npmjs.com/package/serve) package for NodeJS? I will concede that in many cases,
//! serve will probably suit your needs better. However, because `serve-directory` is written in Rust
//! and natively compiled, it can be used and distributed in a single executable without a runtime.

use lazy_static::lazy_static;
use log::{info, LevelFilter};
use routes::routes;
use std::net::{IpAddr, SocketAddr};
use structopt::StructOpt;
use tokio::signal::ctrl_c;

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

    /// Whether additional data should be logged to the console
    #[structopt(long, short)]
    pub verbose: bool,

    /// Deploy on localhost, rather than attempting to bind to the external interface
    #[structopt(long, short)]
    pub localhost: bool,

    /// The root directory that should be served by the program
    #[structopt(default_value = ".")]
    pub folder: String,
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
    let handle = tokio::spawn(warp::serve(routes()).bind(socket_addr));
    info!("Server listening on http://{}", socket_addr);

    ctrl_c().await.expect("Unalbe to get Ctrl+C signal");
    info!("Ctrl+C received. Shutting down");
    handle.abort();
    handle.await.unwrap_or(());
}

/// Initialize the logger to print output
fn initialize_logger() {
    let level = if ARGUMENTS.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::builder()
        .format_module_path(false)
        .filter(Some("serve_directory"), level)
        .init()
}
