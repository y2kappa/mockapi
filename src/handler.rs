type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use crate::types::{Request, Response};

pub async fn handle(request: Request) -> Result<Response, Error> {
    println!("Request {:?}", request);
    let response = Response::from("🦀 Hello, Netlify 🦀");
    Ok(response)
}
