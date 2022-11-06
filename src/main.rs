extern crate hyper;
extern crate include_dir;

use cached::proc_macro::cached;
use cached::SizedCache;
use debug_print::debug_println;
use flate2::read::GzDecoder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use include_dir::{include_dir, Dir};
use std::convert::Infallible;
use std::env;
use std::io::Read;
use std::net::SocketAddr;

static WEBSITE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/site");

#[cached(
    type = "SizedCache<String, String>",
    create = "{ SizedCache::with_size(5) }",
    convert = r#"{ path.path().to_str().unwrap().to_string() }"#
)]
fn uncompress(path: &include_dir::File) -> String {
    let mut gz = GzDecoder::new(path.contents());
    let mut body = String::new();
    gz.read_to_string(&mut body).unwrap();
    body
}

async fn get_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::GET | &Method::HEAD => {
            // requested_path => &str
            let mut requested_path = req.uri().path().to_string();

            if requested_path.ends_with('/') {
                requested_path.push_str("index.html");
            };
            requested_path.push_str(".gz");

            // Remove / at the start
            let path = &requested_path[1..];
            debug_println!("Looking for {}", path);

            match WEBSITE_DIR.get_file(path) {
                Some(path) => Ok(Response::new(Body::from(uncompress(path)))),
                None => {
                    // remove .gz
                    let path = &requested_path[1..requested_path.len() - 3];
                    debug_println!("Looking for {}", path);
                    match WEBSITE_DIR.get_file(path) {
                        Some(path) => {
                            let body = path.contents_utf8().unwrap();
                            Ok(Response::new(Body::from(body)))
                        }
                        None => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("not found"))
                            .unwrap()),
                    }
                }
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body(Body::from("not implemented"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => 3000,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(get_file)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on port {}", port);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
