/*
Luke Harding (@lukeh990) - Main entrypoint of program
Copyright (C) 2023 Luke Harding

Licensing information can be found in the "LICENSE" file in the root directory

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use mime_guess;
use rouille::{router, Response};
use std::env;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

mod vcard;

fn main() {
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
                } if let Ok(_card) = File::open(format!("cards/{}.vcf", id)) {
                    let data = vcard::create_vcard(id).to_string();
                    return Response::from_data("text/vcard", data);
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
        Some(socket) => return socket,
        None => return SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5000),
    }
}

fn not_found() -> Response {
    if let Ok(file) = File::open("public/404.html") {
        return Response::from_file("text/html", file);
    }

    Response::text("404 Not Found").with_status_code(404)
}
