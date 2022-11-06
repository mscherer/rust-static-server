extern crate hyper;
extern crate include_dir;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use include_dir::{include_dir, Dir};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::env;

static WEBSITE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/site");

async fn get_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::GET | &Method::HEAD => {
            // requested_path => &str
            let mut requested_path = req.uri().path().to_string();

            if requested_path.ends_with('/') {
                requested_path.push_str("index.html");
            };

            // Remove / at the start
            let path = &requested_path[1..];

            match WEBSITE_DIR.get_file(path) {
                Some(path) => {
                    let body = path.contents_utf8().unwrap();
                    Ok(Response::new(Body::from(body)))
                }
                None => {
                    let mut res = Response::new(Body::from("not found"));
                    *res.status_mut() = StatusCode::NOT_FOUND;
                    Ok(res)
                }
            }
        }
        _ => {
            let mut res = Response::new(Body::from("not implemented"));
            *res.status_mut() = StatusCode::NOT_IMPLEMENTED;
            Ok(res)
        }
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = match env::var("PORT") {
        Ok(val) => {
            val.parse().unwrap()
        }
        Err(_) => 3000,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(get_file))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on port {}", port);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
