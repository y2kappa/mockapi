type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use aws_lambda_events::encodings::Body;
use http::Response;
use llambda::Request;

pub async fn handle(_: Request) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from("🦀 Hello, Netlify 🦀"))
        .expect("failed to render response");

    Ok(response)
}
