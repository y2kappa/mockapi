use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod handler;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let addr = ([127, 0, 0, 1], 7878).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

async fn handle(request: Request<Body>) -> Result<Response<Body>, Error> {
    let req = llambda::Request::from_hyper(request).await?;
    let lambda_response = handler::handle(req).await?;
    let resp = llambda::response::from_lambda(lambda_response);

    println!("Response {:?}", resp);

    Ok(resp)
}
