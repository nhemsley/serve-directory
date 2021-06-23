//! A simple web server to serve a directory of static files

use lazy_static::lazy_static;
use local_ip_address::find_af_inet;
use log::{error, info, warn, LevelFilter};
use routes::routes;
use std::net::{IpAddr, Ipv4Addr};
use structopt::StructOpt;
use tokio::signal;
use tokio::sync::watch;
use tokio::task::JoinHandle;

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

    /// The root directory that should be served by the
    #[structopt(default_value = ".")]
    pub folder: String,
}

/// Program Entry Point
#[tokio::main]
async fn main() {
    initialize_logger();
    info!("Starting up");
    let (tx, rx) = watch::channel(false);

    let mut handles = Vec::with_capacity(2);
    handles.push(start_service(Ipv4Addr::new(127, 0, 0, 1), rx.clone()));
    if let Some(addr) = ip_address() {
        handles.push(start_service(addr, rx));
    }

    if signal::ctrl_c().await.is_err() || tx.send(true).is_err() {
        error!("An error occurred during shutdown");
    } else {
        info!("Ctrl+C signal received. Initiating shutdown");
    }

    for handle in handles {
        if handle.await.is_err() {
            error!("There was a problem shutting down");
        }
    }
}

/// Initialize the logger to print output
fn initialize_logger() {
    env_logger::builder()
        .format_module_path(false)
        .filter_level(LevelFilter::Info)
        .init()
}

/// Attempt to get the current IP address of the system
fn ip_address() -> Option<Ipv4Addr> {
    find_af_inet()
        .ok()?
        .into_iter()
        .map(|(_dev, addr)| addr)
        .filter_map(|x| match x {
            IpAddr::V4(x) => Some(x),
            _ => None,
        })
        .find(|x| x.is_private())
}

/// Starts the service, bound to the specified IP address
fn start_service(ip: Ipv4Addr, mut rx: watch::Receiver<bool>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let binding = (ip.octets(), ARGUMENTS.port);
        let shutdown = async move { rx.changed().await.unwrap_or(()) };
        let (addr, server) = warp::serve(routes()).bind_with_graceful_shutdown(binding, shutdown);
        info!("Server listening on {}", addr);
        server.await
    })
}
