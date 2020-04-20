use hyper::{Body, Response};

pub fn no_route() -> Response<Body> {
    return Response::builder()
        .status(404)
        .body(Body::from(r#"{"error":"no_route","message":"No route found matching request path and method"}"#))
        .expect("Failed to build no_route error response")
}