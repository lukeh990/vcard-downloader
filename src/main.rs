use rouille::{Response, router};
use std::env;
use std::fs::File;

fn main() {
    let bind: String = match env::var("BIND") {
        Ok(port) => port,
        Err(_e) => "0.0.0.0:5000".to_string()
    };

    println!("Starting server on {}", bind);

    rouille::start_server(bind, move |request| {
        router!(request,
            (GET) (/) => {
                Response::text("I will survive")
            },

            (GET) (/{id: String}) => {
                let file = match File::open(format!("cards/{}.vcf", id)) {
                    Ok(file) => file,
                    Err(_) => return Response::text("404 Not Found").with_status_code(404)
                };

                Response::from_file("text/x-vcard", file)
            },

            _ => Response::text("404 Not Found").with_status_code(404)
        )
    });
}
