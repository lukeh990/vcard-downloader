// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2023 Luke Harding

use rouille::{router, Response};
use std::env;
use std::error;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

pub mod models;
pub mod schema;

mod db;
mod vcard_processor;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let bind = get_bind_address();

    println!("Starting server on {}:{1}", bind.ip(), bind.port());

    rouille::start_server(bind, move |request| {
        router!(request,
            (GET) (/) => {
                if let Ok(file) = File::open("public/index.html") {
                    return Response::from_file("text/html", file);
                }

                not_found()
            },

            (GET) (/{id: String}) => {
                if let Ok(public) = File::open(format!("public/{}", id)) {
                    let mime_type = match mime_guess::from_path(format!("public/{}", id)).first() {
                        Some(x) => x.to_string(),
                        None => {
                             return Response::text("500 Server Error").with_status_code(500);
                        }
                    };
                    return Response::from_file(mime_type, public);
                } else if let Ok(card) = db::search_card(id) {
                        return Response::from_data("text/vcard", vcard_processor::create_vcard(card))
                }

                not_found()
            },

            (GET) (/{alias_org: String}/{alias_name: String}) => {
                if let Ok(card) = db::alias_search_card(format!("{}/{1}", alias_org, alias_name)) {
                    return Response::from_data("text/vcard", vcard_processor::create_vcard(card))
                }

                not_found()
            },

            _ => not_found()
        )
    });
}

fn get_bind_address() -> SocketAddr {
    let environment = env::var("BIND").unwrap_or_default();

    let env_socket = match environment.to_socket_addrs() {
        Ok(mut socket_iter) => socket_iter.next(),
        Err(_) => None,
    };

    match env_socket {
        Some(socket) => socket,
        None => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5000),
    }
}

fn not_found() -> Response {
    if let Ok(file) = File::open("public/404.html") {
        return Response::from_file("text/html", file);
    }

    Response::text("404 Not Found").with_status_code(404)
}
