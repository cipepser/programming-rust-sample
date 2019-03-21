use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
    routes: HashMap<String, C>,
}

impl<C> BasicRouter<C> where C: Fn(&Request) -> Response {
    /// Create an empty router.
    fn new() -> BasicRouter<C> {
        BasicRouter { routes: HashMap::new() }
    }

    /// Add a rounte to the router.
    fn add_router(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

fn get_from_response() -> Response {
    Response {
        code: 0,
        headers: HashMap::new(),
        body: vec![0],
    }
}

fn get_gcd_response(req: Request) -> Response {
    Response {
        code: 1,
        headers: HashMap::new(),
        body: vec![1],
    }
}

fn main() {
    let mut router = BasicRouter::new();
    router.add_router("/", |_| get_from_response());
    router.add_router("/gcd", |&req| get_gcd_response(
        Request {
            method: "req".to_string(),
            url: "example.com".to_string(),
            headers: HashMap::new(),
            body: vec![1],
        }
    ));
}
