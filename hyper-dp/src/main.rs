use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

mod handler;

async fn start_server() -> Result<(), String> {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our routes main function.
    let service = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handler::register))
    });

    let server = Server::bind(&addr).serve(service);
    println!("Starting server on http://{}", addr);

    // Run this server... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    start_server().await?;
    Ok(())
}
