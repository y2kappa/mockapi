use std::collections::HashMap;

#[derive(Debug)]
pub struct GetRequest {
    pub path: String,
    pub parameters: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub struct PostRequest {
    pub path: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum Request {
    Get(GetRequest),
    Post(PostRequest),
}

pub struct Response {
    pub bytes: Vec<u8>,
}

impl Request {
    pub async fn try_from_hyper(req: hyper::Request<hyper::Body>) -> Result<Request, String> {
        match req.method() {
            &http::Method::GET => {
                let path = req.uri().path().to_owned();

                let parameters = match req.uri().query() {
                    Some(parameters) => {
                        let parameters: HashMap<String, String> =
                            url::form_urlencoded::parse(parameters.as_bytes())
                                .into_owned()
                                .collect();
                        let parameters: HashMap<String, Vec<String>> =
                            parameters.into_iter().map(|(k, v)| (k, vec![v])).collect();
                        parameters
                    }
                    None => HashMap::new(),
                };

                Ok(Request::Get(GetRequest { path, parameters }))
            }
            &http::Method::POST => {
                let path = req.uri().path().to_owned();
                let bytes = hyper::body::to_bytes(req.into_body())
                    .await
                    .unwrap()
                    .to_vec();

                Ok(Request::Post(PostRequest { path, bytes }))
            }
            _ => Err("Other method types are not supported".to_string()),
        }
    }

    pub fn try_from_lambda(req: netlify_lambda_http::Request) -> Result<Request, String> {
        match req.method() {
            &http::Method::GET => Ok(Request::Get(utils::get_request_to_raw(&req))),
            &http::Method::POST => {
                let post_request = match utils::post_request_to_raw(req) {
                    Ok(post) => post,
                    Err(err) => {
                        return Err(err.to_string());
                    }
                };

                Ok(Request::Post(post_request))
            }
            _ => Err("Unmatched request type".to_string()),
        }
    }
}

impl Response {
    pub fn into_lambda(
        self: Response,
    ) -> Result<netlify_lambda_http::Response<aws_lambda_events::encodings::Body>, String> {
        Ok(netlify_lambda_http::Response::builder()
            .status(200)
            .header("Content-Encoding", "application/protobuf")
            .body(aws_lambda_events::encodings::Body::from(self.bytes))
            .expect("failed to render response"))
    }

    pub fn into_hyper(self: Response) -> Result<hyper::Response<hyper::Body>, String> {
        Ok(hyper::Response::builder()
            .status(200)
            .header("Content-Encoding", "application/protobuf")
            .body(hyper::Body::from(self.bytes))
            .expect("failed to render response"))
    }
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        Response { bytes: s.into() }
    }
}

mod utils {
    use super::*;

    pub fn query_to_hashmap(query: &netlify_lambda_http::StrMap) -> HashMap<String, Vec<String>> {
        let mut hash = HashMap::new();
        for item in query.iter() {
            if let Some(values) = query.get_all(item.0) {
                hash.insert(
                    item.0.to_string(),
                    values.iter().map(|x| x.to_string()).collect(),
                );
            } else {
                hash.insert(item.0.to_string(), vec![]);
            }
        }

        hash
    }

    pub fn body_to_bytes(req: netlify_lambda_http::Request) -> Result<Vec<u8>, &'static str> {
        use aws_lambda_events::encodings::Body;
        let request = match req.into_body() {
            Body::Binary(v) => Ok(v),
            Body::Text(s) => Ok(s.into_bytes()),
            _ => Err("not found"),
        };
        request
    }

    pub fn request_to_path(request: &netlify_lambda_http::Request) -> String {
        let uri = request.uri();
        let path = uri.path();
        path.to_owned()
    }

    pub fn get_request_to_raw(request: &netlify_lambda_http::Request) -> GetRequest {
        use netlify_lambda_http::ext::RequestExt;
        GetRequest {
            path: request_to_path(&request),
            parameters: query_to_hashmap(&request.query_string_parameters()),
        }
    }

    pub fn post_request_to_raw(
        request: netlify_lambda_http::Request,
    ) -> Result<PostRequest, &'static str> {
        let path = request_to_path(&request);
        let bytes = match body_to_bytes(request) {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err);
            }
        };
        let request = PostRequest { path, bytes };
        Ok(request)
    }
}
