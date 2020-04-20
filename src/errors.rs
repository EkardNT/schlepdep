use hyper::{Body, Response};

pub fn no_route() -> Response<Body> {
    return Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"no_route","message":\
            "No route found matching request path and method"}"#))
        .expect("Failed to build no_route error response")
}

pub fn body_too_large() -> Response<Body> {
    return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"body_too_large","message":\
            "Request body was larger than the maximum allowed"}"#))
        .expect("Failed to build body_too_large error response")
}

pub fn body_read_failed() -> Response<Body> {
    return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"body_read_failed","message":\
            "IO failure while reading request body"}"#))
        .expect("Failed to build body_read_failed error response")
}

pub fn req_json_parse() -> Response<Body> {
    return Response::builder()
        .status(400)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"req_json_parse","message":\
            "Request body did not parse as valid JSON"}"#))
        .expect("Failed to build req_json_parse error response")
}

pub fn internal() -> Response<Body> {
    return Response::builder()
        .status(500)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"internal","message":\
            "An unexpected internal error occurred within the service"}"#))
        .expect("Failed to build internal error response")
}

pub fn no_content_length() -> Response<Body> {
    return Response::builder()
        .status(411)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"no_content_length","message":\
            "The mandatory Content-Length header was not present"}"#))
        .expect("Failed to build no_content_length error response")
}