use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod handler;
pub mod types;

#[lambda(http)]
#[tokio::main]
async fn main(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let response = handler::handle(types::Request::try_from_lambda(request)?)
        .await
        .unwrap();
    Ok(types::Response::into_lambda(response).unwrap())
}
