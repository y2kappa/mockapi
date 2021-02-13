// Hyper dependencies
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod handler;
pub mod types;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let addr = ([127, 0, 0, 1], 7878).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Error> {
    let response = handler::handle(types::Request::try_from_hyper(req).await?)
        .await
        .unwrap();
    Ok(types::Response::into_hyper(response).unwrap())
}
