use http::{Request, Response};
use http::{Request, Response, StatusCode};

fn send(req: Request<()>) -> Response<()> {
    // ...
}

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
}

fn main() {


    let mut request = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "my-awesome-agent/1.0");
    
    if needs_awesome_header() {
        request = request.header("Awesome", "yes");
    }
    
    let response = send(request.body(()).unwrap());
    
    
}