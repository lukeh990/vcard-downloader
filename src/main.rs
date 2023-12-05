use rouille::Request;
use rouille::Response;
use std::env;

fn main() {
    let bind: String = match env::var("BIND") {
        Ok(port) => port,
        Err(e) => "0.0.0.0:5000".to_string()
    };

    println!("Starting server on {}", bind);

    rouille::start_server(bind, move |request| {
        Response::text("Hello World")
    });
}
