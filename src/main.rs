use dotenvy::dotenv;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use log::{error, info};
use std::{convert::Infallible, env, net::SocketAddr, str::FromStr};
use url::Url;

async fn proxy(
    req: Request<Body>,
    client: Client<HttpConnector>,
    backend_url: Url,
) -> Result<Response<Body>, hyper::Error> {
    let mut new_uri = backend_url.clone();
    new_uri.set_path(req.uri().path());
    new_uri.set_query(req.uri().query());

    let uri = Uri::from_str(new_uri.as_ref()).unwrap();
    let (parts, body) = req.into_parts();

    let mut proxied_req = Request::builder()
        .method(parts.method)
        .uri(uri)
        .body(body)
        .unwrap();

    *proxied_req.headers_mut() = parts.headers;

    client.request(proxied_req).await
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let backend_url = env::var("BACKEND_URL").expect("BACKEND_URL must be set");
    let parsed_url = Url::parse(&backend_url).expect("Invalid BACKEND_URL");

    let addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let socket_addr: SocketAddr = addr.parse().expect("Invalid LISTEN_ADDR");

    let client = Client::builder()
        .pool_max_idle_per_host(
            env::var("MAX_IDLE_CONNS_PER_HOST")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
        )
        .build_http();

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        let backend_url = parsed_url.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy(req, client.clone(), backend_url.clone())
            }))
        }
    });

    info!(
        "Starting proxy server at {} forwarding to {}",
        socket_addr, backend_url
    );

    if let Err(e) = Server::bind(&socket_addr).serve(make_svc).await {
        error!("Server error: {}", e);
    }
}
