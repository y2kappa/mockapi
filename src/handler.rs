type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use aws_lambda_events::encodings::Body;
use http::Response;
use llambda::request::{GetRequest, Request};

pub async fn handle(req: Request) -> Result<Response<Body>, Error> {
    println!("{:?}", req);

    let body = match req {
        Request::Get(GetRequest { path, parameters }) => match path.as_str() {
            "/.netlify/functions/mockapi/quotes" => routes::quote(),
            "/.netlify/functions/mockapi/jokes" => routes::joke(),
            "/.netlify/functions/mockapi/delay" => routes::delay(parameters),
            _ => "Not implemented!".to_string(),
        },
        _ => "🦀 Hello, Netlify 🦀".to_string(),
    };

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(Body::from(body))
        .expect("failed to render response");

    Ok(response)
}

mod routes {

    use crate::data_provider;
    use std::collections::HashMap;

    pub fn joke() -> String {
        let jokes = data_provider::JOKES;
        let len = jokes.len();
        jokes[rand(len)].to_string()
    }

    pub fn quote() -> String {
        let quotes = data_provider::QUOTES;
        let len = quotes.len();
        let quote = quotes[rand(len)];
        format!("{} - {}", quote.0, quote.1)
    }
    fn rand(max: usize) -> usize {
        let nr: usize = rand::random();
        nr % max
    }

    pub fn delay(params: HashMap<String, Vec<String>>) -> String {
        let delay = params.get("seconds").unwrap_or(&vec!["0".to_string()])[0].clone();
        match delay.parse::<u32>() {
            Ok(duration) => {
                std::thread::sleep(std::time::Duration::from_secs(duration.into()));
                format!("Response returned after {:?} seconds", duration)
            }
            Err(err) => {
                format!("Could not parse 'seconds' parameter {:?}", err)
            }
        }
    }
}
