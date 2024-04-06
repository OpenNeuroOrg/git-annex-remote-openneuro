use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::{body::Buf, Request};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use serde::Deserialize;

pub async fn fetch_json(url: hyper::Uri) -> Result<Vec<User>, E> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    // Fetch the url...
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let res = sender.send_request(req).await?;

    // asynchronously aggregate the chunks of the body
    let body = res.collect().await?.aggregate();

    // try to parse as json with serde_json
    let users = serde_json::from_reader(body.reader())?;

    Ok(users)
}


#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    #[allow(unused)]
    name: String,
}