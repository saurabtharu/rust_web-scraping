mod news_scraper;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let url = "https://news.ycombinator.com/";
    let mut status = StatusCode::OK;

    let page = match async { reqwest::get(url).await?.text().await }.await {
        Ok(b) => b,
        Err(err) => {
            status = err.status().unwrap_or(StatusCode::BAD_REQUEST);
            format!("{err}")
        }
    };

    let mut news = news_scraper::NewsScraper::new();
    news.scrape(page);
    let response = news.get_news();
    let body = String::from_utf8_lossy(response.as_bytes()).to_string();

    let mut res = Response::new(Body::from(body));
    *res.status_mut() = status;
    Ok(res)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check if there's an environment variable for the port
    let port = std::env::var("PORT").unwrap_or_else(|_| "80".to_string());
    // parse the port number into u16
    let port = port.parse::<u16>()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Listening on {}", addr);

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    Ok(server.await?)
}
